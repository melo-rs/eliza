use proc_macro::TokenStream;
use proc_macro2::{Span, TokenStream as TokenStream2};
use quote::quote;
use std::collections::VecDeque;
use syn::{
    Error, Ident, LitStr, Path, Result, Token, bracketed, parenthesized,
    parse::{Parse, ParseStream},
    parse_macro_input,
    punctuated::Punctuated,
};

mod kw {
    syn::custom_keyword!(patterns);
    syn::custom_keyword!(routes);
}

struct Route {
    method: u8,
    handler: Path,
}

impl Parse for Route {
    fn parse(input: ParseStream<'_>) -> Result<Self> {
        let content;
        parenthesized!(content in input);

        let method: Ident = content.parse()?;
        content.parse::<Token![,]>()?;

        let handler = content.parse()?;

        Ok(Self {
            method: method_order(&method),
            handler,
        })
    }
}

struct Routes {
    patterns: Vec<Vec<u8>>,
    routes: Vec<Route>,
}

impl Parse for Routes {
    fn parse(input: ParseStream<'_>) -> Result<Self> {
        input.parse::<kw::patterns>()?;
        input.parse::<Token![:]>()?;

        let patterns_content;
        bracketed!(patterns_content in input);

        let patterns = Punctuated::<LitStr, Token![,]>::parse_terminated(&patterns_content)?;

        input.parse::<Token![,]>()?;

        input.parse::<kw::routes>()?;
        input.parse::<Token![:]>()?;

        let routes_content;
        bracketed!(routes_content in input);

        let routes = Punctuated::<Route, Token![,]>::parse_terminated(&routes_content)?;

        // allow the trailing comma after `routes: [...]`.
        if input.peek(Token![,]) {
            input.parse::<Token![,]>()?;
        }

        if patterns.len() != routes.len() {
            return Err(syn::Error::new(
                input.span(),
                "number of route patterns does not match number of routes",
            ));
        }

        Ok(Self {
            patterns: patterns
                .into_iter()
                .map(|path| path.value().into_bytes())
                .collect(),
            routes: routes.into_iter().collect(),
        })
    }
}

fn method_order(method: &Ident) -> u8 {
    match method.to_string().as_str() {
        "GET" => 0,
        "HEAD" => 1,
        "POST" => 2,
        "PUT" => 3,
        "DELETE" => 4,
        "CONNECT" => 5,
        "OPTIONS" => 6,
        "TRACE" => 7,
        "PATCH" => 8,
        _ => unreachable!(),
    }
}

const MAX_ROUTES: usize = u16::MAX as usize + 1;

pub(crate) fn build_router_macro(input: TokenStream) -> TokenStream {
    let routes = parse_macro_input!(input as Routes);

    if routes.patterns.len() > MAX_ROUTES {
        return Error::new(
            Span::call_site(),
            "more than u16::MAX + 1 routes are not supported",
        )
        .into_compile_error()
        .into();
    }

    let radix_tree = build_radix_tree(&routes.patterns, routes.routes);

    let RouterLayout {
        dispatch_arms,
        chunk_blob,
        vertices,
    } = build_router_layout(&radix_tree);

    let vertices = vertices.into_iter().map(|vertex| {
        let PackedVertex {
            chunk_offset,
            chunk_len,
            children_offset,
            children_len,
            handlers_offset,
            method_bitset,
        } = vertex;

        quote! {
            ::eliza::routing::internal::make_vertex(
                #chunk_offset,
                #chunk_len,
                #children_offset,
                #children_len,
                #handlers_offset,
                #method_bitset,
            )
        }
    });

    quote! {
        ::eliza::routing::internal::make_router(
            [#(#vertices),*],
            [#(#chunk_blob),*],
            |handler_id: usize| match handler_id {
                #(#dispatch_arms)*
                _ => unreachable!(),
            },
        )
    }
    .into()
}

/// `NONE` represents an invalid index.
///
/// [`usize::MAX`] is used as a [sentinel value]. Since a collection's length
/// cannot exceed [`usize::MAX`] and valid indices are always strictly less
/// than its length, [`usize::MAX`] can never be a valid index.
///
/// [sentinel value]: https://en.wikipedia.org/wiki/Sentinel_value
const NONE: usize = usize::MAX;

/// `SUPPORTED_METHOD_COUNT` is the number of standard HTTP methods.
///
/// See [RFC 9110 §9](https://datatracker.ietf.org/doc/html/rfc9110#section-9)
/// and [RFC 5789](https://datatracker.ietf.org/doc/html/rfc5789) for `PATCH`.
const SUPPORTED_METHOD_COUNT: usize = 9;

struct BuildRouteSet {
    method_bitset: u16,
    handlers: [Option<Path>; SUPPORTED_METHOD_COUNT],
}

impl BuildRouteSet {
    fn new(method: u8, handler: Path) -> Self {
        let method_bitset = 1 << method;
        let mut handlers = core::array::from_fn(|_| Default::default());

        handlers[method as usize] = Some(handler);

        Self {
            method_bitset,
            handlers,
        }
    }

    fn insert(&mut self, method: u8, handler: Path) {
        self.method_bitset |= 1 << method;
        self.handlers[method as usize] = Some(handler);
    }
}

struct BuildVertex<'a> {
    /// The byte chunk matched by this vertex, borrowed from a parsed route path.
    chunk: &'a [u8],

    /// The index of this vertex's first child or [`NONE`] if it has none.
    first_child: usize,

    /// The index of this vertex's next sibling or [`NONE`] if it has none.
    next_sibling: usize,

    route_set: Option<BuildRouteSet>,
}

impl<'a> Default for BuildVertex<'a> {
    fn default() -> Self {
        Self {
            chunk: &[],
            first_child: NONE,
            next_sibling: NONE,
            route_set: Default::default(),
        }
    }
}

/// Consumes each route's handler while borrowing its path for the tree's lifetime.
fn build_radix_tree<'a>(patterns: &'a [Vec<u8>], metadata: Vec<Route>) -> Vec<BuildVertex<'a>> {
    let mut vertices = Vec::<BuildVertex>::with_capacity(patterns.len() + 1);

    vertices.push(BuildVertex::default());

    for (pattern, Route { method, handler }) in patterns.iter().zip(metadata) {
        let pattern: &'a [u8] = pattern.as_slice();

        let mut parent_index = 0;
        let mut route_offset = 0;

        loop {
            // First we need to find whether the current vertex already has a
            // child whose prefix can match the remaining route. Radix-tree
            // siblings cannot start with the same byte, otherwise they would
            // share a common prefix and should have been represented by the
            // same child.
            //
            // Because of that invariant, comparing only the first byte of each
            // child's prefix is enough to choose the only possible child to
            // descend into.
            let wanted = pattern[route_offset];

            let mut child_index = vertices[parent_index].first_child;
            let mut previous_sibling_index = NONE;

            while child_index != NONE {
                let child_vertex = &vertices[child_index];

                if child_vertex.chunk[0] == wanted {
                    break;
                }

                previous_sibling_index = child_index;
                child_index = child_vertex.next_sibling;
            }

            // When no matching child is found, none of the existing branches
            // share a prefix with the remaining route. We can therefore create
            // a new child containing all remaining bytes instead of creating
            // one vertex per byte.
            //
            // Children are stored as a linked list during construction, so the
            // new vertex becomes either the parent's first child or the next
            // sibling of the last child visited above. Since the entire
            // remaining route is consumed, it's a terminal vertex.
            if child_index == NONE {
                let new_vertex_index = vertices.len();

                vertices.push(BuildVertex {
                    chunk: &pattern[route_offset..],
                    first_child: NONE,
                    next_sibling: NONE,
                    route_set: Some(BuildRouteSet::new(method, handler)),
                });

                if previous_sibling_index == NONE {
                    vertices[parent_index].first_child = new_vertex_index;
                } else {
                    vertices[previous_sibling_index].next_sibling = new_vertex_index;
                }

                break;
            }

            // A matching child was found, so its prefix and the remaining
            // route share at least their first byte. We now need to find the
            // length of their common prefix to determine how far we can
            // descend into the existing branch, or where it needs to be split.
            let remaining = &pattern[route_offset..];
            let child_vertex = &vertices[child_index];

            let common_prefix_len = common_prefix_len(remaining, child_vertex.chunk);

            // The entire child prefix matches the remaining route, so this
            // branch can be consumed without changing the tree.
            //
            //   remaining route:  abcdef
            //   child prefix:     abc
            //                     └─┘
            //                   full match
            //
            // We can therefore advance past `abc` and descend into the child. If no bytes
            // remain after advancing, the child itself represents the inserted route and
            // only needs to be marked terminal.
            if common_prefix_len == child_vertex.chunk.len() {
                route_offset += common_prefix_len;

                if route_offset == pattern.len() {
                    match vertices[child_index].route_set.as_mut() {
                        Some(route_set) => route_set.insert(method, handler),
                        None => {
                            vertices[child_index].route_set =
                                Some(BuildRouteSet::new(method, handler));
                        }
                    }

                    break;
                }

                parent_index = child_index;
                continue;
            }

            // The child only partially matches the remaining route, so the
            // existing branch must be split at their common prefix. A new
            // vertex is inserted in place of the child to represent the shared
            // prefix, while the existing child is shortened to contain only
            // its unmatched suffix.
            //
            // EXAMPLE: inserting `/session` when `/sessions` already exists:
            //
            //   before:                 after:
            //
            //   /sessions*              /session*
            //                             └── s*
            //
            // The new shared vertex is terminal because the inserted route
            // ends exactly at the split point. The existing child remains
            // terminal and now represents only the `s` suffix.
            //
            // When the inserted route continues past the shared prefix,
            // another child is added below it for that suffix instead.
            let old_next_sibling_index = child_vertex.next_sibling;
            let old_chunk = child_vertex.chunk;
            let shared_prefix_index = vertices.len();

            let new_route_offset = route_offset + common_prefix_len;
            let route_ends_at_split = new_route_offset == pattern.len();

            vertices.push(BuildVertex {
                chunk: &pattern[route_offset..route_offset + common_prefix_len],
                first_child: child_index,
                next_sibling: old_next_sibling_index,
                route_set: None,
            });

            vertices[child_index].chunk = &old_chunk[common_prefix_len..];

            vertices[child_index].next_sibling = NONE;

            // If the new route also has bytes remaining after the shared prefix,
            // those bytes form a second branch alongside the unmatched suffix
            // of the existing child.
            //
            // EXAMPLE: inserting `/session/revoke` when `/sessions` already
            // exists:
            //
            //   before:                 after:
            //
            //   /sessions*              /session
            //                             ├── s*
            //                             └── /revoke*
            //
            // The shared `/session` vertex is not terminal here because
            // neither registered route ends at the split point. Both suffix
            // vertices are terminal because each completes one of the two
            // routes.
            if route_ends_at_split {
                vertices[shared_prefix_index].route_set = Some(BuildRouteSet::new(method, handler));
            } else {
                let new_suffix_index = vertices.len();

                vertices.push(BuildVertex {
                    chunk: &pattern[new_route_offset..],
                    first_child: NONE,
                    next_sibling: NONE,
                    route_set: Some(BuildRouteSet::new(method, handler)),
                });

                vertices[child_index].next_sibling = new_suffix_index;
            }

            if previous_sibling_index == NONE {
                vertices[parent_index].first_child = shared_prefix_index;
            } else {
                vertices[previous_sibling_index].next_sibling = shared_prefix_index;
            }

            break;
        }
    }

    vertices
}

/// Returns the length of the common prefix of two byte slices.
const fn common_prefix_len(a: &[u8], b: &[u8]) -> usize {
    let mut index = 0;

    while index < a.len() && index < b.len() {
        if a[index] != b[index] {
            break;
        }

        index += 1;
    }

    index
}

struct PackedVertex {
    chunk_offset: u16,
    chunk_len: u16,
    children_offset: u16,
    children_len: u16,
    method_bitset: u16,
    handlers_offset: u16,
}

struct RouterLayout {
    vertices: Vec<PackedVertex>,
    chunk_blob: Vec<u8>,
    dispatch_arms: Vec<TokenStream2>,
}

fn build_router_layout(build_vertices: &[BuildVertex<'_>]) -> RouterLayout {
    let mut vertices = Vec::<PackedVertex>::with_capacity(build_vertices.len());
    let mut chunk_blob = Vec::<u8>::new();
    let mut dispatch_arms = Vec::<TokenStream2>::new();

    let mut queue = VecDeque::with_capacity(build_vertices.len());

    let mut next_children_offset = 0;
    let mut root_child_index = build_vertices[0].first_child;

    while root_child_index != NONE {
        queue.push_back(root_child_index);

        root_child_index = build_vertices[root_child_index].next_sibling;
        next_children_offset += 1;
    }

    while let Some(index) = queue.pop_front() {
        let vertex = &build_vertices[index];

        let chunk_offset = chunk_blob.len();
        let handlers_offset = dispatch_arms.len();

        chunk_blob.extend_from_slice(vertex.chunk);

        let mut child_count = 0;
        let mut child_index = vertex.first_child;

        while child_index != NONE {
            queue.push_back(child_index);

            child_count += 1;
            child_index = build_vertices[child_index].next_sibling;
        }

        vertices.push(PackedVertex {
            chunk_offset: chunk_offset as u16,
            chunk_len: vertex.chunk.len() as u16,
            children_offset: next_children_offset,
            children_len: child_count,
            method_bitset: vertex
                .route_set
                .as_ref()
                .map_or(0, |route_set| route_set.method_bitset),
            handlers_offset: handlers_offset as u16,
        });

        if let Some(route_set) = &vertex.route_set {
            let mut methods = route_set.method_bitset;

            while methods != 0 {
                let method_order = methods.trailing_zeros();
                let handler = route_set.handlers[method_order as usize].as_ref().unwrap();
                let handler_index = dispatch_arms.len();

                dispatch_arms.push(quote! {
                    #handler_index => #handler(),
                });

                methods &= methods - 1;
            }
        }

        next_children_offset += child_count;
    }

    RouterLayout {
        vertices,
        chunk_blob,
        dispatch_arms,
    }
}

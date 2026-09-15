mod route;
mod router;

use proc_macro::TokenStream;
use quote::format_ident;

/// Attribute to mark a handler as a GET route.
///
/// This and all other route attributes can only be applied to free functions:
///
/// ```rust
/// #[get("/users")]
/// fn index() -> &'static str {
///   "Hello world!"
/// }
/// ```
#[proc_macro_attribute]
pub fn get(attr: TokenStream, item: TokenStream) -> TokenStream {
    route::route_macro(attr, item, format_ident!("GET"))
}

#[proc_macro_attribute]
pub fn head(attr: TokenStream, item: TokenStream) -> TokenStream {
    route::route_macro(attr, item, format_ident!("HEAD"))
}

#[proc_macro_attribute]
pub fn post(attr: TokenStream, item: TokenStream) -> TokenStream {
    route::route_macro(attr, item, format_ident!("POST"))
}

#[proc_macro_attribute]
pub fn put(attr: TokenStream, item: TokenStream) -> TokenStream {
    route::route_macro(attr, item, format_ident!("PUT"))
}

#[proc_macro_attribute]
pub fn delete(attr: TokenStream, item: TokenStream) -> TokenStream {
    route::route_macro(attr, item, format_ident!("DELETE"))
}

#[proc_macro_attribute]
pub fn connect(attr: TokenStream, item: TokenStream) -> TokenStream {
    route::route_macro(attr, item, format_ident!("CONNECT"))
}

#[proc_macro_attribute]
pub fn options(attr: TokenStream, item: TokenStream) -> TokenStream {
    route::route_macro(attr, item, format_ident!("OPTIONS"))
}

#[proc_macro_attribute]
pub fn trace(attr: TokenStream, item: TokenStream) -> TokenStream {
    route::route_macro(attr, item, format_ident!("TRACE"))
}

#[proc_macro_attribute]
pub fn patch(attr: TokenStream, item: TokenStream) -> TokenStream {
    route::route_macro(attr, item, format_ident!("PATCH"))
}

#[proc_macro]
pub fn __build_router(input: TokenStream) -> TokenStream {
    router::build_router_macro(input)
}

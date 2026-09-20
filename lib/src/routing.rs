#[must_use]
pub struct Router<const RADIX_TREE_LEN: usize, const CHUNK_BLOB_LEN: usize, D> {
    radix_tree: [internal::Vertex; RADIX_TREE_LEN],
    chunk_blob: [u8; CHUNK_BLOB_LEN],
    dispatch: D,
}

impl<const RADIX_TREE_LEN: usize, const CHUNK_BLOB_LEN: usize, D> core::fmt::Debug
    for Router<RADIX_TREE_LEN, CHUNK_BLOB_LEN, D>
{
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("Router")
            .field("radix_tree_len", &RADIX_TREE_LEN)
            .field("chunk_blob_len", &CHUNK_BLOB_LEN)
            .finish_non_exhaustive()
    }
}

#[doc(hidden)]
pub mod internal {
    use crate::{response::Response, routing::Router};

    #[doc(hidden)]
    pub struct Vertex {
        chunk_offset: u16,
        chunk_len: u16,
        children_offset: u16,
        children_len: u16,
        handlers_offset: u16,
        method_bitset: u16,
    }

    #[doc(hidden)]
    pub const fn make_vertex(
        chunk_offset: u16,
        chunk_len: u16,
        children_offset: u16,
        children_len: u16,
        handlers_offset: u16,
        method_bitset: u16,
    ) -> Vertex {
        Vertex {
            chunk_offset,
            chunk_len,
            children_offset,
            children_len,
            handlers_offset,
            method_bitset,
        }
    }

    #[doc(hidden)]
    pub const fn make_router<const RADIX_TREE_LEN: usize, const CHUNK_BLOB_LEN: usize, D>(
        radix_tree: [Vertex; RADIX_TREE_LEN],
        chunk_blob: [u8; CHUNK_BLOB_LEN],
        dispatch: D,
    ) -> Router<RADIX_TREE_LEN, CHUNK_BLOB_LEN, D>
    where
        D: Fn(usize) -> Response,
    {
        Router {
            radix_tree,
            chunk_blob,
            dispatch,
        }
    }
}

use std::marker::PhantomData;

pub(crate) struct Bytes<'a> {
    start: *const u8,
    /// INVARIANT: `cursor >= start` and `cursor <= end`
    cursor: *const u8,
    end: *const u8,
    phantom: PhantomData<&'a [u8]>,
}

impl<'a> Bytes<'a> {
    pub(crate) fn new(slice: &'a [u8]) -> Self {
        let start = slice.as_ptr();

        // SAFETY: The memory range `start.addr()..start.add(slice.len())` is 
        // in bounds of the slice allocation.
        let end = unsafe { start.add(slice.len()) };

        Self {
            start,
            end,
            cursor: start,
            phantom: PhantomData,
        }
    }

    pub(crate) unsafe fn peek_n_unchecked<const N: usize>(&self) -> [u8; N] {
        unsafe { self.cursor.cast::<[u8; N]>().read() }
    }
}

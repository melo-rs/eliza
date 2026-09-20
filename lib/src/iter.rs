use std::marker::PhantomData;

pub(crate) struct Buffer<'a> {
    start: *const u8,
    /// INVARIANT: `cursor >= start` and `cursor <= end`
    cursor: *const u8,
    end: *const u8,
    phantom: PhantomData<&'a [u8]>,
}

impl<'a> Buffer<'a> {
    /// Constructs a new `Buffer` from the given slice.
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

    /// Returns the length of the remaining buffer.
    #[inline]
    pub(crate) fn len(&self) -> usize {
        self.end as usize - self.cursor as usize
    }

    /// Peeks at `index` ahead of cursor.
    ///
    /// # Safety
    ///
    /// Caller must ensure that `index <= self.len()` and there are at least 
    /// `N` bytes after it.
    pub(crate) unsafe fn peek_ahead_unchecked<const N: usize>(&self, index: usize) -> [u8; N] {
        // SAFETY: the caller ensures `self.cursor.add(n)` is in bounds of the
        // slice allocation.
        let ptr = unsafe { self.cursor.add(index) };

        // SAFETY: this pointer points to valid area of the buffer containing at 
        // least `N` bytes.
        return unsafe { ptr.cast::<[u8; N]>().read() };
    }

    /// Advances the buffer's cursor by `n`.
    ///
    /// # Safety
    ///
    /// Caller must ensure that `n <= self.len()`, otherwise
    /// `self.cursor.add(n)` is UB.
    pub(crate) unsafe fn advance(&mut self, n: usize) {
        // SAFETY: the caller ensures `self.cursor.add(n)` is in bounds of the
        // slice allocation.
        self.cursor = unsafe { self.cursor.add(n) };
    }

    pub(crate) unsafe fn peek_n_unchecked<const N: usize>(&self) -> [u8; N] {
        unsafe { self.cursor.cast::<[u8; N]>().read() }
    }
}

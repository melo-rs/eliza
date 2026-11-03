use core::marker::PhantomData;

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

    /// Reads at `cursor.add(index)` without bounds checking.
    ///
    /// # Safety
    ///
    /// Caller must ensure that `index < self.len()`, otherwise the resulting
    /// pointer isn't within the buffer allocation.
    pub(crate) unsafe fn peek_at_unchecked(&self, index: usize) -> u8 {
        // SAFETY: the caller ensures that `self.cursor.add(index)` is within
        // the remaining buffer.
        unsafe { self.cursor.add(index).read() }
    }

    /// Reads `N` elements at cursor without bounds checking.
    ///
    /// # Safety
    ///
    /// Caller must ensure that `N <= self.len()`.
    pub(crate) unsafe fn peek_n_unchecked<const N: usize>(&self) -> [u8; N] {
        // SAFETY: [u8; N] has the same alignment as u8 (1b). The caller
        // guarantees that at least N initialized bytes are readable from this
        // pointer.
        unsafe { self.cursor.cast::<[u8; N]>().read() }
    }

    /// Reads `N` elements from `cursor.add(index)` without bounds checking.
    ///
    /// # Safety
    ///
    /// Caller must ensure that `index <= self.len()` and
    /// `N <= self.len() - index`, otherwise the resulting pointer isn't valid for
    /// reading `N` elements.
    pub(crate) unsafe fn peek_n_at_unchecked<const N: usize>(&self, index: usize) -> [u8; N] {
        // SAFETY: the caller ensures `self.cursor.add(n)` is in bounds of the
        // slice allocation.
        let ptr = unsafe { self.cursor.add(index) };

        // SAFETY: this pointer points to valid area of the buffer containing at
        // least `N` bytes.
        unsafe { ptr.cast::<[u8; N]>().read() }
    }
}

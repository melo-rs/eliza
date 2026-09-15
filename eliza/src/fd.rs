pub use std::os::fd::{AsRawFd, FromRawFd, RawFd};

/// An owned file descriptor.
///
/// The file descriptor is closed when this value is dropped. An `OwnedFd`
/// assumes exclusive ownership of the file descriptor it contains.
///
/// This type is `#[repr(transparent)]` over `RawFd` and always contains a
/// nonnegative file descriptor.
#[repr(transparent)]
pub struct OwnedFd(RawFd);

impl FromRawFd for OwnedFd {
    /// Constructs a new `OwnedFd` from the given raw file descriptor.
    ///
    /// # Safety
    ///
    /// `fd` must be a valid, open file descriptor whose ownership is being
    /// transferred to this value. No other owner may close the descriptor, and
    /// the resource must require no cleanup other than `close(2)`.
    ///
    /// # Panics
    ///
    /// Panics if `fd` is negative.
    unsafe fn from_raw_fd(fd: RawFd) -> Self {
        assert!(fd >= 0, "fd >= 0");
        Self(fd)
    }
}

impl AsRawFd for OwnedFd {
    fn as_raw_fd(&self) -> RawFd {
        return self.0;
    }
}

impl Drop for OwnedFd {
    fn drop(&mut self) {
        let _ = crate::io::close(self.0);
    }
}

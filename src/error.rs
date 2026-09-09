use core::fmt::{self, Debug};
use crate::errno::Errno;

#[repr(transparent)]
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct Error(Errno);

impl Error {
    #[inline]
    pub const fn errno(&self) -> Errno {
        self.0
    }
}

impl fmt::Display for Error {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> fmt::Result {
        self.0.fmt(f)
    }
}

impl core::error::Error for Error {}

impl From<Errno> for Error {
    fn from(errno: Errno) -> Self {
        Self(errno)
    }
}

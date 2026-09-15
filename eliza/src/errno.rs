// TODO: add constants and auxiliar methods
#[repr(transparent)]
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct Errno(pub usize);

impl Errno {
    pub const EINTR: Self = Self(4);
}

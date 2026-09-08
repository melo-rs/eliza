// TODO: add constants and auxiliar methods
#[derive(Debug, PartialEq)]
pub struct Errno(pub usize);

impl Errno {
    pub const EINTR: Self = Self(4);
}

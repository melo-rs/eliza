#[repr(transparent)]
#[derive(PartialEq)]
pub struct Version(u8);

const HTTP_10: u8 = 0;
const HTTP_11: u8 = 1;

impl Version {
    pub const HTTP_10: Self = Self(HTTP_10);
    pub const HTTP_11: Self = Self(HTTP_11);
}

impl Default for Version {
    #[inline]
    fn default() -> Self {
        Self::HTTP_11
    }
}

impl core::fmt::Debug for Version {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> core::fmt::Result {
        let version_str = match self.0 {
            HTTP_10 => "HTTP/1.0",
            HTTP_11 => "HTTP/1.1",
            _ => unreachable!(),
        };

        f.write_str(version_str)
    }
}

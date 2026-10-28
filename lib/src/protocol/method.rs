/// An HTTP request method.
///
/// See [RFC 9110 § 9](https://datatracker.ietf.org/doc/html/rfc9110#section-9).
#[repr(transparent)]
pub struct Method(u8);

pub enum BadMethod {
    Malformed,
    Unknown,
}

impl Method {
    /// The HTTP `GET` method.
    ///
    /// See [RFC 9110 § 9.3.1](https://datatracker.ietf.org/doc/html/rfc9110#section-9.3.1).
    pub const GET: Self = Self(0);

    /// The HTTP `HEAD` method.
    ///
    /// See [RFC 9110 § 9.3.2](https://datatracker.ietf.org/doc/html/rfc9110#section-9.3.2).
    pub const HEAD: Self = Self(1);

    /// The HTTP `POST` method.
    ///
    /// See [RFC 9110 § 9.3.3](https://datatracker.ietf.org/doc/html/rfc9110#section-9.3.3).
    pub const POST: Self = Self(2);

    /// The HTTP `PUT` method.
    ///
    /// See [RFC 9110 § 9.3.4](https://datatracker.ietf.org/doc/html/rfc9110#section-9.3.4).
    pub const PUT: Self = Self(3);

    /// The HTTP `DELETE` method.
    ///
    /// See [RFC 9110 § 9.3.5](https://datatracker.ietf.org/doc/html/rfc9110#section-9.3.5).
    pub const DELETE: Self = Self(4);

    /// The HTTP `CONNECT` method.
    ///
    /// See [RFC 9110 § 9.3.6](https://datatracker.ietf.org/doc/html/rfc9110#section-9.3.6).
    pub const CONNECT: Self = Self(5);

    /// The HTTP `OPTIONS` method.
    ///
    /// See [RFC 9110 § 9.3.7](https://datatracker.ietf.org/doc/html/rfc9110#section-9.3.7).
    pub const OPTIONS: Self = Self(6);

    /// The HTTP `TRACE` method.
    ///
    /// See [RFC 9110 § 9.3.8](https://datatracker.ietf.org/doc/html/rfc9110#section-9.3.8).
    pub const TRACE: Self = Self(7);

    /// The HTTP `PATCH` method.
    ///
    /// See [RFC 5789 § 2](https://datatracker.ietf.org/doc/html/rfc5789#section-2).
    pub const PATCH: Self = Self(8);

    pub const fn bit(self) -> u16 {
        1 << self.0
    }

    pub fn from_slice(src: &[u8]) -> Result<Self, BadMethod> {
        match src {
            b"GET" => Ok(Self::GET),
            b"PUT" => Ok(Self::PUT),
            b"HEAD" => Ok(Self::HEAD),
            b"POST" => Ok(Self::POST),
            b"TRACE" => Ok(Self::TRACE),
            b"PATCH" => Ok(Self::PATCH),
            b"DELETE" => Ok(Self::DELETE),
            b"CONNECT" => Ok(Self::CONNECT),
            b"OPTIONS" => Ok(Self::OPTIONS),
            _ => {
                if is_token(src) {
                    Err(BadMethod::Unknown)
                } else {
                    Err(BadMethod::Malformed)
                }
            }
        }
    }
}

/// Returns whether `slice` is a valid [`token`](https://datatracker.ietf.org/doc/html/rfc9110#section-5.6.2)
pub(crate) fn is_token(slice: &[u8]) -> bool {
    slice.iter().all(|slice| is_tchar(slice))
}

/// Returns whether `character` is a valid [`tchar`](https://datatracker.ietf.org/doc/html/rfc9110#section-5.6.2).
pub(crate) fn is_tchar(character: &u8) -> bool {
    matches!(
        character,
        b'0'..=b'9'
            | b'A'..=b'Z'
            | b'a'..=b'z'
            | b'!'
            | b'#'
            | b'$'
            | b'%'
            | b'&'
            | b'\''
            | b'*'
            | b'+'
            | b'-'
            | b'.'
            | b'^'
            | b'_'
            | b'`'
            | b'|'
            | b'~'
    )
}
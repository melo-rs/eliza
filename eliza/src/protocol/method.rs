/// An HTTP request method.
///
/// See [RFC 9110 § 9](https://datatracker.ietf.org/doc/html/rfc9110#section-9).
#[repr(transparent)]
pub struct Method(u8);

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
}

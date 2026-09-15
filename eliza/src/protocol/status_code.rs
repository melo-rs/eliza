use core::num::NonZeroU16;

/// An HTTP response status code.
///
/// See [RFC 9110 § 15](https://datatracker.ietf.org/doc/html/rfc9110#section-15).
#[repr(transparent)]
#[derive(Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct StatusCode(NonZeroU16);

macro_rules! status_codes {
    (
        $(
            $(#[$docs:meta])*
            ($num:expr, $konst:ident),
        )+
    ) => {
        impl StatusCode {
        $(
            $(#[$docs])*
            pub const $konst: StatusCode = StatusCode(unsafe { NonZeroU16::new_unchecked($num) });
        )+
        }
    }
}

status_codes! {
    /// 100 Continue
    ///
    /// See [RFC 9110 § 15.2.1](https://datatracker.ietf.org/doc/html/rfc9110#section-15.2.1).
    (100, CONTINUE),

    /// 101 Switching Protocols
    ///
    /// See [RFC 9110 § 15.2.2](https://datatracker.ietf.org/doc/html/rfc9110#section-15.2.2).
    (101, SWITCHING_PROTOCOLS),

    /// 102 Processing
    ///
    /// See [RFC 2518 § 10.1](https://datatracker.ietf.org/doc/html/rfc2518#section-10.1).
    (102, PROCESSING),

    /// 103 Early Hints
    ///
    /// See [RFC 8297 § 2](https://datatracker.ietf.org/doc/html/rfc8297#section-2).
    (103, EARLY_HINTS),

    /// 200 OK
    ///
    /// See [RFC 9110 § 15.3.1](https://datatracker.ietf.org/doc/html/rfc9110#section-15.3.1).
    (200, OK),

    /// 201 Created
    ///
    /// See [RFC 9110 § 15.3.2](https://datatracker.ietf.org/doc/html/rfc9110#section-15.3.2).
    (201, CREATED),

    /// 202 Accepted
    ///
    /// See [RFC 9110 § 15.3.3](https://datatracker.ietf.org/doc/html/rfc9110#section-15.3.3).
    (202, ACCEPTED),

    /// 203 Non-Authoritative Information
    ///
    /// See [RFC 9110 § 15.3.4](https://datatracker.ietf.org/doc/html/rfc9110#section-15.3.4).
    (203, NON_AUTHORITATIVE_INFORMATION),

    /// 204 No Content
    ///
    /// See [RFC 9110 § 15.3.5](https://datatracker.ietf.org/doc/html/rfc9110#section-15.3.5).
    (204, NO_CONTENT),

    /// 205 Reset Content
    ///
    /// See [RFC 9110 § 15.3.6](https://datatracker.ietf.org/doc/html/rfc9110#section-15.3.6).
    (205, RESET_CONTENT),

    /// 206 Partial Content
    ///
    /// See [RFC 9110 § 15.3.7](https://datatracker.ietf.org/doc/html/rfc9110#section-15.3.7).
    (206, PARTIAL_CONTENT),

    /// 207 Multi-Status
    ///
    /// See [RFC 4918 § 11.1](https://datatracker.ietf.org/doc/html/rfc4918#section-11.1).
    (207, MULTI_STATUS),

    /// 208 Already Reported
    ///
    /// See [RFC 5842 § 7.1](https://datatracker.ietf.org/doc/html/rfc5842#section-7.1).
    (208, ALREADY_REPORTED),

    /// 226 IM Used
    ///
    /// See [RFC 3229 § 10.4.1](https://datatracker.ietf.org/doc/html/rfc3229#section-10.4.1).
    (226, IM_USED),

    /// 300 Multiple Choices
    ///
    /// See [RFC 9110 § 15.4.1](https://datatracker.ietf.org/doc/html/rfc9110#section-15.4.1).
    (300, MULTIPLE_CHOICES),

    /// 301 Moved Permanently
    ///
    /// See [RFC 9110 § 15.4.2](https://datatracker.ietf.org/doc/html/rfc9110#section-15.4.2).
    (301, MOVED_PERMANENTLY),

    /// 302 Found
    ///
    /// See [RFC 9110 § 15.4.3](https://datatracker.ietf.org/doc/html/rfc9110#section-15.4.3).
    (302, FOUND),

    /// 303 See Other
    ///
    /// See [RFC 9110 § 15.4.4](https://datatracker.ietf.org/doc/html/rfc9110#section-15.4.4).
    (303, SEE_OTHER),

    /// 304 Not Modified
    ///
    /// See [RFC 9110 § 15.4.5](https://datatracker.ietf.org/doc/html/rfc9110#section-15.4.5).
    (304, NOT_MODIFIED),

    /// 305 Use Proxy
    ///
    /// See [RFC 9110 § 15.4.6](https://datatracker.ietf.org/doc/html/rfc9110#section-15.4.6).
    (305, USE_PROXY),

    /// 307 Temporary Redirect
    ///
    /// See [RFC 9110 § 15.4.8](https://datatracker.ietf.org/doc/html/rfc9110#section-15.4.8).
    (307, TEMPORARY_REDIRECT),

    /// 308 Permanent Redirect
    ///
    /// See [RFC 9110 § 15.4.9](https://datatracker.ietf.org/doc/html/rfc9110#section-15.4.9).
    (308, PERMANENT_REDIRECT),

    /// 400 Bad Request
    ///
    /// See [RFC 9110 § 15.5.1](https://datatracker.ietf.org/doc/html/rfc9110#section-15.5.1).
    (400, BAD_REQUEST),

    /// 401 Unauthorized
    ///
    /// See [RFC 9110 § 15.5.2](https://datatracker.ietf.org/doc/html/rfc9110#section-15.5.2).
    (401, UNAUTHORIZED),

    /// 402 Payment Required
    ///
    /// See [RFC 9110 § 15.5.3](https://datatracker.ietf.org/doc/html/rfc9110#section-15.5.3).
    (402, PAYMENT_REQUIRED),

    /// 403 Forbidden
    ///
    /// See [RFC 9110 § 15.5.4](https://datatracker.ietf.org/doc/html/rfc9110#section-15.5.4).
    (403, FORBIDDEN),

    /// 404 Not Found
    ///
    /// See [RFC 9110 § 15.5.5](https://datatracker.ietf.org/doc/html/rfc9110#section-15.5.5).
    (404, NOT_FOUND),

    /// 405 Method Not Allowed
    ///
    /// See [RFC 9110 § 15.5.6](https://datatracker.ietf.org/doc/html/rfc9110#section-15.5.6).
    (405, METHOD_NOT_ALLOWED),

    /// 406 Not Acceptable
    ///
    /// See [RFC 9110 § 15.5.7](https://datatracker.ietf.org/doc/html/rfc9110#section-15.5.7).
    (406, NOT_ACCEPTABLE),

    /// 407 Proxy Authentication Required
    ///
    /// See [RFC 9110 § 15.5.8](https://datatracker.ietf.org/doc/html/rfc9110#section-15.5.8).
    (407, PROXY_AUTHENTICATION_REQUIRED),

    /// 408 Request Timeout
    ///
    /// See [RFC 9110 § 15.5.9](https://datatracker.ietf.org/doc/html/rfc9110#section-15.5.9).
    (408, REQUEST_TIMEOUT),

    /// 409 Conflict
    ///
    /// See [RFC 9110 § 15.5.10](https://datatracker.ietf.org/doc/html/rfc9110#section-15.5.10).
    (409, CONFLICT),

    /// 410 Gone
    ///
    /// See [RFC 9110 § 15.5.11](https://datatracker.ietf.org/doc/html/rfc9110#section-15.5.11).
    (410, GONE),

    /// 411 Length Required
    ///
    /// See [RFC 9110 § 15.5.12](https://datatracker.ietf.org/doc/html/rfc9110#section-15.5.12).
    (411, LENGTH_REQUIRED),

    /// 412 Precondition Failed
    ///
    /// See [RFC 9110 § 15.5.13](https://datatracker.ietf.org/doc/html/rfc9110#section-15.5.13).
    (412, PRECONDITION_FAILED),

    /// 413 Content Too Large
    ///
    /// See [RFC 9110 § 15.5.14](https://datatracker.ietf.org/doc/html/rfc9110#section-15.5.14).
    (413, CONTENT_TOO_LARGE),

    /// 414 URI Too Long
    ///
    /// See [RFC 9110 § 15.5.15](https://datatracker.ietf.org/doc/html/rfc9110#section-15.5.15).
    (414, URI_TOO_LONG),

    /// 415 Unsupported Media Type
    ///
    /// See [RFC 9110 § 15.5.16](https://datatracker.ietf.org/doc/html/rfc9110#section-15.5.16).
    (415, UNSUPPORTED_MEDIA_TYPE),

    /// 416 Range Not Satisfiable
    ///
    /// See [RFC 9110 § 15.5.17](https://datatracker.ietf.org/doc/html/rfc9110#section-15.5.17).
    (416, RANGE_NOT_SATISFIABLE),

    /// 417 Expectation Failed
    ///
    /// See [RFC 9110 § 15.5.18](https://datatracker.ietf.org/doc/html/rfc9110#section-15.5.18).
    (417, EXPECTATION_FAILED),

    /// 421 Misdirected Request
    ///
    /// See [RFC 9110 § 15.5.20](https://datatracker.ietf.org/doc/html/rfc9110#section-15.5.20).
    (421, MISDIRECTED_REQUEST),

    /// 422 Unprocessable Content
    ///
    /// See [RFC 9110 § 15.5.21](https://datatracker.ietf.org/doc/html/rfc9110#section-15.5.21).
    (422, UNPROCESSABLE_CONTENT),

    /// 423 Locked
    ///
    /// See [RFC 4918 § 11.3](https://datatracker.ietf.org/doc/html/rfc4918#section-11.3).
    (423, LOCKED),

    /// 424 Failed Dependency
    ///
    /// See [RFC 4918 § 11.4](https://datatracker.ietf.org/doc/html/rfc4918#section-11.4).
    (424, FAILED_DEPENDENCY),

    /// 425 Too Early
    ///
    /// See [RFC 8470 § 5.2](https://datatracker.ietf.org/doc/html/rfc8470#section-5.2).
    (425, TOO_EARLY),

    /// 426 Upgrade Required
    ///
    /// See [RFC 9110 § 15.5.22](https://datatracker.ietf.org/doc/html/rfc9110#section-15.5.22).
    (426, UPGRADE_REQUIRED),

    /// 428 Precondition Required
    ///
    /// See [RFC 6585 § 3](https://datatracker.ietf.org/doc/html/rfc6585#section-3).
    (428, PRECONDITION_REQUIRED),

    /// 429 Too Many Requests
    ///
    /// See [RFC 6585 § 4](https://datatracker.ietf.org/doc/html/rfc6585#section-4).
    (429, TOO_MANY_REQUESTS),

    /// 431 Request Header Fields Too Large
    ///
    /// See [RFC 6585 § 5](https://datatracker.ietf.org/doc/html/rfc6585#section-5).
    (431, REQUEST_HEADER_FIELDS_TOO_LARGE),

    /// 451 Unavailable For Legal Reasons
    ///
    /// See [RFC 7725 § 3](https://datatracker.ietf.org/doc/html/rfc7725#section-3).
    (451, UNAVAILABLE_FOR_LEGAL_REASONS),

    /// 500 Internal Server Error
    ///
    /// See [RFC 9110 § 15.6.1](https://datatracker.ietf.org/doc/html/rfc9110#section-15.6.1).
    (500, INTERNAL_SERVER_ERROR),

    /// 501 Not Implemented
    ///
    /// See [RFC 9110 § 15.6.2](https://datatracker.ietf.org/doc/html/rfc9110#section-15.6.2).
    (501, NOT_IMPLEMENTED),

    /// 502 Bad Gateway
    ///
    /// See [RFC 9110 § 15.6.3](https://datatracker.ietf.org/doc/html/rfc9110#section-15.6.3).
    (502, BAD_GATEWAY),

    /// 503 Service Unavailable
    ///
    /// See [RFC 9110 § 15.6.4](https://datatracker.ietf.org/doc/html/rfc9110#section-15.6.4).
    (503, SERVICE_UNAVAILABLE),

    /// 504 Gateway Timeout
    ///
    /// See [RFC 9110 § 15.6.5](https://datatracker.ietf.org/doc/html/rfc9110#section-15.6.5).
    (504, GATEWAY_TIMEOUT),

    /// 505 HTTP Version Not Supported
    ///
    /// See [RFC 9110 § 15.6.6](https://datatracker.ietf.org/doc/html/rfc9110#section-15.6.6).
    (505, HTTP_VERSION_NOT_SUPPORTED),

    /// 506 Variant Also Negotiates
    ///
    /// See [RFC 2295 § 8.1](https://datatracker.ietf.org/doc/html/rfc2295#section-8.1).
    (506, VARIANT_ALSO_NEGOTIATES),

    /// 507 Insufficient Storage
    ///
    /// See [RFC 4918 § 11.5](https://datatracker.ietf.org/doc/html/rfc4918#section-11.5).
    (507, INSUFFICIENT_STORAGE),

    /// 508 Loop Detected
    ///
    /// See [RFC 5842 § 7.2](https://datatracker.ietf.org/doc/html/rfc5842#section-7.2).
    (508, LOOP_DETECTED),

    /// 511 Network Authentication Required
    ///
    /// See [RFC 6585 § 6](https://datatracker.ietf.org/doc/html/rfc6585#section-6).
    (511, NETWORK_AUTHENTICATION_REQUIRED),
}

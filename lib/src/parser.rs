use crate::{
    iter::Buffer,
    protocol::{
        Version,
        method::{Method, is_tchar},
    },
};

/// The current stage of parsing an HTTP request.
#[derive(Default)]
pub(crate) enum PartialRequestState {
    #[default]
    Method,
    Path,
    Version,
    Complete,
}

/// An HTTP request in the process of being parsed.
///
/// Holds the fields parsed so far and the current parsing state.
#[derive(Default)]
pub(crate) struct PartialRequest {
    state: PartialRequestState,
    method: Option<Method>,
}

#[derive(Debug, PartialEq)]
enum ParseError {
    /// The request method does not match the HTTP `token` grammar.
    ///
    /// See [RFC 9110 § 9.1](https://datatracker.ietf.org/doc/html/rfc9110#section-9.1)
    /// and [RFC 9110 § 5.6.2](https://datatracker.ietf.org/doc/html/rfc9110#section-5.6.2).
    MalformedMethod,

    /// The request method is a valid HTTP `token` but is not supported.
    ///
    /// See [RFC 9110 § 9.1](https://datatracker.ietf.org/doc/html/rfc9110#section-9.1).
    UnknownMethod,

    /// The request version does not match the HTTP `HTTP-version` grammar.
    ///
    /// See [RFC 9112 § 2.3](https://datatracker.ietf.org/doc/html/rfc9112#section-2.3).
    MalformedVersion,

    /// The request version is a valid `HTTP-version` but it is not supported.
    ///
    /// See [RFC 9110 § 2.5](https://datatracker.ietf.org/doc/html/rfc9110#section-2.5)
    /// and [RFC 9110 § 15.6.6](https://datatracker.ietf.org/doc/html/rfc9110#section-15.6.6).
    UnknownVersion,
}

impl PartialRequest {
    pub(crate) fn is_complete(&self) -> bool {
        matches!(self.state, PartialRequestState::Complete)
    }

    pub(crate) fn parse(&mut self, _buffer: &mut Buffer) -> Result<(), ParseError> {
        todo!()
    }
}

fn parse_method(buffer: &mut Buffer) -> Result<Method, ParseError> {
    const GET_SPACE: [u8; 4] = *b"GET ";
    const PUT_SPACE: [u8; 4] = *b"PUT ";
    const POST: [u8; 4] = *b"POST";
    const HEAD: [u8; 4] = *b"HEAD";
    const PATCH_PREFIX: [u8; 4] = *b"PATC";
    const TRACE_PREFIX: [u8; 4] = *b"TRAC";
    const DELETE_PREFIX: [u8; 4] = *b"DELE";
    const CONNECT_PREFIX: [u8; 4] = *b"CONN";
    const OPTIONS_PREFIX: [u8; 4] = *b"OPTI";

    if buffer.len() < 8 {
        todo!()
    }

    // SAFETY: we ensure `buffer.len() >= 8`, so peek and advance calls within
    // this limit are all safe.
    match unsafe { buffer.peek_n_unchecked::<4>() } {
        GET_SPACE => {
            unsafe { buffer.advance(4) };
            Ok(Method::GET)
        }
        PUT_SPACE => {
            unsafe { buffer.advance(4) };
            Ok(Method::PUT)
        }
        POST if unsafe { buffer.peek_at_unchecked(4) } == b' ' => {
            unsafe { buffer.advance(5) };
            Ok(Method::POST)
        }
        HEAD if unsafe { buffer.peek_at_unchecked(4) } == b' ' => {
            unsafe { buffer.advance(5) }
            Ok(Method::HEAD)
        }
        PATCH_PREFIX if unsafe { buffer.peek_n_at_unchecked::<2>(4) == *b"H " } => {
            unsafe { buffer.advance(6) }
            Ok(Method::PATCH)
        }
        TRACE_PREFIX if unsafe { buffer.peek_n_at_unchecked::<2>(4) == *b"E " } => {
            unsafe { buffer.advance(6) };
            Ok(Method::TRACE)
        }
        DELETE_PREFIX if unsafe { buffer.peek_n_at_unchecked::<3>(4) == *b"TE " } => {
            unsafe { buffer.advance(7) };
            Ok(Method::DELETE)
        }
        CONNECT_PREFIX if unsafe { buffer.peek_n_at_unchecked::<4>(4) == *b"ECT " } => {
            unsafe { buffer.advance(8) };
            Ok(Method::CONNECT)
        }
        OPTIONS_PREFIX if unsafe { buffer.peek_n_at_unchecked::<4>(4) == *b"ONS " } => {
            unsafe { buffer.advance(8) };
            Ok(Method::OPTIONS)
        }
        _ => {
            let characters = unsafe { buffer.peek_n_unchecked::<8>() };

            for (index, character) in characters.into_iter().enumerate() {
                if character == b' ' && index != 0 {
                    return Err(ParseError::UnknownMethod);
                }

                if !is_tchar(character) {
                    return Err(ParseError::MalformedMethod);
                }
            }

            return Err(ParseError::UnknownMethod);
        }
    }
}

fn parse_version(buffer: &mut Buffer) -> Result<Version, ParseError> {
    const SUPPORTED_VERSION_LENGTH: usize = 8;

    const HTTP_10: [u8; SUPPORTED_VERSION_LENGTH] = *b"HTTP/1.0";
    const HTTP_11: [u8; SUPPORTED_VERSION_LENGTH] = *b"HTTP/1.1";

    if buffer.len() < SUPPORTED_VERSION_LENGTH {
        todo!()
    }

    // SAFETY: we ensure `buffer.len() >= SUPPORTED_VERSION_LENGTH`, so peek
    // and advance calls within this limit are all safe.
    match unsafe { buffer.peek_n_unchecked::<SUPPORTED_VERSION_LENGTH>() } {
        HTTP_10 => {
            unsafe { buffer.advance(8) }
            Ok(Version::HTTP_10)
        }
        HTTP_11 => {
            unsafe { buffer.advance(8) }
            Ok(Version::HTTP_11)
        }
        _ => {
            // SAFETY: we ensure `buffer.len >= SUPPORTED_VERSION_LENGTH`, therefore
            // at least 8 bytes are readable.
            if unsafe { buffer.peek_n_unchecked::<4>() == *b"HTTP" }
                && unsafe { buffer.peek_at_unchecked(4) } == b'/'
                && is_decimal(unsafe { buffer.peek_at_unchecked(5) })
                && unsafe { buffer.peek_at_unchecked(6) == b'.' }
                && is_decimal(unsafe { buffer.peek_at_unchecked(7) })
            {
                Err(ParseError::UnknownVersion)
            } else {
                Err(ParseError::MalformedVersion)
            }
        }
    }
}

#[inline]
fn is_decimal(character: u8) -> bool {
    character >= b'0' && character <= b'9'
}

fn parse_uri<'a>(buffer: &mut Buffer<'a>) -> Result<&'a [u8], ParseError> {
    todo!()
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::buffer;
    use rstest::rstest;

    #[rstest]
    #[case(b"GET / HTTP/1.1", Method::GET)]
    #[case(b"PUT / HTTP/1.1", Method::PUT)]
    #[case(b"POST / HTTP/1.1", Method::POST)]
    #[case(b"HEAD / HTTP/1.1", Method::HEAD)]
    #[case(b"PATCH / HTTP/1.1", Method::PATCH)]
    #[case(b"TRACE / HTTP/1.1", Method::TRACE)]
    #[case(b"DELETE / HTTP/1.1", Method::DELETE)]
    #[case(b"CONNECT / HTTP/1.1", Method::CONNECT)]
    #[case(b"OPTIONS / HTTP/1.1", Method::OPTIONS)]
    fn test_parse_method_succeeds_with_known_methods(#[case] slice: &[u8], #[case] method: Method) {
        let mut buffer = buffer!(slice);
        let result = parse_method(&mut buffer);
        assert!(result.is_ok_and(|m| m == method));
        assert!(buffer.len() == 10);
    }

    #[rstest]
    #[case(b"CREATE / HTTP/1.1")]
    #[case(b"CREATE / HTTP/1.1")]
    #[case(b"BREW / HTTP/1.1")]
    #[case(b"PROPFIND / HTTP/1.1")]
    #[case(b"delete / HTTP/1.1")]
    #[case(b"DeLETE / HTTP/1.1")]
    #[case(b"G!T / HTTP/1.1")]
    #[case(b"GET_ / HTTP/1.1")]
    #[case(b"M-SEARCH / HTTP/1.1")]
    #[case(b"GETTING / HTTP/1.1")]
    #[case(b"PUTTY / HTTP/1.1")]
    #[case(b"POSTER / HTTP/1.1")]
    #[case(b"HEADERS / HTTP/1.1")]
    #[case(b"TRACES / HTTP/1.1")]
    #[case(b"PATCHED / HTTP/1.1")]
    #[case(b"DELETE2 / HTTP/1.1")]
    #[case(b"OPTIONSX / HTTP/1.1")]
    #[case(b"CONNECTED / HTTP/1.1")]
    fn test_parse_method_fails_with_unknown_but_valid_methods(#[case] slice: &[u8]) {
        let mut buffer = buffer!(slice);
        let result = parse_method(&mut buffer);
        assert!(result.is_err_and(|err| err == ParseError::UnknownMethod));
        assert!(buffer.len() == slice.len());
    }

    #[rstest]
    #[case(b" / HTTP/1.1")]
    #[case(b"G@T / HTTP/1.1")]
    #[case(b"G/T / HTTP/1.1")]
    #[case(b"GE(T / HTTP/1.1")]
    #[case(b"POST@ / HTTP/1.1")]
    #[case(b"GET/ HTTP/1.1")]
    #[case(b"GET\t/ HTTP/1.1")]
    #[case(b"GET\r/ HTTP/1.1")]
    #[case(b"GET\n/ HTTP/1.1")]
    #[case(b"G\x00T / HTTP/1.1")]
    #[case(b"G\x7FT / HTTP/1.1")]
    #[case(b"G\xFFT / HTTP/1.1")]
    fn test_parse_method_fails_with_malformed_methods(#[case] slice: &[u8]) {
        let mut buffer = buffer!(slice);
        let result = parse_method(&mut buffer);
        assert!(result.is_err_and(|err| err == ParseError::MalformedMethod));
        assert!(buffer.len() == slice.len());
    }

    #[rstest]
    #[case(b"HTTP/1.0", Version::HTTP_10)]
    #[case(b"HTTP/1.1", Version::HTTP_11)]
    #[case(b"HTTP/1.0\r\n", Version::HTTP_10)]
    #[case(b"HTTP/1.1\r\n", Version::HTTP_11)]
    fn test_parse_version_succeeds_with_known_versions(
        #[case] slice: &[u8],
        #[case] version: Version,
    ) {
        let mut buffer = buffer!(slice);
        let result = parse_version(&mut buffer);
        assert!(result.is_ok_and(|v| v == version));
        assert!(buffer.len() == slice.len() - 8);
    }

    #[rstest]
    #[case(b"HTTP/0.9")]
    #[case(b"HTTP/1.2")]
    #[case(b"HTTP/1.9")]
    #[case(b"HTTP/2.0")]
    #[case(b"HTTP/3.0")]
    #[case(b"HTTP/9.9")]
    fn test_parse_version_fails_with_unknown_versions(#[case] slice: &[u8]) {
        let mut buffer = buffer!(slice);
        let result = parse_version(&mut buffer);
        assert!(result.is_err_and(|err| err == ParseError::UnknownVersion));
        assert!(buffer.len() == slice.len())
    }

    #[rstest]
    #[case(b"hTTP/1.1")]
    #[case(b"HTTX/1.1")]
    #[case(b"HTTP-1.1")]
    #[case(b"HTTP/1/1")]
    #[case(b"HTTP/1.X")]
    #[case(b"HTTP/X.1")]
    #[case(b"HTTP/1. ")]
    #[case(b"HTTP/1.\0")]
    #[case(b"HTTP/1.\xFF")]
    #[case(b" HTTP/1.1")]
    #[case(b"HTTP/1.X\r\nHost: crates.io\r\n")]
    fn test_parse_version_fails_with_malformed_versions(#[case] slice: &[u8]) {
        let mut buffer = buffer!(slice);
        let result = parse_version(&mut buffer);
        assert!(result.is_err_and(|err| err == ParseError::MalformedVersion));
        assert!(buffer.len() == slice.len());
    }
}

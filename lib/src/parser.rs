use crate::{
    iter::Buffer,
    protocol::method::{Method, is_tchar},
};

/// The current stage of parsing an HTTP request.
#[derive(Default)]
pub(crate) enum PartialRequestState {
    #[default]
    Method,
    Path,
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

#[derive(Debug)]
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
        POST if unsafe { buffer.peek_ahead_unchecked(4) } == *b" " => {
            unsafe { buffer.advance(5) };
            Ok(Method::POST)
        }
        HEAD if unsafe { buffer.peek_ahead_unchecked::<1>(4) } == *b" " => {
            unsafe { buffer.advance(5) }
            Ok(Method::HEAD)
        }
        PATCH_PREFIX if unsafe { buffer.peek_ahead_unchecked::<2>(4) == *b"H " } => {
            unsafe {
                buffer.advance(6);
            }
            Ok(Method::PATCH)
        }
        TRACE_PREFIX if unsafe { buffer.peek_ahead_unchecked::<2>(4) == *b"E " } => {
            unsafe { buffer.advance(6) };
            Ok(Method::TRACE)
        }
        DELETE_PREFIX if unsafe { buffer.peek_ahead_unchecked::<3>(4) == *b"TE " } => {
            unsafe { buffer.advance(7) };
            Ok(Method::DELETE)
        }
        CONNECT_PREFIX if unsafe { buffer.peek_ahead_unchecked::<4>(4) == *b"ECT " } => {
            unsafe { buffer.advance(8) };
            Ok(Method::CONNECT)
        }
        OPTIONS_PREFIX if unsafe { buffer.peek_ahead_unchecked::<4>(4) == *b"ONS " } => {
            unsafe { buffer.advance(8) };
            Ok(Method::OPTIONS)
        }
        _ => {
            let characters = unsafe { buffer.peek_n_unchecked::<8>() };

            for (index, character) in characters.iter().enumerate() {
                if character == &b' ' && index != 0 {
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

#[cfg(test)]
mod tests {
    use std::assert_matches;

    use crate::iter::Buffer;
    use crate::parser::{ParseError, parse_method};
    use crate::protocol::Method;

    macro_rules! assert_method {
        ($($method:ident),+ $(,)?) => {
            $(
                {
                    let request = concat!(stringify!($method), " / HTTP/1.1").as_bytes();
                    let mut buffer = Buffer::new(request);

                    assert_matches!(parse_method(&mut buffer), Ok(Method::$method));
                    assert_eq!(buffer.len(), b"/ HTTP/1.1".len());
                }
            )+
        };
    }

    macro_rules! assert_error {
        ($error:path; $($request:expr),+ $(,)?) => {
            $(
                {
                    let request: &[u8] = $request;
                    let mut buffer = Buffer::new(request);
                    assert_matches!(parse_method(&mut buffer), Err($error));
                    assert_eq!(buffer.len(), request.len());
                }
            )+
        };
    }

    #[test]
    fn test_parse_method_succeeds_with_known_methods() {
        assert_method!(GET, HEAD, POST, PUT, DELETE, CONNECT, OPTIONS, TRACE, PATCH);
    }

    #[test]
    fn test_parse_method_fails_with_unknown_but_valid_methods() {
        assert_error!(ParseError::UnknownMethod;
            // Unknown methods.
            b"CREATE / HTTP/1.1",
            b"BREW / HTTP/1.1",
            b"PROPFIND / HTTP/1.1",

            // Case sensitivity.
            b"get / HTTP/1.1",
            b"GeT / HTTP/1.1",

            // Valid token characters.
            b"G!T / HTTP/1.1",
            b"GET_ / HTTP/1.1",
            b"M-SEARCH / HTTP/1.1",

            // Known-method prefixes must not match.
            b"GETTING / HTTP/1.1",
            b"PUTTY / HTTP/1.1",
            b"POSTER / HTTP/1.1",
            b"HEADERS / HTTP/1.1",
            b"TRACES / HTTP/1.1",
            b"PATCHED / HTTP/1.1",
            b"DELETE2 / HTTP/1.1",
            b"OPTIONSX / HTTP/1.1",
            b"CONNECTED / HTTP/1.1",
        );
    }

    #[test]
    fn test_parse_method_fails_with_malformed_methods() {
        assert_error!(ParseError::MalformedMethod;
            // Empty method.
            b" / HTTP/1.1",

            // Invalid token characters.
            b"G@T / HTTP/1.1",
            b"G/T / HTTP/1.1",
            b"GE(T / HTTP/1.1",

            // Invalid characters following a known prefix.
            b"POST@ / HTTP/1.1",
            b"GET/ HTTP/1.1",

            // Strictly reject non-SP separators.
            b"GET\t/ HTTP/1.1",
            b"GET\r/ HTTP/1.1",
            b"GET\n/ HTTP/1.1",

            // Control and non-ASCII bytes.
            b"G\x00T / HTTP/1.1",
            b"G\x7FT / HTTP/1.1",
            b"G\xFFT / HTTP/1.1",
        );
    }

    #[test]
    fn test_parse_method_with_exactly_eight_bytes() {
        let mut buffer = Buffer::new(b"GET /abc");
        assert_matches!(parse_method(&mut buffer), Ok(Method::GET));
        assert_eq!(buffer.len(), 4);

        let mut buffer = Buffer::new(b"OPTIONS ");
        assert_matches!(parse_method(&mut buffer), Ok(Method::OPTIONS));
        assert_eq!(buffer.len(), 0);

        let mut buffer = Buffer::new(b"CONNECT ");
        assert_matches!(parse_method(&mut buffer), Ok(Method::CONNECT));
        assert_eq!(buffer.len(), 0);
    }
}

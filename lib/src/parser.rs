use crate::{iter::Bytes, protocol::method::{Method, is_token}};

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
    pub (crate) fn is_complete(&self) -> bool {
        matches!(self.state, PartialRequestState::Complete)
    }

    pub(crate) fn parse(&mut self, buffer: &mut Bytes) -> Result<(), ParseError> {
        match self.state {
            PartialRequestState::Method => {
                let method = self.parse_method(buffer)?;
                self.method = Some(method);

                Ok(())
            }
            _ => todo!(),
        }
    }

    fn parse_method(&mut self, buffer: &mut [u8]) -> Result<Method, ParseError> {
        const GET_SPACE: [u8; 4] = *b"GET ";
        const PUT_SPACE: [u8; 4] = *b"PUT ";
        const POST: [u8; 4] = *b"POST";
        const HEAD: [u8; 4] = *b"HEAD";
        const PATCH_PREFIX: [u8; 4] = *b"PATC";
        const TRACE_PREFIX: [u8; 4] = *b"TRAC";
        const DELETE_PREFIX: [u8; 4] = *b"DELE";
        const CONNECT_PREFIX: [u8; 4] = *b"CONN";
        const OPTIONS_PREFIX: [u8; 4] = *b"OPTI";

        if buffer.len() < 16 {
            todo!()
        }

        match unsafe { peak_n::<4>(buffer) } {
            GET_SPACE => {
                buffer = ;
                Ok(Method::GET)
            },
            PUT_SPACE => Ok(Method::PUT),
            POST if unsafe { peak_n::<1>(&buffer[4..]) } == *b" " => Ok(Method::POST),
            HEAD if unsafe { peak_n::<1>(&buffer[4..]) } == *b" " => Ok(Method::HEAD),
            PATCH_PREFIX if unsafe { peak_n::<2>(&buffer[4..]) == *b"H " } => Ok(Method::PATCH),
            TRACE_PREFIX if unsafe { peak_n::<2>(&buffer[4..]) == *b"E " } => Ok(Method::TRACE),
            DELETE_PREFIX if unsafe { peak_n(&buffer[3..]) == *b"ETE " } => Ok(Method::DELETE),
            CONNECT_PREFIX if unsafe { peak_n(&buffer[4..]) == *b"ECT " } => Ok(Method::CONNECT),
            OPTIONS_PREFIX if unsafe { peak_n(&buffer[4..]) == *b"ONS " } => Ok(Method::OPTIONS),
            _ => {
                let characters = unsafe { peak_n::<8>(buffer) };

                if is_token(&characters) {
                    Err(ParseError::UnknownMethod)
                } else {
                    Err(ParseError::MalformedMethod)
                }
            }
        }
    }
}

/// Returns the first `N` bytes of `buf` as an array.
///
/// # Safety
///
/// The caller must ensure `buf.len() >= N`.
unsafe fn peak_n<const N: usize>(buf: &[u8]) -> [u8; N] {
    // SAFETY: The length precondition makes the `N` bytes readable; byte arrays
    // have alignment 1, so `buf.as_ptr()` is correctly aligned.
    unsafe { buf.as_ptr().cast::<[u8; N]>().read() }
}

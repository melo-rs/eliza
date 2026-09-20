use core::arch::x86_64::{_mm_set1_epi8, _mm_cmpeq_epi8};
use std::arch::x86_64::_mm_movemask_epi8;
use crate::simd::load_16b_chunk;

pub(crate) enum RequestState {
    Method,
    Path,
    Finished,
}

pub(crate) struct Request {
    method_end: usize,
    path_start: usize,
    path_end: usize,
}

impl Request {
    pub(crate) fn parse(&mut self, buffer: &[u8], state: RequestState) {
        match state {
            RequestState::Method => parse_method(),
        }
    }

    #[target_feature(enable = "sse2")]
    fn parse_method(&mut self, buffer: &[u8]) {
        const GET_SPACE: [u8; 4] = *b"GET ";
        const PUT_SPACE: [u8; 4] = *b"PUT ";
        const POST: [u8; 4] = *b"POST";
        const HEAD: [u8; 4] = *b"HEAD";
        const CONNECT_SPACE: [u8; 8] = *b"CONNECT ";
        const OPTIONS_SPACE: [u8; 8] = *b"OPTIONS ";
        // MISSING: PATCH, TRACE & DELETE

        if buffer.len() < 16 {
            todo!()
        }

        // SAFETY: buffer is guaranteed to have at least 16 readable bytes
        let chunk = unsafe { load_16b_chunk(buffer) };

        // SAFETY: eliza is x86-64 only so this is valid.
        let delimiters = _mm_set1_epi8(b' ' as i8);

        // SAFETY: ...
        let matches = _mm_cmpeq_epi8(chunk, delimiters);

        // SAFETY: ...
        let mask = _mm_movemask_epi8(matches) as u32;

        if mask == 0 {
            todo!()
        }

        let delimiter = mask.trailing_zeros();

        match delimiter {
            3 => {
                match unsafe { self.load_4b(buffer) } {
                    GET_SPACE => todo!(),
                    PUT_SPACE => todo!(),
                    _ => todo!()
                }
            },
            4 => {
                match unsafe { self.load_4b(buffer) } {
                    POST => todo!(),
                    HEAD => todo!(),
                    _ => todo!(),
                }
            },
            7 => {
                match unsafe { self.load_8b(buffer) } {
                    CONNECT_SPACE => todo!(),
                    OPTIONS_SPACE => todo!(),
                    _ => todo!(),
                }
            },
            _ => {
                
            }
        }
    }

    #[inline]
    unsafe fn load_4b(&self, buffer: &[u8]) -> [u8; 4] {
        unsafe { buffer.as_ptr().cast::<[u8; 4]>().read_unaligned() }
    }

    #[inline]
    unsafe fn load_8b(&self, buffer: &[u8]) -> [u8; 8] {
        unsafe { buffer.as_ptr().cast::<[u8; 8]>().read_unaligned() }
    }
}

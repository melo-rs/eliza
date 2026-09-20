use core::arch::x86_64::{ _mm_loadu_si128, __m128i };

pub unsafe fn load_16b_chunk(buf: &[u8]) -> __m128i {
    let chunk = unsafe {
        _mm_loadu_si128(buf.as_ptr().cast::<__m128i>())
    };

    chunk
}

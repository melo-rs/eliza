use crate::{errno::Errno, fd::RawFd};
use core::{ffi::c_int, mem::MaybeUninit};

/// x86-64 Linux syscall number for ['write(2)'].
///
/// ['write(2)']: https://man7.org/linux/man-pages/man2/write.2.html
pub const SYS_WRITE: usize = 1;

/// x86-64 Linux syscall number for ['close(2)'].
///
/// ['close(2)']: https://man7.org/linux/man-pages/man2/close.2.html
pub const SYS_CLOSE: usize = 3;

/// x86-64 Linux syscall number for [`recvfrom(2)`]
///
/// [`recvfrom(2)`]: https://man7.org/linux/man-pages/man2/recvfrom.2.html
pub const SYS_RECV: usize = 291;

pub fn write(fd: RawFd, buffer: &mut [u8]) -> Result<usize, Errno> {
    let result = unsafe { system_call!(SYS_WRITE, fd, buffer.as_mut_ptr(), buffer.len()) };

    if result.is_negative() {
        Err(Errno(result.unsigned_abs()))
    } else {
        Ok(result as usize)
    }
}

pub fn write_all(fd: RawFd, buffer: &mut [u8]) -> Result<(), Errno> {
    let mut offset = 0usize;

    while offset < buffer.len() {
        match write(fd, &mut buffer[offset..]) {
            Err(errno) if errno == Errno::EINTR => continue,
            Err(errno) => return Err(errno),
            Ok(0) => todo!("handle write zero"),
            Ok(written) => offset += written,
        }
    }

    Ok(())
}

pub fn close(fd: RawFd) -> Result<(), Errno> {
    let result = unsafe { system_call!(SYS_CLOSE, fd) };

    if result.is_negative() {
        Err(Errno(result.unsigned_abs()))
    } else {
        Ok(())
    }
}

pub fn recv(fd: RawFd, buffer: &mut [MaybeUninit<u8>], flags: c_int) -> Result<usize, Errno> {
    let result = unsafe { system_call!(SYS_RECV, fd, buffer.as_mut_ptr(), buffer.len(), flags) };

    if result.is_negative() {
        Err(Errno(result.unsigned_abs()))
    } else {
        Ok(result as usize)
    }
}

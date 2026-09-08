use crate::errno::Errno;
use core::{ffi::c_int, mem::size_of};

const SYS_SOCKET: usize = 41;
const SYS_BIND: usize = 49;
const SYS_LISTEN: usize = 50;
const SYS_ACCEPT: usize = 43;

pub const AF_INET: c_int = 2;
pub const AF_INET6: c_int = 10;

pub const SOCK_STREAM: c_int = 1;

pub fn socket(domain: c_int, type_: c_int, protocol: c_int) -> Result<c_int, Errno> {
    // SAFETY: The arguments match the expected `socket` syscall signature,
    // and `system_call!` passes them according to the target syscall ABI.
    let result = unsafe { system_call!(SYS_SOCKET, domain, type_, protocol) };

    if result.is_negative() {
        Err(Errno(result.unsigned_abs()))
    } else {
        Ok(result as c_int)
    }
}

#[repr(C)]
struct in_addr {
    s_addr: u32,
}

#[repr(C)]
pub struct sockaddr_in {
    sin_family: u16,
    sin_port: u16,
    sin_addr: in_addr,
    sin_zero: [u8; 8],
}

impl sockaddr_in {
    pub const fn new(octets: [u8; 4], port: u16) -> Self {
        Self {
            sin_family: AF_INET as u16,
            sin_port: port.to_be(),
            sin_addr: in_addr {
                s_addr: u32::from_be_bytes(octets).to_be(),
            },
            sin_zero: [0u8; 8],
        }
    }
}

pub fn bind(sockfd: c_int, addr: &sockaddr_in) -> Result<(), Errno> {
    let result = unsafe { system_call!(SYS_BIND, sockfd, addr, size_of::<sockaddr_in>()) };

    if result.is_negative() {
        Err(Errno(result.unsigned_abs()))
    } else {
        Ok(())
    }
}

pub fn listen(sockfd: c_int, backlog: c_int) -> Result<(), Errno> {
    let result = unsafe { system_call!(SYS_LISTEN, sockfd, backlog) };

    if result.is_negative() {
        Err(Errno(result.unsigned_abs()))
    } else {
        Ok(())
    }
}

pub fn accept(sock_fd: c_int) -> Result<c_int, Errno> {
    let result = unsafe {
        system_call!(
            SYS_ACCEPT,
            sock_fd,
            core::ptr::null_mut::<sockaddr_in>(),
            core::ptr::null_mut::<c_int>()
        )
    };

    if result.is_negative() {
        Err(Errno(result.unsigned_abs()))
    } else {
        Ok(result as c_int)
    }
}

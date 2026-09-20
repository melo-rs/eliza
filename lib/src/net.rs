use crate::{
    errno::Errno,
    error::Error,
    fd::{AsRawFd, OwnedFd, RawFd},
};
use core::{ffi::c_int, mem::size_of};

const SYS_SOCKET: usize = 41;
const SYS_ACCEPT: usize = 43;
const SYS_BIND: usize = 49;
const SYS_LISTEN: usize = 50;

pub const AF_INET: c_int = 2;
pub const AF_INET6: c_int = 10;

pub const SOCK_STREAM: c_int = 1;

#[repr(C)]
struct in_addr {
    s_addr: u32,
}

#[repr(C)]
struct sockaddr_in {
    sin_family: u16,
    sin_port: u16,
    sin_addr: in_addr,
    sin_zero: [u8; 8],
}

#[repr(C)]
struct sockaddr {
    sa_family: u16,
    sa_data: [u8; 14],
}

#[repr(transparent)]
/// An IPv4 socket address.
///
/// IPv4 socket addresses consist of an IPv4 address and a 16-bit port number,
/// as stated in [IETF RFC 793].
///
/// [IETF RFC 793]: https://datatracker.ietf.org/doc/html/rfc793
pub struct SocketAddrV4(sockaddr_in);

impl SocketAddrV4 {
    /// Creates a new socket address from an IPv4 address and a port number.
    pub const fn new(addr: [u8; 4], port: u16) -> Self {
        let socket_addr = sockaddr_in {
            sin_family: AF_INET as u16,
            sin_port: port.to_be(),
            sin_addr: in_addr {
                s_addr: u32::from_be_bytes(addr).to_be(),
            },
            sin_zero: [0u8; 8],
        };

        Self(socket_addr)
    }
}

pub const INADDR_ANY: [u8; 4] = [0u8; 4];

pub struct Listener {
    socket_fd: OwnedFd,
}

impl Listener {
    pub fn bind(socket_address: SocketAddrV4) -> Result<Self, Error> {
        let raw_fd = socket(AF_INET, SOCK_STREAM, 0)?;

        // SAFETY: On success, `socket` returns a fresh file descriptor owned by the
        // caller, so transferring that ownership to `OwnedFd` is valid.
        let socket_fd = unsafe { OwnedFd::from_raw_fd(raw_fd) };

        bind(socket_fd.as_raw_fd(), &socket_address.0)?;
        listen(socket_fd.as_raw_fd(), 128)?;

        Ok(Self { socket_fd })
    }

    pub fn accept(&self) -> Result<RawFd, Error> {
        let client_fd = accept(self.socket_fd.as_raw_fd())?;
        Ok(client_fd)
    }
}

impl AsRawFd for Listener {
    fn as_raw_fd(&self) -> RawFd {
        self.socket_fd.as_raw_fd()
    }
}

fn socket(domain: c_int, ty: c_int, protocol: c_int) -> Result<RawFd, Errno> {
    // SAFETY: The arguments match the expected `socket` syscall signature,
    // and `system_call!` passes them according to the target syscall ABI.
    let result = unsafe { system_call!(SYS_SOCKET, domain, ty, protocol) };

    if result.is_negative() {
        Err(Errno(result.unsigned_abs()))
    } else {
        Ok(result as RawFd)
    }
}

fn bind(sockfd: RawFd, addr: &sockaddr_in) -> Result<(), Errno> {
    let result = unsafe { system_call!(SYS_BIND, sockfd, addr, size_of::<sockaddr_in>()) };

    if result.is_negative() {
        Err(Errno(result.unsigned_abs()))
    } else {
        Ok(())
    }
}

fn listen(sockfd: RawFd, backlog: c_int) -> Result<(), Errno> {
    let result = unsafe { system_call!(SYS_LISTEN, sockfd, backlog) };

    if result.is_negative() {
        Err(Errno(result.unsigned_abs()))
    } else {
        Ok(())
    }
}

fn accept(sockfd: RawFd) -> Result<RawFd, Errno> {
    let result = unsafe {
        system_call!(
            SYS_ACCEPT,
            sockfd,
            core::ptr::null_mut::<sockaddr>(),
            core::ptr::null_mut::<c_int>()
        )
    };

    if result.is_negative() {
        Err(Errno(result.unsigned_abs()))
    } else {
        Ok(result as RawFd)
    }
}

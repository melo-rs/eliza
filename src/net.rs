use crate::{
    errno::Errno, error::Error, fd::{AsRawFd, FromRawFd, OwnedFd, RawFd},
};
use core::{ffi::c_int, mem::size_of};

const SYS_SOCKET: usize = 41;
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
        let socket_fd = socket(AF_INET, SOCK_STREAM, 0)?;

        bind(&socket_fd, &socket_address.0)?;
        listen(&socket_fd, 128)?;

        Ok(Self { socket_fd })
    }

    pub fn accept(&self) -> Result<(), Error> {
        todo!()
    }
}

impl AsRawFd for Listener {
    fn as_raw_fd(&self) -> RawFd {
        self.socket_fd.as_raw_fd()
    }
}

fn socket(domain: c_int, ty: c_int, protocol: c_int) -> Result<OwnedFd, Errno> {
    // SAFETY: The arguments match the expected `socket` syscall signature,
    // and `system_call!` passes them according to the target syscall ABI.
    let result = unsafe { system_call!(SYS_SOCKET, domain, ty, protocol) };

    if result.is_negative() {
        Err(Errno(result.unsigned_abs()))
    } else {
        // SAFETY: On success, `socket(2)` returns a fresh file descriptor owned by the
        // caller, so transferring that ownership to `OwnedFd` is valid.
        Ok(unsafe { OwnedFd::from_raw_fd(result as c_int) })
    }
}

fn bind(sockfd: &OwnedFd, addr: &sockaddr_in) -> Result<(), Errno> {
    let result =
        unsafe { system_call!(SYS_BIND, sockfd.as_raw_fd(), addr, size_of::<sockaddr_in>()) };

    if result.is_negative() {
        Err(Errno(result.unsigned_abs()))
    } else {
        Ok(())
    }
}

fn listen(sockfd: &OwnedFd, backlog: c_int) -> Result<(), Errno> {
    let result = unsafe { system_call!(SYS_LISTEN, sockfd.as_raw_fd(), backlog) };

    if result.is_negative() {
        Err(Errno(result.unsigned_abs()))
    } else {
        Ok(())
    }
}

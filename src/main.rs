use eliza::{
    net::{INADDR_ANY, Listener, SocketAddrV4},
};

const PORT: u16 = 3000;

const _STATIC_RESPONSE: &[u8] = b"HTTP/1.1 200 OK\r\nContent-Type: text/plain\r\nContent-Length: 12\r\nConnection: close\r\n\r\nhello world!";

fn main() -> Result<(), eliza::error::Error> {
    let socket_address = SocketAddrV4::new(INADDR_ANY, 8000);
    let _listener = Listener::bind(socket_address)?;

    println!("Listening at :{PORT}");

    // TODO: handle requests from router
    loop {}
}

use serv::{
    io::{close, write_all},
    net::{AF_INET, SOCK_STREAM, accept, bind, listen, sockaddr_in, socket},
};

const PORT: u16 = 3000;

const STATIC_RESPONSE: &[u8] = b"HTTP/1.1 200 OK\r\nContent-Type: text/plain\r\nContent-Length: 12\r\nConnection: close\r\n\r\nhello world!";

fn main() {
    let socket_fd = socket(AF_INET, SOCK_STREAM, 0).unwrap();

    let socket_address = sockaddr_in::new([127, 0, 0, 1], PORT);

    bind(socket_fd, &socket_address).unwrap();
    listen(socket_fd, 0).unwrap();

    println!("Listening at :{PORT}");

    loop {
        let connection_fd = accept(socket_fd).unwrap();
        write_all(connection_fd, STATIC_RESPONSE).unwrap();
        close(connection_fd).unwrap();
    }
}

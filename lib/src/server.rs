use crate::{error::Error, io::recv, net::Listener, routing::Router};
use core::mem::MaybeUninit;

const REQUEST_BUFFER_SIZE: usize = 4096;

pub fn serve<const R: usize, const C: usize>(
    listener: Listener,
    router: &'static Router<R, C>,
) -> Result<(), Error> {
    let mut buffer = [MaybeUninit::<u8>::uninit(); REQUEST_BUFFER_SIZE];

    while let Ok(client_fd) = listener.accept() {
        let mut buffer_offset = 0usize;

        loop {
            let received = recv(client_fd, &mut buffer[buffer_offset..], 0)?;
            let initialized_part = &buffer[buffer_offset..buffer_offset + received];

            let initialized_buffer = unsafe { initialized_part.assume_init_ref() };

            buffer_offset += received
        }

        // let method = &buffer[receiver_state.method];
        // let path = &buffer[receiver_state.path];
    }

    Ok(())
}

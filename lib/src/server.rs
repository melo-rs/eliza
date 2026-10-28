use crate::{error::Error, io::recv, net::Listener, routing::Router};
use core::mem::MaybeUninit;
use core::ops::Range;

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

fn parse(state: &mut ReceiverState, cursor: &mut ReceiverCursor, slice: &[u8]) {
    for (index, character) in slice.iter().enumerate() {
        match cursor {
            ReceiverCursor::Method => {
                if *character == b' ' {
                    state.method = 0..index;
                    state.path = (index + 1)..(index + 1);

                    *cursor = ReceiverCursor::Path;
                }

                continue;
            },
            ReceiverCursor::Path => {
                if *character == b' ' {
                    state.path.end = index;
                    *cursor = ReceiverCursor::Finished;

                    break;
                }

                continue;
            },
            ReceiverCursor::Finished => break,
        }
    }
}

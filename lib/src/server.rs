use crate::{error::Error, io::recv, net::Listener, routing::Router};
use core::mem::MaybeUninit;
use core::ops::Range;
use memchr::memchr;

const REQUEST_BUFFER_SIZE: usize = 4096;

#[derive(Default)]
struct ReceiverState {
    method: Range<usize>,
    path: Range<usize>,
}

enum ReceiverCursor {
    Method,
    Path,
    Finished,
}

pub fn serve<const R: usize, const C: usize>(
    listener: Listener,
    router: &'static Router<R, C>,
) -> Result<(), Error> {
    let mut buffer = [MaybeUninit::<u8>::uninit(); REQUEST_BUFFER_SIZE];

    while let Ok(client_fd) = listener.accept() {
        let mut buffer_offset = 0usize;

        let mut method_end = 0;
        let mut target_start = 0;
        let mut target_end = 0;

        loop {
            let received = recv(client_fd, &mut buffer[buffer_offset..], 0)?;
            let initialized_part = &buffer[buffer_offset..buffer_offset + received];

            let initialized_buffer = unsafe { initialized_part.assume_init_ref() };

            while target_end == 0 {
                
            }
            let separator = memchr(b' ', initialized_buffer).unwrap();

            if method_end == 0 {
                method_end = separator;
                target_start = method_end + 1;
            }

            let slice = initialized_buffer[]

            parse(&mut receiver_state, &mut receiver_cursor, initialized_buffer);

            if matches!(receiver_cursor, ReceiverCursor::Finished) {
                break;
            }

            buffer_offset += received
        }

        let method = &buffer[receiver_state.method];
        let path = &buffer[receiver_state.path];
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

use scrap::{Capturer, Display};
use std::io::ErrorKind::WouldBlock;
use std::fs::File;
use std::thread;
use std::time::Duration;

fn make_screenshot() -> Vec<u8>{
    let one_second = Duration::new(1, 0);
    let one_frame = one_second / 60;

    let display = Display::primary().expect("Couldn't find primary display.");
    let mut capturer = Capturer::new(display).expect("Couldn't begin capture.");
    let (w, h) = (capturer.width(), capturer.height());


    let buffer = match capturer.frame() {
        Ok(buffer) => buffer,
        Err(error) => {
            panic!("{error:?}");
        }
    };
    let mut bitflipped = Vec::with_capacity(w * h * 4);
    let stride = buffer.len() / h;

    for y in 0..h {
        for x in 0..w {
            let i = stride * y + 4 * x;
            bitflipped.extend_from_slice(&[
                buffer[i + 2],
                buffer[i + 1],
                buffer[i],
                255,
            ]);
        }
    }

    return bitflipped;
    
}
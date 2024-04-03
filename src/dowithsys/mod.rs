use scrap::{Capturer, Display};
use std::io::{ErrorKind::WouldBlock, Read};
use std::thread;
use std::fs::File;
use std::time::Duration;
use image::{codecs::png::PngEncoder, ImageEncoder, Rgb, RgbImage};
use std::convert::TryInto;

pub fn make_screenshot() -> Vec<u8>{
    let one_second = Duration::new(1, 0);
    let one_frame = one_second / 60;

    let display = Display::primary().expect("Couldn't find primary display.");
    let mut capturer = Capturer::new(display).expect("Couldn't begin capture.");
    let (w, h) = (capturer.width(), capturer.height());


    loop {

        let buffer = match capturer.frame() {
            Ok(buffer) => buffer,
            Err(error) => {
                if error.kind() == WouldBlock {
                    thread::sleep(one_frame);
                    continue;
                } else {
                    panic!("Error: {}", error);
                }
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
        let data: Result<Vec<_>, _> = buffer.bytes().collect();

        let mut img = RgbImage::from_vec(w as u32, h as u32, data.unwrap()).unwrap();
        for x in 0u32..32 {
            for y in 0u32..32 {
                let r = ((x + y) % 4).try_into().unwrap();
                let g = 255 - r;
                let b = 127 - r;
                img.put_pixel(x, y, Rgb([r, g, b]));
            }
        }

        let mut cursor = std::io::Cursor::new(Vec::new());
        let encoder = PngEncoder::new(&mut cursor);
        encoder
            .write_image(&img, h as u32, w as u32, image::ExtendedColorType::Rgb8)
            .expect("msg");

        let bytes = cursor.into_inner();
        return bytes;
    }
    
}
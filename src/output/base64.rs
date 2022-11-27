use std::io::Cursor;

use image::{ImageBuffer, Rgb};
use base64;

pub fn generate_from_image(img: ImageBuffer<Rgb<u8>, Vec<u8>>) -> String {
    let mut buf = Cursor::new(vec![]);
    img.write_to(&mut buf, image::ImageOutputFormat::Png).unwrap();

    base64::encode(buf.into_inner())
}
use image::{ImageBuffer, Rgb};

pub fn save_image(img: ImageBuffer<Rgb<u8>, Vec<u8>>, filename: String) {
    img.save(filename).unwrap()
}
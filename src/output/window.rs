use image::{ImageBuffer, Rgb};
use imageproc::window;

pub fn display_image(img: ImageBuffer<Rgb<u8>, Vec<u8>>) {
    window::display_image("image", &img, 1920, 1080);
}
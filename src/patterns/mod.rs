use image::{ImageBuffer, Rgb};

pub trait Pattern {
    fn generate(&self) -> ImageBuffer<Rgb<u8>, Vec<u8>>;    
}

pub mod julia;

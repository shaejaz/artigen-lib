use image::{ImageBuffer, Rgb};
use serde::Deserialize;

pub trait Pattern {
    fn generate(&self) -> ImageBuffer<Rgb<u8>, Vec<u8>>;    
}

pub mod julia;
pub mod blocks;

pub fn from_json_str<'a, T: Deserialize<'a>>(s: &'a str) -> Result<T, serde_json::Error> {
    serde_json::from_str(s)
}

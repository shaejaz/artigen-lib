use std::{collections::HashMap};
use image::{ImageBuffer, Rgb};
use num_complex::Complex;

use super::Pattern;

#[derive(Debug)]
pub struct Julia {
    pub config: HashMap<String, String>,
}

impl Pattern for Julia {
    fn generate(&self) -> ImageBuffer<Rgb<u8>, Vec<u8>> {
        let imgx = 200;
        let imgy = 200;
    
        let scalex = 3.0 / imgx as f32;
        let scaley = 3.0 / imgy as f32;
    
        // Create a new ImgBuf with width: imgx and height: imgy
        let mut imgbuf = ImageBuffer::new(imgx, imgy);
    
        // Iterate over the coordinates and pixels of the image
        for (x, y, pixel) in imgbuf.enumerate_pixels_mut() {
            let a = (0.3 * x as f32) as u8;
            let b = (0.3 * y as f32) as u8;
            *pixel = image::Rgb([a, 0, b]);
        }
    
        // A redundant loop to demonstrate reading image data
        for x in 0..imgx {
            for y in 0..imgy {
                let cx = y as f32 * scalex - 1.5;
                let cy = x as f32 * scaley - 1.5;
    
                let c = Complex::new(-0.4, 0.6);
                let mut z = Complex::new(cx, cy);
    
                let mut i = 0;
                while i < 255 && z.norm() <= 2.0 {
                    z = z * z + c;
                    i += 1;
                }
    
                let pixel = imgbuf.get_pixel_mut(x, y);
                let image::Rgb(data) = *pixel;
                *pixel = image::Rgb([data[0], i as u8, data[2]]);
            }
        }

        imgbuf
    }
}

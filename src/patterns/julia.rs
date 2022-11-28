use image::{ImageBuffer, Rgb};
use num_complex::Complex;
use rand::{Rng, rngs::ThreadRng};

use super::Pattern;

#[derive(Debug)]
pub struct JuliaConfig {
    pub x: u32,
    pub y: u32,
}

#[derive(Debug)]
pub struct Julia {
    pub config: JuliaConfig,
}

impl Julia {
    fn generate_scales(&self, mut rng: ThreadRng) -> (f32, f32) {
        let default: f32 = rng.gen_range(2.5..5.0);
        let x = self.config.x;
        let y = self.config.y;

        if x == y {
            return (default / x as f32, default / y as f32);
        }

        let ratio: f32 = self.config.x as f32 / self.config.y as f32;

        let mut scalex = default;
        let mut scaley = default;

        if ratio > 1.0 {
            scalex = scaley * ratio;
        } else {
            scaley = scaley / ratio;
        }

        (scalex / x as f32, scaley / y as f32)
    }
}

impl Pattern for Julia {
    fn generate(&self) -> ImageBuffer<Rgb<u8>, Vec<u8>> {
        let mut rng = rand::thread_rng();
        
        let imgx = self.config.x;
        let imgy = self.config.y;
    
        let (scalex, scaley) = self.generate_scales(rng);

        let mut imgbuf = ImageBuffer::new(imgx, imgy);
    
        for (x, y, pixel) in imgbuf.enumerate_pixels_mut() {
            let cx = y as f32 * scalex - 1.5;
            let cy = x as f32 * scaley - 1.5;
    
            let c = Complex::new(0.0, 0.803);
            let mut z = Complex::new(cx, cy);
    
            let mut i = 0;
            while i < 255 && z.norm() <= 2.0 {
                z = z * z + c;
                i += 1;
            }
    
            let Rgb(data) = *pixel;
            *pixel = Rgb([data[0], i as u8, data[2]]);
        }
    
        imgbuf
    }
}

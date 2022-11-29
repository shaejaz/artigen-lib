use std::{vec, f32::consts::PI};

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

// TODO: improve random generation of interesting julia sets
impl Julia {
    fn generate_scales(&self, rng: &mut ThreadRng) -> (f32, f32) {
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

    fn generate_complex_nums(&self, size: u8, rng: &mut ThreadRng) -> Vec<Complex<f32>> {
        let mut cvec: Vec<Complex<f32>> = vec![];
        let centerx: f32 = -1.0;
        let centery: f32 = 0.0;
        let inner_rad: f32 = 0.26;
        let outer_rad: f32 = 0.255555;

        for _ in 0..size {
            let r = (rng.gen_range(0.0..1.0) * (outer_rad.powi(2) - inner_rad.powi(2)) + inner_rad.powi(2)).sqrt();
            let theta = rng.gen_range(0.0..1.0) * 2.0 * PI;

            let re: f32 = centerx + r + theta.cos();
            let im: f32 = centery + r + theta.sin();
            
            let n = Complex::new(re, im);
            cvec.push(n);
        }

        cvec
    }

    fn generate_xy_start_pos(&self, size: u8, rng: &mut ThreadRng) -> Vec<(u32, u32)> {
        let imgx = self.config.x;
        let imgy = self.config.y;
        let mut xyvec: Vec<(u32, u32)> = vec![];

        for _ in 0..size {
            xyvec.push((
                rng.gen_range(0..imgx),
                rng.gen_range(0..imgy)
            ))
        }

        xyvec
    }

    fn calculate_coordinates(&self, x: u32, y: u32, currentx: u32, currenty: u32) -> (u32, u32) {
        let mut relativex = currentx as i32 - x as i32;
        let mut relativey = currenty as i32 - y as i32;

        if relativex < 0 {
            relativex = 0;
        }
        if relativey < 0 {
            relativey = 0;
        }

        (relativex as u32, relativey as u32)
    }
}

impl Pattern for Julia {
    fn generate(&self) -> ImageBuffer<Rgb<u8>, Vec<u8>> {
        let mut rng = rand::thread_rng();
        
        let num_fractals = rng.gen_range(5..12);
        // let num_fractals = 2;
        let fractals = self.generate_complex_nums(num_fractals, &mut rng);
        let coordinates = self.generate_xy_start_pos(num_fractals, &mut rng);

        let imgx = self.config.x;
        let imgy = self.config.y;

        let (scalex, scaley) = self.generate_scales(&mut rng);

        let mut imgbuf = ImageBuffer::new(imgx, imgy);
    
        for (currentx, currenty, pixel) in imgbuf.enumerate_pixels_mut() {
            let mut i = 0;
            
            for idx in 0..num_fractals {
                let c = fractals[idx as usize];

                let (x, y) = coordinates[idx as usize];
                let (relativex, relativey) = self.calculate_coordinates(x, y, currentx, currenty);

                let cx = relativex as f32 * scalex - 1.5;
                let cy = relativey as f32 * scaley - 1.5;

                let mut z = Complex::new(cx, cy);

                while i < 255 && z.norm() <= 2.0 {
                    z = z * z + c;
                    i += 1;
                }
            }

            let Rgb(data) = *pixel;
            *pixel = Rgb([data[0], i as u8, data[2]]);
        }
    
        imgbuf
    }
}

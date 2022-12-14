use std::{vec, num::ParseIntError};

use image::{ImageBuffer, Rgb};
use num_complex::Complex;
use rand::{Rng, rngs::ThreadRng, seq::SliceRandom};
use serde::{Serialize, Deserialize};

use super::Pattern;

#[derive(Serialize, Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct JuliaConfig {
    pub x: u32,
    pub y: u32,
    pub color1: String,
    pub color2: String,
    pub color3: String,
    pub bg_color: String,
    pub min: u8,
    pub max: u8,
}

#[derive(Debug)]
pub struct Julia {
    pub config: JuliaConfig,
}

// TODO: improve random generation of interesting julia sets
impl Julia {
    fn get_julia_set_numbers(&self) -> [(f32, f32); 11] {
        [
            (-0.8, 0.156),
            (-0.7269, 0.1889),
            (-0.4, 0.6),
            (0.37, 0.1),
            (0.355, 0.355),
            (-0.2527, -0.6709),
            (0.3568, -0.07694),
            (0.3256, 0.5066),
            (-0.8884, -0.2436),
            (-0.5601, 0.4909),
            (-0.6539, 0.4128),
        ]
    }

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

    fn generate_xy_start_pos(&self, size: u8, quads: &[(u32, u32, u32, u32)], rng: &mut ThreadRng) -> Vec<(u32, u32)> {
        let mut xyvec: Vec<(u32, u32)> = vec![];

        let mut count = 0;
        for _ in 0..size {
            let (x1, y1, x2, y2) = quads[count];

            xyvec.push((
                rng.gen_range(x1..x2),
                rng.gen_range(y1..y2)
            ));

            if count >= quads.len() - 1 {
                count = 0;
            } else {
                count += 1;
            }
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

    fn get_color_rgbs(&self) -> ([Rgb<u8>; 3], Rgb<u8>) {
        let color1 = self.decode_hex(&self.config.color1);
        let color2 = self.decode_hex(&self.config.color2);
        let color3 = self.decode_hex(&self.config.color3);
        let bg_color = self.decode_hex(&self.config.bg_color);

        ([color1, color2, color3], bg_color)
    }

    fn decode_hex(&self, s: &str) -> Rgb<u8> {
        let color: Result<Vec<u8>, ParseIntError> = (0..s.len())
            .step_by(2)
            .map(|i| u8::from_str_radix(&s[i..i+2], 16))
            .collect();

        match color {
            Ok(v) => return Rgb([v[0], v[1], v[2]]),
            Err(_) => return Rgb([0, 0, 0])
        }
    }

    fn generate_quadrants(&self) -> ([(u32, u32, u32, u32); 4]) {
        let x = self.config.x;
        let y = self.config.y;

        let halfx = x / 2;
        let halfy = y / 2;

        let quad1 = (0 as u32, 0 as u32, halfx, halfy);
        let quad2 = (halfx, 0 as u32, x, halfy);
        let quad3 = (0 as u32, halfy, halfx, y);
        let quad4 = (halfx, halfy, x, y);

        [quad1, quad2, quad3, quad4]
    }
}

impl Pattern for Julia {
    fn generate(&self) -> ImageBuffer<Rgb<u8>, Vec<u8>> {
        let mut rng = rand::thread_rng();
        
        let (colors, bg_color) = self.get_color_rgbs();
        
        let quads = self.generate_quadrants();
        
        let num_fractals = rng.gen_range(self.config.min..self.config.max) as u8;
        let fractals: Vec<(f32, f32)> = self.get_julia_set_numbers()
            .choose_multiple(&mut rng, num_fractals as usize)
            .cloned().collect();

        let coordinates = self.generate_xy_start_pos(num_fractals, &quads, &mut rng);

        let imgx = self.config.x;
        let imgy = self.config.y;

        let (scalex, scaley) = self.generate_scales(&mut rng);

        let mut imgbuf = ImageBuffer::from_pixel(imgx, imgy, bg_color);

        for (currentx, currenty, pixel) in imgbuf.enumerate_pixels_mut() {
            let mut i = 0;
            
            for idx in 0..num_fractals {
                let (cr, ci) = fractals[idx as usize];

                let (x, y) = coordinates[idx as usize];
                let (relativex, relativey) = self.calculate_coordinates(x, y, currentx, currenty);

                let cx = relativex as f32 * scalex - 1.5;
                let cy = relativey as f32 * scaley - 1.5;

                let mut z = Complex::new(cx, cy);
                let c = Complex::new(cr, ci);

                while i < 255 && z.norm() <= 2.0 {
                    z = z * z + c;
                    i += 1;
                }
            }

            if i < 50 {
                *pixel = bg_color;
            } else if i < 120 {
                *pixel = colors[0];
            } else if i < 200 {
                *pixel = colors[1];
            } else {
                *pixel = colors[2];
            }

            // let Rgb(data) = *pixel;
            // *pixel = Rgb([data[0], i as u8, data[2]]);
        }
    
        imgbuf
    }
}

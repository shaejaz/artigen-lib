use std::num::ParseIntError;

use image::{ImageBuffer, Rgb};
use imageproc::{drawing::{draw_antialiased_line_segment_mut, draw_filled_rect_mut, draw_hollow_rect_mut}, pixelops::interpolate, rect::Rect};
use rand::{Rng, rngs::ThreadRng, seq::SliceRandom};
use serde::{Serialize, Deserialize};

use super::Pattern;

#[derive(Serialize, Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct BlocksConfig {
    pub x: u32,
    pub y: u32,
    pub color1: String,
    pub color2: String,
    pub color3: String,
    pub bg_color: String,
    pub block_size: u8,
    pub line_size: u8,
    pub density: f64,
}

#[derive(Debug)]
pub struct Blocks {
    pub config: BlocksConfig
}

impl Blocks {
    fn get_weighted_choice(&self, items: [char; 4], weights: [i32; 4], rng: &mut ThreadRng) -> char {
        let mut total = 0;
        for i in weights {
            total += i;
        }

        let threshold = rng.gen_range(0..total);

        total = 0;
        for i in 0..items.len() {
            total += weights[i];

            if total >= threshold {
                return items[i];
            }
        }

        items[items.len() - 1]
    }

    fn generate_rule(&self, min: u32, max: u32, branching: bool, rng: &mut ThreadRng) -> String {
        let num_symbols = rng.gen_range(min..max);

        let possible_symbols = ['F', '+', '-', 'A'];
        let symbols_weights = [15, 10, 10, 8];
        let mut rule = String::from("");

        for _i in 0..num_symbols {
            let symbol = self.get_weighted_choice(possible_symbols, symbols_weights, rng);
            rule.push(symbol);
        }

        if !branching {
            return rule;
        }

        let brackets = ['[', ']', '[', ']', '[', ']'];
        let mut bracket_positions: Vec<usize> = (0..rule.len()).collect();
        bracket_positions.shuffle(rng);
        let bs = &mut bracket_positions[0..brackets.len()];
        bs.sort();

        let mut new_rule = String::from("");
        for i in 0..bs.len() {
            let mut prev_idx = 0;
            if i != 0 {
                prev_idx = i - 1;
            }
            new_rule.push_str(&rule[prev_idx..bs[i]]);
            new_rule.push(brackets[i]);
        }

        new_rule
    }

    fn generate_lindenmayer(&self, s: String, rules: &Vec<[String; 2]>) -> String {
        let mut output = String::from("");
        let mut is_match = false;

        for i in s.chars() {
            for rule in rules {
                if i.to_string() == rule[0] {
                    output.push_str(&rule[1]);
                    is_match = true;
                    break;
                }   
            }
            if !is_match {
                output.push(i);
            }
        }

        output
    }

    fn generate_quadrants(&self) -> ([(i32, i32, i32, i32); 4]) {
        let x = self.config.x as i32;
        let y = self.config.y as i32;

        let halfx = x / 2;
        let halfy = y / 2;

        let quad1 = (0, 0, halfx, halfy);
        let quad2 = (halfx, 0, x, halfy);
        let quad3 = (0, halfy, halfx, y);
        let quad4 = (halfx, halfy, x, y);

        [quad1, quad2, quad3, quad4]
    }

    fn check_point_in_quadrants(&self, coords: (i32, i32), quads: &[(i32, i32, i32, i32)]) -> i32 {
        let (x, y) = coords;

        let result = quads.iter().position(|c| {
            let (qx1, qy1, qx2, qy2) = c;

            return x >= *qx1 && x <= *qx2 && y >= *qy1 && y <= *qy2
        });

        match result {
            Some(idx) => return idx as i32,
            None => return -1,
        }
    }

    fn get_scaled_size(&self, factor: f64) -> (u32, u32) {
        let imgx = self.config.x;
        let imgy = self.config.y;

        let base =  (imgx + imgy) as f64 * factor;
        let scale_lower = base * 0.01;
        let scale_upper = base * 0.03;

        (scale_lower as u32, scale_upper as u32)
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
}

impl Pattern for Blocks {
    fn generate(&self) -> image::ImageBuffer<image::Rgb<u8>, Vec<u8>> {
        let mut rng = rand::thread_rng();

        let imgx = self.config.x;
        let imgy = self.config.y;

        let (colors, bg_color) = self.get_color_rgbs();

        let quads = self.generate_quadrants();

        let mut imgbuf = ImageBuffer::from_pixel(imgx, imgy, bg_color);

        let mut x: i32 = imgx as i32 / 2;
        let mut y: i32 = imgy as i32 / 2;
        let (step_lower, step_upper) = self.get_scaled_size(self.config.line_size as f64);
        let step = rng.gen_range(step_lower..step_upper) as f64;
        let angle = 90.0;

        let mut axiom = String::from("+FA+");
        let num_loops: u32 = 2;
        let mut rules: Vec<[String; 2]> = vec![];

        let mut current_angle: f64 = 0.0;
        let mut saved_state: Vec<(i32, i32, f64)> = vec![];

        let (rule_lower, rule_upper) = self.get_scaled_size(self.config.density);

        for _i in 0..num_loops {
            let rule = self.generate_rule(rule_lower, rule_upper, true, &mut rng);
            rules.push([String::from("F"), rule]);
            axiom = self.generate_lindenmayer(axiom, &rules);
        }

        for i in axiom.chars() {
            match i {
                'F' => {
                    let radians = current_angle.to_radians();
                    let x1 = ((x as f64) + (step * radians.cos())) as i32; 
                    let y1 = ((y as f64) + (step * radians.sin())) as i32;
                    
                    draw_antialiased_line_segment_mut(&mut imgbuf, (x, y), (x1, y1), Rgb([0, 0, 0]), interpolate);

                    let color = match colors.choose(&mut rng) {
                        Some(r) => r,
                        None => &Rgb([255, 128, 30]),
                    };
                    
                    let (lower, upper) = self.get_scaled_size(self.config.block_size as f64);
                    let rect = Rect::at(x as i32, y as i32).of_size(rng.gen_range(lower..upper), rng.gen_range(lower..upper));
                    draw_filled_rect_mut(&mut imgbuf, rect, *color);
                    draw_hollow_rect_mut(&mut imgbuf, rect, Rgb([0, 0, 0]));

                    x = x1;
                    y = y1;
                }
                '+' => {
                    current_angle += angle;
                }
                '-' => {
                    current_angle -= angle;
                }
                '[' => {
                    saved_state.push((x, y, current_angle));
                }
                ']' => {
                    if let Some((saved_x, saved_y, saved_angle)) = saved_state.pop() {
                        x = saved_x;
                        y = saved_y;
                        current_angle = saved_angle;
                    }
                }
                'A' => {
                    let quad_idx = self.check_point_in_quadrants((x, y), &quads);

                    let (qx1, qy1, qx2, qy2) = match quad_idx {
                        0 => quads[3],
                        1 => quads[2],
                        2 => quads[1],
                        3 => quads[0],
                        _ => (0, 0, imgx as i32, imgy as i32)
                    };

                    x = rng.gen_range(qx1..qx2) as i32;
                    y = rng.gen_range(qy1..qy2) as i32;
                }
                _ => {
                }
            }
        }

        imgbuf
    }
}

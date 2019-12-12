use std::fs::File;
use std::io::BufWriter;
use std::io::{self, BufRead};
use std::path::Path;

pub fn read_lines<P>(filename: P) -> io::Result<io::Lines<io::BufReader<File>>>
where
    P: AsRef<Path>,
{
    let file = File::open(filename)?;
    Ok(io::BufReader::new(file).lines())
}

pub fn get_input(filename: String) -> Vec<String> {
    let mut input = Vec::new();
    if let Ok(lines) = read_lines(filename) {
        for line in lines {
            if let Ok(value) = line {
                input.push(value);
            }
        }
    }
    input
}

pub struct Image {
    data: Vec<u8>,
    width: usize,
    height: usize,
    scale: usize,
}

impl Image {
    pub fn new(w: usize, h: usize, s: usize) -> Image {
        Image {
            data: vec![255; 4 * w * s * h * s],
            width: w,
            height: h,
            scale: s,
        }
    }

    pub fn set_pixel(&mut self, x: usize, y: usize, color: (u8, u8, u8, u8)) {
        let real_x = self.scale * x;
        let real_y = self.scale * y;
        let real_width = self.scale * self.width;

        for i in 0..self.scale {
            for j in 0..self.scale {
                let index = 4 * ((real_y + i) * real_width + (real_x + j));
                self.data[index + 0] = color.0;
                self.data[index + 1] = color.1;
                self.data[index + 2] = color.2;
                self.data[index + 3] = color.3;
            }
        }
    }

    pub fn save_png(&self, filename: &String) {
        let path = Path::new(filename);
        let file = File::create(path).unwrap();
        let ref mut w = BufWriter::new(file);

        let mut encoder = png::Encoder::new(
            w,
            (self.width * self.scale) as u32,
            (self.height * self.scale) as u32,
        );
        encoder.set_color(png::ColorType::RGBA);
        encoder.set_depth(png::BitDepth::Eight);
        let mut writer = encoder.write_header().unwrap();
        writer.write_image_data(&self.data).unwrap();
    }
}

use std::collections::BTreeMap;
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

pub fn get_commands_from_line(line: &String) -> BTreeMap<i64, i64> {
    let command_strings: Vec<&str> = line.split(",").collect();
    let mut commands: BTreeMap<i64, i64> = BTreeMap::new();
    command_strings
        .iter()
        .filter_map(|s| s.parse::<i64>().ok())
        .enumerate()
        .for_each(|(i, c)| {
            commands.insert(i as i64, c);
        });
    commands
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
            data: vec![0; 4 * w * s * h * s],
            width: w,
            height: h,
            scale: s,
        }
    }

    pub fn clear(&mut self, color: (u8, u8, u8, u8)) {
        for w in 0..self.width {
            for h in 0..self.height {
                self.set_pixel(w, h, color);
            }
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

use std::fs::File;
use std::io::{self, BufRead};
use std::path::Path;
use std::io::BufWriter;

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

pub fn save_png(data:  &Vec<u8>, width: u32, height: u32, filename: &String) {
    let path = Path::new(filename);
    let file = File::create(path).unwrap();
    let ref mut w = BufWriter::new(file);

    let mut encoder = png::Encoder::new(w, width, height); // Width is 2 pixels and height is 1.
    encoder.set_color(png::ColorType::RGBA);
    encoder.set_depth(png::BitDepth::Eight);
    let mut writer = encoder.write_header().unwrap();

    writer.write_image_data(&data).unwrap(); // Save        
}
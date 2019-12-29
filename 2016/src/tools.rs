use std::fs::File;
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

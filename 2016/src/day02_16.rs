// use std::collections::HashSet;
use std::char;
use std::time::Instant;

use super::tools;

#[allow(dead_code)]
pub fn run() {
    println!("Day 1 of 2016");

    // let input_file = "./input/day02_16_test.txt";
    let input_file = "./input/day02_16_real.txt";

    let start1 = Instant::now();

    let numpad = [[1, 2, 3], [4, 5, 6], [7, 8, 9]];
    let mut pos = (1, 1);
    let mut keycode = String::from("");

    let mut input = Vec::new();

    if let Ok(lines) = tools::read_lines(input_file) {
        for line in lines {
            if let Ok(value) = line {
                input.push(String::from(&value));
                for c in value.chars() {
                    match c {
                        'U' => {
                            if pos.1 > 0 {
                                pos.1 -= 1;
                            }
                        }
                        'D' => {
                            if pos.1 < 2 {
                                pos.1 += 1;
                            }
                        }
                        'L' => {
                            if pos.0 > 0 {
                                pos.0 -= 1;
                            }
                        }
                        'R' => {
                            if pos.0 < 2 {
                                pos.0 += 1;
                            }
                        }
                        _ => {}
                    }
                }
                keycode.push(char::from_digit(numpad[pos.1][pos.0], 10).unwrap());
            }
        }
    }

    let after1 = Instant::now();
    println!(
        "Part 1: {}, in {:?}",
        keycode,
        after1.duration_since(start1)
    );

    let start2 = Instant::now();

    keycode = String::from("");
    pos = (0, 2);

    let keypad = [
        [' ', ' ', '1', ' ', ' '],
        [' ', '2', '3', '4', ' '],
        ['5', '6', '7', '8', '9'],
        [' ', 'A', 'B', 'C', ' '],
        [' ', ' ', 'D', ' ', ' '],
    ];
    for line in input {
        for c in line.chars() {
            match c {
                'U' => {
                    if pos.1 > 0 && keypad[pos.1 - 1][pos.0] != ' ' {
                        pos.1 -= 1;
                    }
                }
                'D' => {
                    if pos.1 < 4 && keypad[pos.1 + 1][pos.0] != ' ' {
                        pos.1 += 1;
                    }
                }
                'L' => {
                    if pos.0 > 0 && keypad[pos.1][pos.0 - 1] != ' ' {
                        pos.0 -= 1;
                    }
                }
                'R' => {
                    if pos.0 < 4 && keypad[pos.1][pos.0 + 1] != ' ' {
                        pos.0 += 1;
                    }
                }
                _ => {}
            }
        }
        keycode.push(keypad[pos.1][pos.0]);
    }

    let after2 = Instant::now();
    println!(
        "Part 2: {}, in {:?}",
        keycode,
        after2.duration_since(start2)
    );
}

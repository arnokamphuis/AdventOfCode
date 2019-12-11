use super::tools;
use std::time::Instant;
use std::collections::BTreeMap;
use std::collections::BTreeSet;

use super::intcode::IntCodeComputer;
use super::tools::Image;

const DIR_UP: (i64, i64) = (0, -1);
const DIR_DOWN: (i64, i64) = (0, 1);
const DIR_LEFT: (i64, i64) = (-1, 0);
const DIR_RIGHT: (i64, i64) = (1, 0);

struct PaintRobot {
    hull: BTreeMap<(i64, i64), i64>,
    painted: BTreeSet<(i64, i64)>,
    current_dir: (i64, i64),
    current_pos: (i64, i64),
    brain: IntCodeComputer,
}

impl PaintRobot {
    fn new(program: &BTreeMap<i64, i64>) -> PaintRobot {
        PaintRobot {
            hull: BTreeMap::new(),
            painted: BTreeSet::new(),
            current_dir: DIR_UP,
            current_pos: (0, 0),
            brain: IntCodeComputer::new(program),
        }
    }

    fn paint(&mut self, start_color: i64) {
        let mut halted = false;
        *self.hull.entry(self.current_pos).or_insert(start_color) = start_color;

        loop {
            let current_panel = if self.hull.contains_key(&self.current_pos) {
                self.hull[&self.current_pos]
            } else {
                0
            };
            
            self.brain.add_input(current_panel);
            
            if !halted {
                halted = self.brain.run();
            }

            let output1 = self.brain.get_output().unwrap();
            let output2 = self.brain.get_output().unwrap();
            self.painted.insert(self.current_pos);

            *self.hull.entry(self.current_pos).or_insert(output1) = output1;

            match output2 {
                0 => match self.current_dir {
                    DIR_UP => {
                        self.current_dir = DIR_LEFT;
                    }
                    DIR_LEFT => {
                        self.current_dir = DIR_DOWN;
                    }
                    DIR_DOWN => {
                        self.current_dir = DIR_RIGHT;
                    }
                    DIR_RIGHT => {
                        self.current_dir = DIR_UP;
                    }
                    _ => {}
                },
                1 => match self.current_dir {
                    DIR_DOWN => {
                        self.current_dir = DIR_LEFT;
                    }
                    DIR_LEFT => {
                        self.current_dir = DIR_UP;
                    }
                    DIR_UP => {
                        self.current_dir = DIR_RIGHT;
                    }
                    DIR_RIGHT => {
                        self.current_dir = DIR_DOWN;
                    }
                    _ => {}
                },
                _ => {}
            }
            self.current_pos.0 = self.current_pos.0 + self.current_dir.0;
            self.current_pos.1 = self.current_pos.1 + self.current_dir.1;
            
            if halted && !self.brain.has_output() {
                break;
            }
        }
    }

    fn print(&self) {
        let mut minx: i64 = std::i64::MAX;
        let mut miny: i64 = std::i64::MAX;
        let mut maxx: i64 = std::i64::MIN;
        let mut maxy: i64 = std::i64::MIN;
        self.painted.iter().for_each(|panel| {
            minx = std::cmp::min(minx, panel.0);
            miny = std::cmp::min(miny, panel.1);
            maxx = std::cmp::max(maxx, panel.0);
            maxy = std::cmp::max(maxy, panel.1);
        });

        let mut count_white: Vec<usize> = vec![];
        for x in minx..=maxx {
            let mut count = 0;
            for y in miny..=maxy {
                let pos = (x, y);
                if !(self.hull.contains_key(&pos) && self.hull[&pos] == 0) {
                    count += 1;
                }
            }
            count_white.push(count);
        }

        let mut pos1 = 0;
        let mut pos2 = 0;
        let mut white_indices: Vec<usize> = vec![];
        count_white.iter().enumerate().for_each(|(i,w)| {
            if *w == 0 {
                white_indices.push(i);
            }
            if i > 0 && pos2==0 {
                if pos1==0 && *w==0 {
                    pos1 = i;
                } else if *w==0 {
                    pos2 = i;
                }
            }
        });
        let width = pos2-pos1;
        let height = (maxy-miny+1) as usize;
        while white_indices[0] >= width {
            white_indices.insert(0, white_indices[0]-width);
        }

        let mut letters = vec![ vec![ vec![0usize; width]; height]; white_indices.len()-1];
        white_indices[..white_indices.len()-1].iter().enumerate().for_each(|(l, i)| {
            (i+1..i+width).into_iter().enumerate().for_each(|(x, vx)| {
                (miny..maxy+1).into_iter().enumerate().for_each(|(y,vy)| {
                    let pos = (vx as i64,vy as i64);
                    if !(self.hull.contains_key(&pos) && self.hull[&pos] == 0) {
                        letters[l][y][x] = 1;
                    }
                });
            });
        });

        let letter_width: usize = width + 1;
        let imagewidth: usize = letters.len() * letter_width;
        let imageheight: usize = height;

        let mut image: Image = Image::new(imagewidth, imageheight, 40);

        letters.iter().enumerate().for_each(|(letter_offset, letter)| {
            letter.iter().enumerate().for_each(|(y, row)| {
                row.iter().enumerate().for_each(|(x, c)| {
                    let color = (
                        if *c==1 {   0 } else { 255 },
                        if *c==1 {   0 } else { 255 },
                        if *c==1 {   0 } else { 255 },
                        255
                    );
                    image.set_pixel( x + letter_offset * letter_width , y, color );
                });
            });
        });

        image.save_png(&"output.png".to_string());
    }
}

#[allow(dead_code)]
pub fn run() {
    println!("Day 11 of 2019");

    // let input_file = "./input/day11_19_test.txt";
    let input_file = "./input/day11_19_real.txt";
    // let input_file = "./input/day11_19_esther.txt";
    let input = tools::get_input(String::from(input_file));
    let line = &input[0];
    let command_strings: Vec<&str> = line.split(",").collect();
    let mut commands: BTreeMap<i64, i64> = BTreeMap::new();
    command_strings
        .iter()
        .filter_map(|s| s.parse::<i64>().ok())
        .enumerate()
        .for_each(|(i, c)| {
            commands.insert(i as i64, c);
        });

    let start1 = Instant::now();

    let mut robot1 = PaintRobot::new(&commands);
    robot1.paint(0);

    let after1 = Instant::now();
    println!(
        "Part 1: {:?}, in {:?}",
        robot1.painted.len(),
        after1.duration_since(start1)
    );

    let start2 = Instant::now();

    let mut robot2 = PaintRobot::new(&commands);
    robot2.paint(1);

    let after2 = Instant::now();
    println!(
        "Part 2: {}, in {:?}",
        "see output image",
        after2.duration_since(start2)
    );

    robot2.print();
}

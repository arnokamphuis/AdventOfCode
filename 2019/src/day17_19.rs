use super::intcode::get_commands_from_line;
use super::intcode::IntCodeComputer;
use super::intcode::Mode;
use super::tools;
use super::tools::Image;
use std::collections::BTreeMap;
use std::time::Instant;

struct Ascii {
    computer: IntCodeComputer,
    map: BTreeMap<(i64, i64), i64>,
    finalmap: BTreeMap<(i64, i64), i64>,
}

impl Ascii {
    fn new(m: &BTreeMap<i64, i64>) -> Ascii {
        Ascii {
            computer: IntCodeComputer::new(&m,false),
            map: BTreeMap::new(),
            finalmap: BTreeMap::new(),
        }
    }

    #[allow(dead_code)]
    fn print_env(&self) {
        let mut minx = std::i64::MAX;
        let mut maxx = std::i64::MIN;
        let mut miny = std::i64::MAX;
        let mut maxy = std::i64::MIN;
        for (p, _) in &self.map {
            minx = std::cmp::min(minx, p.0);
            maxx = std::cmp::max(maxx, p.0);
            miny = std::cmp::min(miny, p.1);
            maxy = std::cmp::max(maxy, p.1);
        }
        // println!("{} {} {} {}", minx, maxx, miny, maxy);
        let width = maxx - minx + 1;
        let height = maxy - miny + 1;

        let mut img = Image::new(width as usize, height as usize, 50);
        // println!("----------------------------------------------------------------------");
        for y in miny..=maxy {
            for x in minx..=maxx {
                if self.map.contains_key(&(x, y)) {
                    if self.map[&(x, y)] == 0 {
                        img.set_pixel(x as usize, y as usize, (0, 0, 0, 255));
                    // print!(".");
                    } else if self.map[&(x, y)] == 1 {
                        img.set_pixel(x as usize, y as usize, (255, 0, 255, 255));
                    // print!("#");
                    } else if self.map[&(x, y)] == 4 {
                        img.set_pixel(x as usize, y as usize, (255, 255, 0, 255));
                    // print!("^");
                    } else if self.map[&(x, y)] == 100 {
                        img.set_pixel(x as usize, y as usize, (0, 255, 0, 255));
                    // print!("O");
                    } else {
                        // print!("r");
                    }
                }
            }
            // print!("\n");
        }
        // println!("----------------------------------------------------------------------");
        img.save_png(&String::from("ascii-scaffold.png"));
    }

    fn find_intersect(&mut self) -> i64 {
        let mut minx = std::i64::MAX;
        let mut maxx = std::i64::MIN;
        let mut miny = std::i64::MAX;
        let mut maxy = std::i64::MIN;
        for (p, _) in &self.map {
            minx = std::cmp::min(minx, p.0);
            maxx = std::cmp::max(maxx, p.0);
            miny = std::cmp::min(miny, p.1);
            maxy = std::cmp::max(maxy, p.1);
        }
        let mut sum = 0;
        for y in miny..=maxy {
            for x in minx..=maxx {
                let pos = (x, y);
                let left = (x - 1, y);
                let right = (x + 1, y);
                let up = (x, y + 1);
                let down = (x, y - 1);
                if self.map.contains_key(&left)
                    && self.map[&left] == 1
                    && self.map.contains_key(&right)
                    && self.map[&right] == 1
                    && self.map.contains_key(&up)
                    && self.map[&up] == 1
                    && self.map.contains_key(&down)
                    && self.map[&down] == 1
                    && self.map[&pos] == 1
                {
                    self.map.entry(pos).and_modify(|p| *p = 100);
                    let rel_pos = (pos.0 - minx, pos.1 - miny);
                    sum += rel_pos.0 * rel_pos.1;
                }
            }
        }
        // self.print_env();
        sum
    }

    fn init(&mut self) {
        self.computer.add_input(0);
        self.computer.run();
        let mut pos = (0, 0);
        while let Some(x) = self.computer.get_output() {
            match x {
                35 => {
                    self.map.insert(pos, 1); // scaffolding
                }
                46 => {
                    self.map.insert(pos, 0); // space
                }
                10 => {
                    pos = (-1, pos.1 + 1);
                }
                94 => {
                    self.map.insert(pos, 4); // startpos moving up
                }
                _ => {
                    panic!("WHAHHAHAHAHA");
                }
            }
            pos = (pos.0 + 1, pos.1);
        }
    }

    fn cleanup(&mut self) -> i64 {
        let mut res = 0;

        self.computer.set_mem(0, Mode::IMM, 2);

        let main_movement: Vec<i64> = vec![
            65, 44, 67, 44, 65, 44, 67, 44, 66, 44, 65, 44, 67, 44, 66, 44, 65, 44, 66, 10,
        ];
        let a: Vec<i64> = vec![82, 44, 54, 44, 76, 44, 54, 44, 76, 44, 49, 48, 10];
        let b: Vec<i64> = vec![
            82, 44, 54, 44, 76, 44, 56, 44, 76, 44, 49, 48, 44, 82, 44, 54, 10,
        ];
        let c: Vec<i64> = vec![
            76, 44, 56, 44, 76, 44, 54, 44, 76, 44, 49, 48, 44, 76, 44, 54, 10,
        ];

        let feed: Vec<i64> = vec![110, 10];

        for i in main_movement {
            self.computer.add_input(i);
        }
        for i in a {
            self.computer.add_input(i);
        }
        for i in b {
            self.computer.add_input(i);
        }
        for i in c {
            self.computer.add_input(i);
        }
        for i in feed {
            self.computer.add_input(i);
        }

        let _returncode = self.computer.run();
        let mut pos = (0, 0);
        while let Some(x) = self.computer.get_output() {
            match x {
                35 => {
                    self.finalmap.insert(pos, 1); // scaffolding
                }
                46 => {
                    self.finalmap.insert(pos, 0); // space
                }
                10 => {
                    pos = (-1, pos.1 + 1);
                }
                94 => {
                    self.finalmap.insert(pos, 4); // startpos moving up
                }
                _ => {
                    if x < 255 {
                        // println!("  unexpected value: {}", (x as u8) as char);
                    } else {
                        res = x;
                    }
                }
            }
            pos = (pos.0 + 1, pos.1);
        }

        self.map = self.finalmap.clone();
        self.print_env();
        res
    }
}

#[allow(dead_code)]
pub fn run() {
    println!("Day 17 of 2019");

    let start0 = Instant::now();

    let input_file = "./input/day17_19_real.txt";
    let input = tools::get_input(String::from(input_file));
    let commands: BTreeMap<i64, i64> = get_commands_from_line(&input[0]);
    let mut robot = Ascii::new(&commands);
    let after0 = Instant::now();
    println!("Init in {:?}", after0.duration_since(start0));

    let start1 = Instant::now();
    robot.init();
    let res1 = robot.find_intersect();

    let after1 = Instant::now();
    println!("Part 1: {}, in {:?}", res1, after1.duration_since(start1));

    let start2 = Instant::now();

    let mut robot2 = Ascii::new(&commands);
    let res2 = robot2.cleanup();

    let after2 = Instant::now();
    println!("Part 2: {}, in {:?}", res2, after2.duration_since(start2));
}

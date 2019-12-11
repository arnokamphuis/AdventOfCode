use super::tools;
use std::time::Instant;
use std::collections::BTreeMap;
use std::collections::BTreeSet;

#[derive(Clone, Hash)]
struct IntCodeComputer {
    memory: BTreeMap<i64, i64>,
    pc: i64,
    relative_base: i64,
    input: Vec<i64>,
    output: Vec<i64>,
}

#[derive(Eq, PartialEq, Clone, Copy, Hash)]
enum Mode {
    POS = 1,
    IMM,
    REL,
}

const DIR_UP: (i64, i64) = (0, -1);
const DIR_DOWN: (i64, i64) = (0, 1);
const DIR_LEFT: (i64, i64) = (-1, 0);
const DIR_RIGHT: (i64, i64) = (1, 0);

impl IntCodeComputer {
    fn new(m: &BTreeMap<i64, i64>) -> IntCodeComputer {
        IntCodeComputer {
            memory: m.clone(),
            pc: 0,
            relative_base: 0,
            input: vec![],
            output: vec![],
        }
    }

    fn get_mem_index(&self, index: i64, mode: Mode) -> i64 {
        let real_index;
        match mode {
            Mode::POS => {
                if self.memory.contains_key(&index) {
                    real_index = self.memory[&index];
                } else {
                    real_index = 0;
                }
            }
            Mode::IMM => {
                real_index = index;
            }
            Mode::REL => {
                real_index = self.get_mem_index(index, Mode::POS) + self.relative_base;
            }
        }
        real_index
    }

    fn get_mem(&self, index: i64, mode: Mode) -> i64 {
        let real_index = self.get_mem_index(index, mode);
        if self.memory.contains_key(&real_index) {
            self.memory[&real_index]
        } else {
            0
        }
    }

    fn set_mem(&mut self, index: i64, mode: Mode, value: i64) {
        let real_index = self.get_mem_index(index, mode);
        if self.memory.contains_key(&real_index) {
            if let Some(x) = self.memory.get_mut(&real_index) {
                *x = value;
            }
        } else {
            self.memory.insert(real_index, value);
        }
    }

    fn add_input(&mut self, inp: i64) {
        self.input.push(inp);
    }

    fn get_input(&mut self) -> Option<i64> {
        if self.input.len() > 0 {
            Some(self.input.remove(0))
        } else {
            None
        }
    }

    fn add_output(&mut self, out: i64) {
        self.output.push(out);
    }

    fn get_output(&mut self) -> Option<i64> {
        if self.output.len() > 0 {
            Some(self.output.remove(0))
        } else {
            None
        }
    }

    fn has_output(&self) -> bool {
        self.output.len() > 0
    }

    fn run(&mut self) -> bool {
        let mut result = false;
        loop {
            let mut instructioncode: i64 = self.get_mem(self.pc + 0, Mode::IMM);
            let opcode = instructioncode % 100;

            if opcode == 3 && self.input.len() == 0 {
                break;
            }

            instructioncode -= opcode;

            let mut modes: Vec<Mode> = vec![Mode::POS; 4];
            if instructioncode > 0 {
                // apparently we need some different operation modes
                for i in 0usize..4 {
                    let parammode = (instructioncode / (10 as i64).pow(i as u32 + 2)) % 10;
                    match parammode {
                        0 => {
                            modes[i] = Mode::POS;
                        }
                        1 => {
                            modes[i] = Mode::IMM;
                        }
                        2 => {
                            modes[i] = Mode::REL;
                        }
                        _ => {}
                    }
                }
            }

            let paramsize: i64;

            let mut pc_modified = false;
            match opcode {
                1 => {
                    paramsize = 3;
                    assert!(modes[2]!=Mode::IMM, "write mode in opcode 1 invalid");
                    self.set_mem(
                        self.pc + 3,
                        modes[2],
                        self.get_mem(self.pc + 1, modes[0]) + self.get_mem(self.pc + 2, modes[1]),
                    );
                }
                2 => {
                    paramsize = 3;
                    assert!(modes[2]!=Mode::IMM, "write mode in opcode 2 invalid");
                    self.set_mem(
                        self.pc + 3,
                        modes[2],
                        self.get_mem(self.pc + 1, modes[0]) * self.get_mem(self.pc + 2, modes[1]),
                    );
                }
                3 => {
                    paramsize = 1;
                    if let Some(input) = self.get_input() {
                        assert!(modes[0]!=Mode::IMM, "write mode in opcode 3 invalid");
                        self.set_mem(self.pc + 1, modes[0], input);
                    }
                }
                4 => {
                    paramsize = 1;
                    self.add_output(self.get_mem(self.pc + 1, modes[0]));
                }
                5 => {
                    paramsize = 2;
                    // jump if true first paramter is non-zero ip is iset to parameter 2
                    if self.get_mem(self.pc + 1, modes[0]) != 0 {
                        self.pc = self.get_mem(self.pc + 2, modes[1]);
                        pc_modified = true;
                    }
                }
                6 => {
                    paramsize = 2;
                    // jump if false first paramter is non-zero ip is iset to parameter 2
                    if self.get_mem(self.pc + 1, modes[0]) == 0 {
                        self.pc = self.get_mem(self.pc + 2, modes[1]);
                        pc_modified = true;
                    }
                }
                7 => {
                    paramsize = 3;
                    assert!(modes[2]!=Mode::IMM, "write mode in opcode 7 invalid");
                    // less than p1 < p2 => 1 in p3 else 0 in p3
                    self.set_mem(
                        self.pc + 3,
                        modes[2],
                        (self.get_mem(self.pc + 1, modes[0]) < self.get_mem(self.pc + 2, modes[1]))
                            as i64,
                    );
                }
                8 => {
                    paramsize = 3;
                    assert!(modes[2]!=Mode::IMM, "write mode in opcode 8 invalid");
                    // less than p1 == p2 => 1 in p3 else 0 in p3
                    self.set_mem(
                        self.pc + 3,
                        modes[2],
                        (self.get_mem(self.pc + 1, modes[0]) == self.get_mem(self.pc + 2, modes[1]))
                            as i64,
                    );
                }
                9 => {
                    paramsize = 1;
                    self.relative_base += self.get_mem(self.pc + 1, modes[0]);
                }
                99 => {
                    result = true;
                    break;
                }
                _ => {
                    break;
                }
            }

            if !pc_modified {
                self.pc += paramsize + 1;
            }
        }
        result
    }
}

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
        
        for l in &letters {
            for row in l {
                for c in row {
                    print!("{}", if *c == 1 { '#' } else { ' ' });
                }
                println!("");
            }
            println!("");
            println!("");
        }
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
        "see print below",
        after2.duration_since(start2)
    );

    robot2.print();
}

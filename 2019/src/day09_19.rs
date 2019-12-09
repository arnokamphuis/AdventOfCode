use super::tools;
use std::time::Instant;
// use std::collections::HashMap;
use std::collections::BTreeMap;

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
                    self.set_mem(
                        self.pc + 3,
                        modes[2],
                        self.get_mem(self.pc + 1, modes[0]) + self.get_mem(self.pc + 2, modes[1]),
                    );
                }
                2 => {
                    paramsize = 3;
                    self.set_mem(
                        self.pc + 3,
                        modes[2],
                        self.get_mem(self.pc + 1, modes[0]) * self.get_mem(self.pc + 2, modes[1]),
                    );
                }
                3 => {
                    paramsize = 1;
                    if let Some(input) = self.get_input() {
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

#[allow(dead_code)]
pub fn run() {
    println!("Day 9 of 2019");

    // let input_file = "./input/day09_19_test.txt";
    let input_file = "./input/day09_19_real.txt";
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

    let mut computer1 = IntCodeComputer::new(&commands);
    computer1.add_input(1);

    let mut res1 = vec![];
    if computer1.run() {
        while let Some(res) = computer1.get_output() {
            res1.push(res);
        }
    }

    let after1 = Instant::now();
    println!(
        "Part 1: {:?}, in {:?}",
        res1[0],
        after1.duration_since(start1)
    );

    let start2 = Instant::now();

    let mut computer2 = IntCodeComputer::new(&commands);
    computer2.add_input(2);
    computer2.run();

    let res2: i64 = computer2.get_output().unwrap();

    let after2 = Instant::now();
    println!("Part 2: {}, in {:?}", res2, after2.duration_since(start2));
}

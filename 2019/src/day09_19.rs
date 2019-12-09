use super::tools;
use std::time::Instant;
use std::collections::HashMap;

#[derive(Clone, Hash)]
struct IntCodeComputer {
    memory: HashMap<usize,i64>,
    pc: i64,
    relative_base: i64,
    input: Vec<i64>,
    output: Vec<i64>,
}

#[derive(Eq, PartialEq, Clone, Copy, Hash)]
enum Mode {
    POS = 1,
    IMM,
}

impl IntCodeComputer {
    fn new(m: &Vec<i64>) -> IntCodeComputer {
        IntCodeComputer {
            memory: m.clone(),
            pc: 0,
            input: vec![],
            output: vec![],
        }
    }

    fn get_mem_index(&self, index: i64, mode: usize) -> i64 {
        let mut real_index = index;
        match mode {
            0 => {
                if self.memory.has_key(index) {
                    real_index = self.memory[index];
                } else {
                    real_index = 0;
                }
            }
            1 => {
                real_index = index;
            }
            2 => {
                real_index = index + self.relative_base;
            }
        }
        real_index
    }

    fn get_mem(&self, index: i64, mode: usize) -> i64 {
        let real_index = self.get_mem_index(index,mode);
        if self.memory.has_key(real_index) {
            self.memory[real_index]
        } else { 0 }
    }

    fn set_mem(&self, index: i64, mode: usize, value: i64) {
        let real_index = self.get_mem_index(index, mode);
        if self.memory.has_key(real_index) {
            self.memory.get_mut(real_index) = value;
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
            // let mut prev_instr = 0;
            let mut instructioncode: i64 = self.get_mem(self.pc + 0);
            let opcode = instructioncode % 100;

            if opcode == 3 && self.input.len() == 0 {
                break;
            }

            instructioncode -= opcode;

            let mut modes: Vec<Mode> = vec![Mode::POS; 4];
            if instructioncode > 0 {
                // apparently we need some different operation modes
                let parammode1 = instructioncode % 1000;
                if parammode1 > 0 {
                    modes[0] = Mode::IMM;
                }
                instructioncode -= parammode1;

                let parammode2 = instructioncode % 10000;
                if parammode2 > 0 {
                    modes[1] = Mode::IMM;
                }
                instructioncode -= parammode2;

                let parammode3 = instructioncode % 10000;
                if parammode3 > 0 {
                    modes[2] = Mode::IMM;
                }
                instructioncode -= parammode3;

                let parammode4 = instructioncode % 100000;
                if parammode4 > 0 {
                    modes[3] = Mode::IMM;
                }
            }

            let mut paramsize: i64 = 3;

            let memaddr: Vec<usize> = if opcode == 1 || opcode == 2 || opcode == 7 || opcode == 8 {
                paramsize = 3;
                self.memory[self.pc + 1..self.pc + paramsize + 1]
                    .iter()
                    .filter_map(|s| Some(*s as usize))
                    .collect()
            } else if opcode == 3 || opcode == 4 {
                paramsize = 1;
                self.memory[self.pc + 1..self.pc + paramsize + 1]
                    .iter()
                    .filter_map(|s| Some(*s as usize))
                    .collect()
            } else if opcode == 5 || opcode == 6 {
                paramsize = 2;
                self.memory[self.pc + 1..self.pc + paramsize + 1]
                    .iter()
                    .filter_map(|s| Some(*s as usize))
                    .collect()
            } else {
                vec![]
            };
            let mut pc_modified = false;
            match opcode {
                1 => {
                    self.set_mem(self.pc+3, modes[2], 
                        self.get_mem(self.pc+1, modes[0]) +
                        self.get_mem(self.pc+2, modes[1]);
                    );
                }
                2 => {
                    let wpos = if modes[2] == Mode::POS {
                        memaddr[2]
                    } else {
                        self.pc + 3
                    };
                    self.memory[wpos] = if modes[0] == Mode::POS {
                        self.memory[memaddr[0]]
                    } else {
                        self.memory[self.pc + 1]
                    } * if modes[1] == Mode::POS {
                        self.memory[memaddr[1]]
                    } else {
                        self.memory[self.pc + 2]
                    }
                }
                3 => {
                    if let Some(input) = self.get_input() {
                        if modes[0] == Mode::POS {
                            self.memory[memaddr[0]] = input;
                        } else {
                            self.memory[self.pc + 1] = input;
                        }
                    }
                }
                4 => {
                    let outval = if modes[0] == Mode::POS {
                        self.memory[memaddr[0]]
                    } else {
                        self.memory[self.pc + 1]
                    };
                    self.add_output(outval);
                }
                5 => {
                    // jump if true first paramter is non-zero ip is iset to parameter 2
                    let val2test: i64 = if modes[0] == Mode::POS {
                        self.memory[memaddr[0]]
                    } else {
                        self.memory[self.pc + 1]
                    };
                    if val2test != 0 {
                        let rpos = if modes[1] == Mode::POS {
                            memaddr[1]
                        } else {
                            self.pc + 2
                        };
                        self.pc = self.memory[rpos as usize] as usize;
                        pc_modified = true;
                    }
                }
                6 => {
                    // jump if false first paramter is non-zero ip is iset to parameter 2
                    let val2test: i64 = if modes[0] == Mode::POS {
                        self.memory[memaddr[0]]
                    } else {
                        self.memory[self.pc + 1]
                    };
                    if val2test == 0 {
                        let rpos = if modes[1] == Mode::POS {
                            memaddr[1]
                        } else {
                            self.pc + 2
                        };
                        self.pc = self.memory[rpos as usize] as usize;
                        pc_modified = true;
                    }
                }
                7 => {
                    // less than p1 < p2 => 1 in p3 else 0 in p3
                    let val1test: i64 = if modes[0] == Mode::POS {
                        self.memory[memaddr[0]]
                    } else {
                        self.memory[self.pc + 1]
                    };
                    let val2test: i64 = if modes[1] == Mode::POS {
                        self.memory[memaddr[1]]
                    } else {
                        self.memory[self.pc + 2]
                    };
                    let wpos = if modes[2] == Mode::POS {
                        memaddr[2]
                    } else {
                        self.pc + 3
                    };
                    self.memory[wpos as usize] = (val1test < val2test) as i64;
                }
                8 => {
                    // less than p1 == p2 => 1 in p3 else 0 in p3
                    let val1test: i64 = if modes[0] == Mode::POS {
                        self.memory[memaddr[0]]
                    } else {
                        self.memory[self.pc + 1]
                    };
                    let val2test: i64 = if modes[1] == Mode::POS {
                        self.memory[memaddr[1]]
                    } else {
                        self.memory[self.pc + 2]
                    };
                    let wpos = if modes[2] == Mode::POS {
                        memaddr[2]
                    } else {
                        self.pc + 3
                    };
                    self.memory[wpos as usize] = (val1test == val2test) as i64;
                }
                99 => {
                    result = true;
                    break;
                }
                _ => {
                    break;
                }
            }
            // prev_instr = self.pc;

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
    let commands: Vec<i64> = command_strings
        .iter()
        .filter_map(|s| s.parse::<i64>().ok())
        .collect();

    let start1 = Instant::now();

    let after1 = Instant::now();
    println!(
        "Part 1: {}, in {:?}",
        0,
        after1.duration_since(start1)
    );

    let start2 = Instant::now();

    let after2 = Instant::now();
    println!(
        "Part 2: {}, in {:?}",
        0,
        after2.duration_since(start2)
    );
}

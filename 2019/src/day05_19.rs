use super::tools;
use std::time::Instant;

#[derive(Eq, PartialEq, Clone, Hash)]
struct IntCodeComputer {
    memory: Vec<i64>,
    pc: usize,
    input: i64,
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
            input: 0,
            output: vec![],
        }
    }

    fn run(&mut self) -> bool {
        let mut result = true;
        loop {
            // let mut prev_instr = 0;
            let mut instructioncode: i64 = self.memory[self.pc + 0];
            let opcode = instructioncode % 100;

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

            let mut paramsize: usize = 3;

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
                    let wpos = if modes[2] == Mode::POS {
                        memaddr[2]
                    } else {
                        self.pc + 3
                    };
                    self.memory[wpos] = if modes[0] == Mode::POS {
                        self.memory[memaddr[0]]
                    } else {
                        self.memory[self.pc + 1]
                    } + if modes[1] == Mode::POS {
                        self.memory[memaddr[1]]
                    } else {
                        self.memory[self.pc + 2]
                    }
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
                    if modes[0] == Mode::POS {
                        self.memory[memaddr[0]] = self.input;
                    } else {
                        self.memory[self.pc + 1] = self.input;
                    }
                }
                4 => {
                    let outval = if modes[0] == Mode::POS {
                        self.memory[memaddr[0]]
                    } else {
                        self.memory[self.pc + 1]
                    };

                    self.output.push(outval);

                    // let pi: Vec<i64> =  self.memory[prev_instr..prev_instr+4].iter().filter_map(|s| Some(*s as i64)).collect();
                    // if outval != 0 {
                    //     println!("Error in instruction {} -> {:?}", prev_instr, pi);
                    //     println!(" Memory: {:?}", self.memory);
                    // }
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
                    break;
                }
                _ => {
                    result = false;
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
    println!("Day 5 of 2019");

    let input_file = "./input/day05_19_real.txt";
    let input = tools::get_input(String::from(input_file));
    let line = &input[0];
    let command_strings: Vec<&str> = line.split(",").collect();
    let commands: Vec<i64> = command_strings
        .iter()
        .filter_map(|s| s.parse::<i64>().ok())
        .collect();

    let start1 = Instant::now();

    let mut computer = IntCodeComputer::new(&commands);
    (&mut computer).input = 1;
    &computer.run();
    let res1 = &computer.output.last().unwrap();

    let after1 = Instant::now();
    println!("Part 1: {}, in {:?}", res1, after1.duration_since(start1));

    let start2 = Instant::now();

    let mut computer2 = IntCodeComputer::new(&commands);
    (&mut computer2).input = 5;
    &computer2.run();
    let res2 = &computer2.output.last().unwrap();

    let after2 = Instant::now();
    println!("Part 2: {}, in {:?}", res2, after2.duration_since(start2));
}

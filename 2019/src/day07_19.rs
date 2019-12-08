use super::tools;
use permutohedron::heap_recursive;
use std::time::Instant;

#[derive(Clone, Hash)]
struct IntCodeComputer {
    memory: Vec<i64>,
    pc: usize,
    input: Vec<i64>,
    input_counter: usize,
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
            input_counter: 0,
            output: vec![],
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
            let mut instructioncode: i64 = self.memory[self.pc + 0];
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

fn run_amplifiers(phases: &Vec<i64>, commands: &Vec<i64>, part: usize) -> i64 {
    const AMPLIFIERCOUNT: usize = 5;
    let mut result = -1;
    let mut amplifiers: Vec<IntCodeComputer> =
        vec![IntCodeComputer::new(&commands); AMPLIFIERCOUNT];

    for i in 0..AMPLIFIERCOUNT {
        amplifiers[i].add_input(phases[i]);
    }

    amplifiers[0].add_input(0);

    let mut running = true;
    while running {
        let mut finished: bool = false;
        for i in 0..AMPLIFIERCOUNT {
            finished = amplifiers[i].run();

            while let Some(out0) = amplifiers[i].get_output() {
                amplifiers[(i + 1) % 5].add_input(out0);
                result = out0
            }
        }

        if finished || part == 1 {
            running = false;
        }
    }
    result
}

#[allow(dead_code)]
pub fn run() {
    println!("Day 7 of 2019");

    // let input_file = "./input/day07_19_test.txt";
    let input_file = "./input/day07_19_real.txt";
    let input = tools::get_input(String::from(input_file));
    let line = &input[0];
    let command_strings: Vec<&str> = line.split(",").collect();
    let commands: Vec<i64> = command_strings
        .iter()
        .filter_map(|s| s.parse::<i64>().ok())
        .collect();

    let start1 = Instant::now();

    let mut part = 1;
    let mut data1 = [0, 1, 2, 3, 4];
    let mut permutations1 = Vec::new();
    heap_recursive(&mut data1, |permutation| {
        permutations1.push(permutation.to_vec())
    });

    let mut highest_signal: i64 = std::i64::MIN;
    permutations1.iter().for_each(|p| {
        let output = run_amplifiers(p, &commands, part);
        if output > highest_signal {
            highest_signal = output;
        }
    });

    let after1 = Instant::now();
    println!(
        "Part 1: {}, in {:?}",
        highest_signal,
        after1.duration_since(start1)
    );

    let start2 = Instant::now();

    part = 2;
    let mut data2 = [5, 6, 7, 8, 9];
    let mut permutations2 = Vec::new();
    heap_recursive(&mut data2, |permutation| {
        permutations2.push(permutation.to_vec())
    });

    highest_signal = std::i64::MIN;
    permutations2.iter().for_each(|p| {
        let output = run_amplifiers(p, &commands, part);
        if output > highest_signal {
            highest_signal = output;
        }
    });

    let after2 = Instant::now();
    println!(
        "Part 2: {}, in {:?}",
        highest_signal,
        after2.duration_since(start2)
    );
}

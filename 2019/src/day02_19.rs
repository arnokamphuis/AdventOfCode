use std::time::{Instant};
use super::tools;

struct IntCodeComputer {
    memory: Vec<u64>,
    pc: usize,
}

impl IntCodeComputer {

    fn new(m: &Vec<u64>) -> IntCodeComputer {
        IntCodeComputer {
            memory: m.clone(),
            pc : 0
        }
    }

    fn set_initialstate(&mut self, noun: u64, verb: u64) {
        self.memory[1] = noun;
        self.memory[2] = verb;
    }

    fn run(&mut self) -> bool{
        let mut result = true;
        loop {
            let opcode = self.memory[self.pc+0];
            const PARAMSIZE: usize = 3;

            
            let memaddr: Vec<usize> = if opcode == 1 || opcode == 2 
                { self.memory[self.pc+1..self.pc+PARAMSIZE+1].iter().filter_map(|s| Some(*s as usize)).collect() } 
            else 
                { vec![] } ;

            match opcode {
                1 => { self.memory[memaddr[2]] = self.memory[memaddr[0]] + self.memory[memaddr[1]]; }
                2 => { self.memory[memaddr[2]] = self.memory[memaddr[0]] * self.memory[memaddr[1]]; }
                99 => { break; }
                _ => { result = false; break; }
            }
            self.pc += 4;
        }
        result
    }
}

#[allow(dead_code)]
pub fn run() {
    println!("Day 2 of 2019");

    // let input_file = "./input/day02_19_test.txt";
    let input_file = "./input/day02_19_real.txt";
    let input = tools::get_input(String::from(input_file));
    let line = &input[0];
    let command_strings: Vec<&str> = line.split(",").collect();
    let commands: Vec::<u64> = command_strings.iter().filter_map(|s| s.parse::<u64>().ok()).collect();

    let mut computer = IntCodeComputer::new(&commands);
    computer.set_initialstate(12,2);
    &computer.run();


    let start1 = Instant::now();

    let after1 = Instant::now();
    println!("Part 1: {}, in {:?}", &computer.memory[0], after1.duration_since(start1));

    let start2 = Instant::now();

    let mut res_verb = 0;
    let mut res_noun = 0;
    for verb in 0..100 {
        for noun in 0..100 {
            let mut computer = IntCodeComputer::new(&commands);
            computer.set_initialstate(noun,verb);
            &computer.run();
            if (&computer).memory[0]==19690720 {
                res_noun = noun;
                res_verb = verb;
                break;
            }
        }
    }

    let after2 = Instant::now();
    println!("Part 2: {}, in {:?}", 100 * res_noun + res_verb, after2.duration_since(start2));
}

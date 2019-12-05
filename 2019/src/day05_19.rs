use std::time::{Instant};
use super::tools;

struct IntCodeComputer {
    memory: Vec<i64>,
    pc: usize,
}

impl IntCodeComputer {

    fn new(m: &Vec<i64>) -> IntCodeComputer {
        IntCodeComputer {
            memory: m.clone(),
            pc : 0
        }
    }

    fn run(&mut self) -> bool{
        let mut result = true;
        loop {
            let opcode = self.memory[self.pc+0];
            let mut paramsize: usize = 3;

            
            let memaddr: Vec<usize> = if opcode == 1 || opcode == 2 
                { self.memory[self.pc+1..self.pc+paramsize+1].iter().filter_map(|s| Some(*s as usize)).collect() } 
            else 
                { vec![] } ;

            match opcode {
                1 => { self.memory[memaddr[2]] = self.memory[memaddr[0]] + self.memory[memaddr[1]]; }
                2 => { self.memory[memaddr[2]] = self.memory[memaddr[0]] * self.memory[memaddr[1]]; }
                99 => { break; }
                _ => { result = false; break; }
            }
            self.pc += paramsize + 1;
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
    let commands: Vec::<i64> = command_strings.iter().filter_map(|s| s.parse::<i64>().ok()).collect();

    let start1 = Instant::now();

    let mut computer = IntCodeComputer::new(&commands);
    &computer.run();

    let after1 = Instant::now();
    println!("Part 1: {}, in {:?}", 0, after1.duration_since(start1));

    let start2 = Instant::now();

    let after2 = Instant::now();
    println!("Part 2: {}, in {:?}", 0, after2.duration_since(start2));
}

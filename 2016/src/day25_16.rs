use super::tools;
use std::time::Instant;

struct Computer {
    reg_a: i64,
    reg_b: i64,
    reg_c: i64,
    reg_d: i64,
    program: Vec<String>,
    pc: usize,
    out: Vec<i64>,
}

impl Computer {
    fn new(a:i64, b:i64, c:i64, d:i64) -> Computer {
        Computer { reg_a: a, reg_b: b, reg_c: c, reg_d: d, program: vec![], pc: 0, out: vec![]}
    }

    fn add_line(&mut self, loc: &String) {
        self.program.push(loc.to_string());
    }

    fn get_value(&self, s: &str) -> i64 {
        match s.parse::<i64>() {
            Ok(n) => { n },
            Err(_e) => {
                match s.as_ref() {
                    "a" => { self.reg_a }
                    "b" => { self.reg_b }
                    "c" => { self.reg_c }
                    "d" => { self.reg_d }
                    _ => { 0 }
                }

            },
        }        
    }
    
    #[allow(dead_code)]
    fn print_regs(&self) {
        println!("{} {} {} {}", self.reg_a, self.reg_b, self.reg_c, self.reg_d);
    }

    fn num_params(&self, instruction: &String) -> usize {
        let tokens = instruction.split_whitespace().collect::<Vec<&str>>();
        tokens.len() - 1
    }

    fn toggle(&mut self, counter: usize, param_count: usize) {
        if counter < self.program.len() {
            match param_count {
                1 => {
                    if self.program[counter][0..3].to_string() == "inc" {
                        self.program[counter] = "dec".to_string() + &self.program[counter][3..];
                    } else {
                        self.program[counter] = "inc".to_string() + &self.program[counter][3..];
                    }
                }
                2 => {
                    if self.program[counter][0..3].to_string() == "jnz" {
                        self.program[counter] = "cpy".to_string() + &self.program[counter][3..];
                    } else {
                        self.program[counter] = "jnz".to_string() + &self.program[counter][3..];
                    }

                }
                _ => {}
            }
        }

        // println!("----------------------------------------------");
        // self.program.iter().for_each(|inst| { println!("{}", inst); });
        // println!("**********************************************");
        // self.print_regs();
        // println!("----------------------------------------------");
    }

    fn check_output_correct(&self) -> bool {
        for i in 1..self.out.len() {
            if !((self.out[i-1] == 0 && self.out[i] == 1) ||
                (self.out[i-1] == 1 && self.out[i] == 0) ) {
                    return false;
                }
        }
        true
    }

    fn execute(&mut self) -> bool {
        let mut next = true;

        if self.pc >= self.program.len() {
            next = false;
        } else {
            let instruction = &self.program[self.pc];
            let tokens = instruction.split_whitespace().collect::<Vec<&str>>();
            let inst = tokens[0];
            // println!("{} - {} \t\t( {}, {}, {}, {} )", self.pc, instruction, self.reg_a, self.reg_b, self.reg_c, self.reg_d);
            match inst.as_ref() {
                "add" => {
                    let value = self.get_value(tokens[1]);
                    let regstr = tokens[2];
                    match regstr.as_ref() {
                        "a" => { self.reg_a+=value; }
                        "b" => { self.reg_b+=value; }
                        "c" => { self.reg_c+=value; }
                        "d" => { self.reg_d+=value; }
                        _ => {}
                    }                    
                }
                "cpy" => {
                    let value : i64 = self.get_value(tokens[1]);
                    let regstr = tokens[2];
                    match regstr.as_ref() {
                        "a" => { self.reg_a=value; }
                        "b" => { self.reg_b=value; }
                        "c" => { self.reg_c=value; }
                        "d" => { self.reg_d=value; }
                        _ => {}
                    }
                }
                "inc" => {
                    let regstr = tokens[1];
                    match regstr.as_ref() {
                        "a" => { self.reg_a+=1; }
                        "b" => { self.reg_b+=1; }
                        "c" => { self.reg_c+=1; }
                        "d" => { self.reg_d+=1; }
                        _ => {}
                    }
                }
                "dec" => {
                    let regstr = tokens[1];
                    match regstr.as_ref() {
                        "a" => { self.reg_a-=1; }
                        "b" => { self.reg_b-=1; }
                        "c" => { self.reg_c-=1; }
                        "d" => { self.reg_d-=1; }
                        _ => {}
                    }
                }
                "jnz" => {
                    let test = self.get_value(tokens[1]);
                    let jump = self.get_value(tokens[2]);
                    if test != 0 {
                        self.pc += jump as usize;
                        self.pc -= 1;
                    }
                }
                "tgl" => {
                    let offset = self.get_value(tokens[1]) as usize;
                    // println!("Should toggle {}", offset);
                    if self.pc+offset < self.program.len() {
                        let instruction = &self.program[self.pc+offset];
                        let param_count = self.num_params(&instruction);
                        // println!("  instruction is {}, arg count: {}", instruction, param_count);
                        self.toggle(self.pc+offset, param_count);
                    }
                }
                "out" => {
                    let value = self.get_value(tokens[1]);
                    self.out.push(value);
                }
                _ => {}
            }
            self.pc += 1;
            // self.print_regs();
        }

        next
    }
}

#[allow(dead_code)]
pub fn run() {
    println!("Day 25 of 2016");

    let input_file = "./input/day25_16_real.txt";

    let start1 = Instant::now();
    let input = tools::get_input(String::from(input_file));

    let mut lowest_a = 0;
    let mut res1 = -1;
    let mut found = false;
    while !found {
        let mut mycomp1 = Computer::new(lowest_a,0,0,0);
        for line in &input {
            mycomp1.add_line(&line.to_string());
        }
        let mut clock_cycle = 0;
        while clock_cycle < 100000 {
            mycomp1.execute();
            clock_cycle += 1;
        }
        found = mycomp1.check_output_correct();
        if found { res1 = lowest_a; }
        lowest_a += 1;

    }

    let after1 = Instant::now();
    println!("Part 1: {}, in {:?}", res1, after1.duration_since(start1));

    let start2 = Instant::now();

    let after2 = Instant::now();
    println!("Part 2: {}, in {:?}", 0, after2.duration_since(start2));
}

use super::tools;
use std::time::Instant;

struct Computer {
    reg_a: i64,
    reg_b: i64,
    reg_c: i64,
    reg_d: i64,
    program: Vec<String>,
    pc: usize
}

impl Computer {
    fn new(a:i64, b:i64, c:i64, d:i64) -> Computer {
        Computer { reg_a: a, reg_b: b, reg_c: c, reg_d: d, program: vec![], pc: 0}
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

    fn execute(&mut self) -> bool {
        let mut next = true;

        if self.pc >= self.program.len() {
            next = false;
        } else {
            let instruction = &self.program[self.pc];
            let tokens = instruction.split_whitespace().collect::<Vec<&str>>();
            let inst = tokens[0];
            // println!("{}", inst);
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
    println!("Day 12 of 2016");

    // let input_file = "./input/day12_16_test.txt";
    let input_file = "./input/day12_16_real.txt";

    let start1 = Instant::now();
    let input = tools::get_input(String::from(input_file));

    let mut mycomp1 = Computer::new(0,0,0,0);
    let mut mycomp2 = Computer::new(0,0,1,0);
    for line in &input {
        mycomp1.add_line(&line.to_string());
        mycomp2.add_line(&line.to_string());
    }

    while mycomp1.execute() {
        // mycomp1.print_regs();
    }

    let after1 = Instant::now();
    println!("Part 1: {}, in {:?}", mycomp1.reg_a, after1.duration_since(start1));

    let start2 = Instant::now();

    while mycomp2.execute() {}

    let after2 = Instant::now();
    println!("Part 2: {}, in {:?}", mycomp2.reg_a, after2.duration_since(start2));
}

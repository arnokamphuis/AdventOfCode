use super::tools;
use std::time::Instant;
use std::collections::HashMap;

#[derive(Debug)]
struct Instruction {
    op: String,
    reg: Option<char>,
    off: Option<i32>,
}

#[allow(dead_code)]
pub fn run(real: bool, print_result: bool) -> (u128, u128, u128) {
    let start0 = Instant::now();

    let input_file: &str = if !real {
        "./input/day23_15_test.txt"
    } else {
        "./input/day23_15_real.txt"
    };
    let input = tools::get_input(String::from(input_file));

    let program = input.iter().map(|line| {
        let mut i = line.split_whitespace();
        let operator = i.next().unwrap();
        let reg = match operator {
            "inc" | "tpl" | "hlf" | "jio" | "jie" => Some(i.next().unwrap().chars().collect::<Vec<char>>()[0]),
            _ => None,
        };

        let offset = match operator {
            "jmp" | "jio" | "jie" => Some(i.next().unwrap().parse::<i32>().unwrap()),
            _ => None,
        };

        Instruction {
            op: operator.to_string(),
            reg: reg,
            off: offset,
        }
    }).collect::<Vec<Instruction>>();

    let mut registers: HashMap<char, i32> = HashMap::new();
    registers.insert('a', 0);
    registers.insert('b', 0);

    let step = | pc: usize, prog: &Vec<Instruction>, regs: &mut HashMap<char, i32> | -> usize {
        let inst: &Instruction = &prog[pc];
        match inst.op.as_str() {
            "inc" => { *regs.get_mut(&inst.reg.unwrap()).unwrap() += 1; pc+1 },
            "hlf" => { *regs.get_mut(&inst.reg.unwrap()).unwrap() /= 2; pc+1 },
            "tpl" => { *regs.get_mut(&inst.reg.unwrap()).unwrap() *= 3; pc+1 },
            "jmp" => { (pc as i32 + inst.off.unwrap()) as usize },
            "jio" => { if regs[&inst.reg.unwrap()] == 1 { (pc as i32 + inst.off.unwrap()) as usize } else { pc+1 }}
            "jie" => { if regs[&inst.reg.unwrap()] % 2 == 0 { (pc as i32 + inst.off.unwrap()) as usize } else { pc+1 }}
            _ => { prog.len()+1 },
        }
        
    };

    let after0 = Instant::now();

    let start1 = Instant::now();

    let mut pc = 0usize;
    while pc < program.len() { 
        pc = step(pc, &program, &mut registers);
    }

    let res1 = registers[&'b'];

    let after1 = Instant::now();
    if print_result {
        println!("Part 1: {}", res1);
    }

    let start2 = Instant::now();

    *registers.get_mut(&'a').unwrap() = 1;
    *registers.get_mut(&'b').unwrap() = 0;
    let mut pc = 0usize;
    while pc < program.len() { 
        pc = step(pc, &program, &mut registers);
    }

    let res2 = registers[&'b'];

    let after2 = Instant::now();
    if print_result {
        println!("Part 2: {}", res2);
    }

    (
        after0.duration_since(start0).as_nanos(),
        after1.duration_since(start1).as_nanos(),
        after2.duration_since(start2).as_nanos(),
    )
}

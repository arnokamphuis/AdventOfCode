use super::tools;
use std::result::Result;
use std::time::Instant;

#[derive(Debug, Clone)]
pub enum InstructionType {
    NOP, ACC, JMP, INVALID
}

pub fn execute(program: &Vec<(InstructionType, i64)>, doloop: bool) -> Result<(i64, Vec<i64>), &str> {
    let mut executed: Vec<i64> = vec![];
    let mut jmpnop: Vec<i64> = vec![];
    let mut pc: i64 = 0;
    let mut acc: i64 = 0;
    
    while pc != (program.len() as i64) {
        if executed.contains(&pc) {
            if !doloop {
                break;
            } else {
                return Err("looping");
            }
        }

        executed.push(pc);

        let instruction = &program[pc as usize];
        match instruction.0 {
            InstructionType::NOP => {
                jmpnop.push(pc);
            }
            InstructionType::ACC => {
                acc += instruction.1;
            }
            InstructionType::JMP => {
                jmpnop.push(pc);
                pc += instruction.1 - 1;
            }
            InstructionType::INVALID => {}
        }

        pc += 1;
    }
    Ok((acc, jmpnop))
}

#[allow(dead_code)]
pub fn run(real: bool, print_result: bool) -> (u128, u128, u128) {
    let start0 = Instant::now();

    let input_file: &str = if !real {
        "./input/day08_20_test.txt"
    } else {
        "./input/day08_20_real.txt"
    };
    let input = tools::get_input(String::from(input_file));

    let mut program: Vec<(InstructionType, i64)> = vec![];
    for line in &input {
        let inst: Vec<_> = line.split(' ').collect();
        if let Ok(n) = inst[1].parse::<i64>() {
            program.push((
                match inst[0] {
                    "nop" => InstructionType::NOP,
                    "acc" => InstructionType::ACC,
                    "jmp" => InstructionType::JMP,
                    &_ => InstructionType::INVALID
                }, n));
        }
    }

    let after0 = Instant::now();

    let start1 = Instant::now();

    let mut accumulator = 0;
    let mut executed_jmpnop: Vec<i64> = vec![];

    if let Ok(res) = execute(&program, false) {
        accumulator = res.0;
        executed_jmpnop = res.1;
    }

    let after1 = Instant::now();
    if print_result {
        println!("Part 1: {}", accumulator);
    }

    let start2 = Instant::now();

    let mut res2 = 0;
    for i in executed_jmpnop {
        let mut new_program = program.clone();
        if let Some((cmd, _)) = new_program.get_mut(i as usize) {
            match cmd {
                InstructionType::JMP => *cmd = InstructionType::NOP,
                InstructionType::NOP => *cmd = InstructionType::JMP,
                _ => continue,
            }

            match execute(&new_program, true) {
                Ok(res) => {
                    res2 = res.0;
                    break;
                }
                Err(_) => {}
            }
        }
    }

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

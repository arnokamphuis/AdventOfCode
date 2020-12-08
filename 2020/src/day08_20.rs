use super::tools;
use std::result::Result;
use std::time::Instant;

pub fn execute(program: &Vec<(String, i64)>, doloop: bool) -> Result<(i64, Vec<i64>), &str> {
    let mut executed: Vec<i64> = vec![];
    let mut jmpnop: Vec<i64> = vec![];
    let mut pc: i64 = 0;
    let mut acc: i64 = 0;
    loop {
        if executed.contains(&pc) {
            if !doloop {
                break;
            } else {
                return Err("looping");
            }
        }

        executed.push(pc);

        let instruction = &program[pc as usize];
        match instruction.0.as_str() {
            "nop" => {
                jmpnop.push(pc);
            }
            "acc" => {
                acc += instruction.1;
            }
            "jmp" => {
                jmpnop.push(pc);
                pc += instruction.1 - 1;
            }
            _ => {}
        }

        pc += 1;

        if pc == program.len() as i64 {
            break;
        }

        if pc < 0 || pc > (program.len() as i64) {
            return Err("out of bounds");
        }
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

    let mut program: Vec<(String, i64)> = vec![];
    for line in &input {
        let inst: Vec<_> = line.split(' ').collect();
        if let Ok(n) = inst[1].parse::<i64>() {
            program.push((inst[0].to_string(), n));
        }
    }

    let after0 = Instant::now();

    let start1 = Instant::now();

    let mut tobechanged: Vec<i64> = vec![];
    let mut acc = 0;
    if let Ok(res) = execute(&program, false) {
        acc = res.0;
        tobechanged = res.1;
    } else {
        println!("Error in executing part 1");
    }

    let after1 = Instant::now();
    if print_result {
        println!("Part 1: {}", acc);
    }

    let start2 = Instant::now();

    let mut res2 = 0;
    for i in tobechanged {
        let mut new_program = program.clone();
        if let Some((cmd, _)) = new_program.get_mut(i as usize) {
            match cmd.to_string().as_str() {
                "jmp" => *cmd = String::from("nop"),
                "nop" => *cmd = String::from("jmp"),
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

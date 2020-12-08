use super::tools;
use std::result::Result;
use std::time::Instant;

pub fn execute(program: &Vec<(String, i64)>, doloop: bool) -> Result<i64, &str> {
    let mut executed: Vec<i64> = vec![];
    let mut pc: i64 = 0;
    let mut acc: i64 = 0;
    let mut first = true;
    loop {
        if executed.contains(&pc) || (!first && pc == 0) {
            if !doloop {
                break;
            } else {
                println!("{:?}", executed);
                return Err("looping");
            }
        }

        executed.push(pc);

        let instruction = &program[pc as usize];
        match instruction.0.as_str() {
            "nop" => {}
            "acc" => {
                acc += instruction.1;
            }
            "jmp" => {
                pc += instruction.1 - 1;
            }
            _ => {}
        }

        pc += 1;

        // print!("{}-{:?} ", pc, &executed);
        if pc == program.len() as i64 {
            break;
        }

        if pc < 0 || pc > (program.len() as i64) {
            return Err("out of bounds");
        }

        first = false;
    }
    // println!("");

    Ok(acc)
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

    let mut acc = 0;
    if let Ok(res) = execute(&program, false) {
        acc = res;
    } else {
        println!("Error in executing part 1");
    }

    let after1 = Instant::now();
    if print_result {
        println!("Part 1: {}", acc);
    }

    let start2 = Instant::now();

    let mut jmp_lines: Vec<usize> = vec![];
    let mut nop_lines: Vec<usize> = vec![];

    for (i, pl) in program.iter().enumerate() {
        match pl.0.as_str() {
            "jmp" => {
                jmp_lines.push(i);
            }
            "nop" => {
                nop_lines.push(i);
            }
            _ => {}
        }
    }

    let mut res2: i64 = 0;
    for jmp in &jmp_lines {
        let mut new_program = program.clone();
        if let Some((cmd, _)) = new_program.get_mut(*jmp) {
            *cmd = String::from("nop");
        }

        match execute(&new_program, true) {
            Ok(res) => {
                res2 = res;
                println!("Success");
            }
            Err(s) => {
                println!("Error: {}", s);
            }
        }
    }
    for nop in &nop_lines {
        let mut new_program = program.clone();
        if let Some((cmd, _)) = new_program.get_mut(*nop) {
            *cmd = String::from("jmp");
        }

        match execute(&new_program, true) {
            Ok(res) => {
                res2 = res;
                println!("Success");
            }
            Err(s) => {
                println!("Error: {}", s);
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

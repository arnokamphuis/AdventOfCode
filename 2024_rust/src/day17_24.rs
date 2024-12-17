use super::tools;
use std::time::Instant;
use regex::Regex;
use itertools::Itertools;

fn combo(operand: u64, registers: &mut (u64,u64,u64)) -> u64 {
    match operand {
        0 | 1 | 2 | 3 => operand,
        4 => registers.0,
        5 => registers.1,
        6 => registers.2,
        _ => panic!("Invalid operand")
    }
}

fn do_op(pc: u64, program: &Vec<(u64,u64)>, registers: &mut (u64,u64,u64), part: u8) -> (Option<u64>, Option<u64>) {
    let op = program[pc as usize];

    let inst = op.0;
    let operand = op.1;

    let mut out = None;

    match inst {
        0 => { if part == 1 { registers.0 = registers.0 >> combo(operand, registers); } },
        1 => { registers.1 = registers.1 ^ operand; },
        2 => { registers.1 = combo(operand, registers) % 8; },
        3 => { if registers.0 != 0 { return (Some(operand), out); } },
        4 => { registers.1 = registers.1 ^ registers.2; }
        5 => { out = Some(combo(operand, registers) % 8); if part == 2 { return (None, out); } },
        6 => { registers.1 = registers.0 >> combo(operand, registers); },
        7 => { registers.2 = registers.0 >> combo(operand, registers); },
        _ => panic!("Invalid instruction")
    };

    return (Some(pc*2+2), out);
}

fn execute(program: &Vec<(u64,u64)>, registers: (u64,u64,u64), part: u8) -> Vec<u64> {
    let mut pc = 0;
    let mut out = vec![];
    let mut reg = registers;
    while pc < program.len() as u64 {
        let res = do_op(pc, program, &mut reg, part);
        if let Some(o) = res.1 {
            out.push(o);
        }
        pc = res.0.unwrap() / 2_u64;
    }
    return out
}

fn find(program: &Vec<(u64,u64)>, target: Vec<u64>, ans: u64) -> Option<u64> {
    if target.len() == 0 {
        return Some(ans);
    }

    for t in 0..8_u64 {
        let mut registers = ((ans << 3) | t, 0, 0);
        for pc in 0..program.len() as u64 {
            let (_, out) = do_op(pc, program, &mut registers, 2);
            if out != None {
                if out == target.last().copied() {
                    if let Some(res) = find(program, target[0..target.len()-1].to_vec(), registers.0) {
                        return Some(res);
                    }
                }
            }
        }
    }
    None
}

#[allow(dead_code)]
pub fn run(real: bool, print_result: bool) -> (u128, u128, u128) {
    let start0 = Instant::now();

    let input_file: &str = if !real {
        "./input/day17-test.txt"
    } else {
        "./input/day17-real.txt"
    };
    let input = tools::get_input(String::from(input_file));
    let re = Regex::new(r"[-+]?\d+").unwrap();
    let registers: (u64,u64,u64) = re.find_iter(&input[0..3].join(" "))
        .map(|m| m.as_str().parse::<u64>().unwrap())
        .into_iter()
        .collect_tuple()
        .unwrap();
    let program_list: Vec<u64> = re.find_iter(&input[4])
        .map(|m| m.as_str().parse::<u64>().unwrap())
        .collect_vec();
    let program: Vec<(u64,u64)> = program_list.iter()
        .chunks(2)
        .into_iter()
        .map(|mut c| (*c.next().unwrap(), *c.next().unwrap()))
        .collect_vec();

    let after0 = Instant::now();

    let start1 = Instant::now();

    let res1 = execute(&program, registers.clone(), 1)
        .iter()
        .map(|x| x.to_string())
        .collect_vec()
        .join(",");

    let after1 = Instant::now();
    if print_result {
        println!("Part 1: {}", res1);
    }

    let start2 = Instant::now();

    let res2 = find(&program, program_list, 0).unwrap();

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

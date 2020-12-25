use super::tools;
use std::result::Result;
use std::time::Instant;

// use std::collections::BTreeMap;
// use petgraph::graph::{Graph, NodeIndex};
// use petgraph::dot::Dot;
// use std::fs::File;
// use std::io::Write;

#[derive(Debug, Clone, Eq, PartialEq)]
pub enum InstructionType {
    NOP, ACC, JMP, INVALID
}

// pub fn save_graph(program: &Vec<(InstructionType, i64)>, steps: &Vec<Vec<i64>>) {
//     let mut graph = Graph::<String, usize>::new();
//     let mut nodes: BTreeMap<i64, NodeIndex> = BTreeMap::new();

//     for (index,instr) in program.iter().enumerate() {
//         let label = index.to_string();
//         nodes.insert(index as i64, graph.add_node(label));
//     }
//     nodes.insert(program.len() as i64, graph.add_node(String::from("THE END")));

//     let mut edges = vec![];

//     for (runindex, run) in steps.iter().enumerate() {
//         for i in 1..run.len() {
//             edges.push( (*nodes.get(&run[i-1]).unwrap(), *nodes.get(&run[i]).unwrap(), runindex ));
//         }
//         graph.extend_with_edges(edges.iter());
//     }


//     let mut output: File = File::create("graphs/program.dot").unwrap();
//     output.write_all(format!("{}", Dot::new(&graph)).as_bytes());
    
// }

pub fn execute(program: &Vec<(InstructionType, i64)>, doloop: bool) -> Result<(i64, Vec<i64>, Vec<i64>), (&str, Vec<i64>)> {
    let mut executed: Vec<i64> = vec![];
    let mut jmpnop: Vec<i64> = vec![];
    let mut pc: i64 = 0;
    let mut acc: i64 = 0;
    
    while pc != (program.len() as i64) {
        if executed.contains(&pc) {
            if !doloop {
                break;
            } else {
                return Err( ("looping", executed) );
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
    // if !doloop {
    //     save_graph(program, &executed);
    // }
    Ok((acc, jmpnop, executed))
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

    let mut steps: Vec<Vec<i64>> = vec![];
    let mut res2 = 0;
    for i in executed_jmpnop.iter().rev() {
        let mut new_program = program.clone();
        if let Some((cmd, _)) = new_program.get_mut(*i as usize) {
            match cmd {
                InstructionType::JMP => *cmd = InstructionType::NOP,
                InstructionType::NOP => *cmd = InstructionType::JMP,
                _ => continue,
            }

            match execute(&new_program, true) {
                Ok(res) => {
                    res2 = res.0;
                    steps.push(res.2);
                    break;
                }
                Err(res) => {
                    steps.push(res.1);
                }
            }
        }
    }

    // save_graph(&program, &steps);

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

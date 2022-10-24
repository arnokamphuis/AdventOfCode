// use super::tools;
use std::time::Instant;

// fn reg_index(s: &String) -> usize {
//     match s.as_str() {
//         "w" => 0,
//         "x" => 1,
//         "y" => 2,
//         "z" => 3,
//         _ => panic!("reg_index {}", s)
//     }
// }

// struct ALU {
//     registers: [i64;4],
//     pc: usize,
//     program: Vec<(u8, String, String)>,
//     input: String,
// }

// impl ALU {
//     fn new() -> ALU {
//         ALU{registers: [0;4], pc: 0, program: vec![], input: String::from("")}
//     }

//     fn add_instruction(&mut self, line: &String) {
//         let mut inst;
//         let mut tokens = line.split(" ");
//         match tokens.next().unwrap() {
//             "inp" => { inst = 0; }
//             "add" => { inst = 1; }
//             "mul" => { inst = 2; }
//             "div" => { inst = 3; }
//             "mod" => { inst = 4; }
//             "eql" => { inst = 5; }
//             _ => { panic!() }
//         };
//         let to: String = tokens.next().unwrap().to_string();
//         let using: String = if inst > 0 { tokens.next().unwrap().to_string() } else { "".to_string() };
//         self.program.push((inst, to, using));
//     }

//     fn get_value(&self, s: &String) -> i64 {
//         if let Ok(val) = s.parse::<i64>() {
//             val
//         } else {
//             self.registers[reg_index(s)]
//         }
//     }

//     fn execute(&mut self) {
//         self.registers = [0;4];
//         let mut local_input = self.input.clone();
//         (0..self.program.len()).for_each(|pc| {
//             let instruction = self.program[pc].clone();

//             if instruction.0 == 0 {
//                 println!("registers: {:?}", self.registers);
//             }

//             let writereg = reg_index(&instruction.1);
//             self.registers[writereg] = match instruction.0 {
//                 0 => { self.input.pop().unwrap().to_digit(10).unwrap() as i64 }
//                 1 => {  self.get_value(&instruction.1) +  self.get_value(&instruction.2) }
//                 2 => {  self.get_value(&instruction.1) *  self.get_value(&instruction.2) }
//                 3 => {  self.get_value(&instruction.1) /  self.get_value(&instruction.2) }
//                 4 => {  self.get_value(&instruction.1) %  self.get_value(&instruction.2) }
//                 5 => { (self.get_value(&instruction.1) == self.get_value(&instruction.2)) as i64 }
//                 _ => panic!()
//             };
//         });
//     }
// }

#[allow(dead_code)]
pub fn run(_real: bool, print_result: bool) -> (u128, u128, u128) {
    let start0 = Instant::now();

    // let input_file: &str = if !real {
    //     "./input/day24_21_test.txt"
    // } else {
    //     "./input/day24_21_real.txt"
    // };
    // let input = tools::get_input(String::from(input_file));

    // let mut alu = ALU::new();
    // input.iter().for_each(|line| {alu.add_instruction(line);} );

    // let check_number = | unit: &mut ALU, number: u64 | -> bool {
    //     unit.input = number.to_string();
    //     if !unit.input.chars().any(|c| c=='0') {
    //         unit.execute();
    //         unit.registers[3] == 0
    //     } else {
    //         false
    //     }
    // };

    let after0 = Instant::now();

    let start1 = Instant::now();

    let program_xs = [11, 12, 10, -8, 15, 15, -11, 10, -3, 15, -3, -1, -10, -16];
    let program_zs = [1, 1, 1, 26, 1, 1, 26, 1, 26, 1, 26, 26, 26, 26]; 
    let program_ys = [8, 8, 12, 10, 2, 8, 4, 9, 10, 3, 7, 7, 2, 2];

    let find = | mut from, updown | -> i64 {
        let mut program_ws: Vec<i64>;
        loop {
            program_ws = (0..14).fold(vec![], |mut v, i| { v.push(from/10_i64.pow(i) % 10); v } );
            from += updown;
    
            if program_ws.iter().all(|&v|v!=0) {
                let mut z = 0;
                for i in 0..14 {
                    let z26  = z % 26;
                    let goal = z26 + program_xs[i];
    
                    if 1 <= goal && goal <= 9 {
                        program_ws[i] = goal;
                    }
                
                    z = z / program_zs[i];
                    let x = if program_ws[i]==goal { 0 } else { 1 };
                    z *= if x==1 { 25 } else { 0 } + 1;
                    z += if x==1 { program_ws[i] + program_ys[i] } else { 0 }; 
                    if program_xs[i] < 10 && program_ws[i] != goal {
                        break;
                    }
                }
                if z == 0 {
                    break;
                }
            }
        }
        program_ws.iter().fold(0, |v, w| v * 10 + w)
    };

    let res1 = find(99999999999999, -1);

    let after1 = Instant::now();
    if print_result {
        println!("Part 1: {}", res1);
    }

    let start2 = Instant::now();

    let res2 = find(11111111111111, 1);

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

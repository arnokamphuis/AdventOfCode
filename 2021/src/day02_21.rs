use super::tools;
use std::time::Instant;

struct SubMarine {
    memory: Vec<(String,i64)>,
    pos: (i64,i64,i64)
}


impl SubMarine {
    fn new(m: &Vec<String>) -> SubMarine {
        let mut sub = SubMarine{memory: vec![], pos: (0,0,0)};
        m.iter().for_each(|l| {
            let mut tokens = l.split_whitespace();
            let op = String::from(tokens.next().unwrap());
            let val = tokens.next().unwrap().parse::<i64>().unwrap();
            sub.memory.push((op,val));
        });
        sub
    }

    fn run(&mut self, part: u8) -> (i64,i64,i64) {
        self.pos = (0,0,0);
        for (inst, amount) in &self.memory {
            match inst.as_str() {
                "forward" => {
                    self.pos.0 += amount; 
                    if part == 2 {
                        self.pos.1 += amount * self.pos.2;
                    }
                }
                "down" => { 
                    if part == 1 {
                        self.pos.1 += amount;
                    } else {
                        self.pos.2 += amount; 
                    }
                }
                "up" => {
                    if part == 1 { 
                        self.pos.1 -= amount; 
                    } else {
                        self.pos.2 -= amount; 
                    }
                }
                _ => {}
            }
        }
        self.pos
    }
}

#[allow(dead_code)]
pub fn run(real: bool, print_result: bool) -> (u128, u128, u128) {
    let start0 = Instant::now();

    let input_file: &str = if !real {
        "./input/day02_21_test.txt"
    } else {
        "./input/day02_21_real.txt"
    };
    let input = tools::get_input(String::from(input_file));

    let mut submarine = SubMarine::new(&input.clone());

    let after0 = Instant::now();

    let start1 = Instant::now();

    let pos = submarine.run(1);
    let res1 = pos.0 * pos.1;

    let after1 = Instant::now();
    if print_result {
        println!("Part 1: {}", res1);
    }

    let start2 = Instant::now();

    let pos = submarine.run(2);
    let res2 = pos.0 * pos.1;

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

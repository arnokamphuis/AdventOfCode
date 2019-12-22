use std::time::{Instant};
use super::tools;
use super::intcode::IntCodeComputer;
use super::intcode::get_commands_from_line;
use std::collections::BTreeMap;

struct TractorBeam {
    memory: BTreeMap<i64, i64>,
}

impl TractorBeam {
    fn new(mem: &BTreeMap<i64, i64>) -> TractorBeam {
        TractorBeam {
            memory: mem.clone(),
        }
    }

    fn check(&mut self, pos: (i64,i64)) -> bool {
        let mut dronesystem = IntCodeComputer::new(&self.memory, true);
        dronesystem.add_input(pos.0);
        dronesystem.add_input(pos.1);
        dronesystem.run();
        if let Some(out) = dronesystem.get_output() {
            (out==1)
        } else {
            false
        }
    }
}


#[allow(dead_code)]
pub fn run() {
    println!("Day 19 of 2019");

    let start0 = Instant::now();

    let input_file = "./input/day19_19_real.txt";
    let input = tools::get_input(String::from(input_file));

    let commands = get_commands_from_line(&input[0]);

    let after0 = Instant::now();
    println!("Init in {:?}", after0.duration_since(start0));

    let start1 = Instant::now();
    let mut beam = TractorBeam::new(&commands);
    let mut count = 0;
    let mut pos = (0,0);
    for y in 0..50 {
        let mut left_empty = 0;
        for x in 0..50 {
            if beam.check((x,y)) {
                count += 1;
                if left_empty==0 {
                    pos = (x,y);
                    left_empty = x;
                }
            }
        }
    }

    let after1 = Instant::now();
    println!("Part 1: {}, in {:?}", count, after1.duration_since(start1));

    let start2 = Instant::now();

    let res2;

    loop {
        let ll = pos;
        let lu = (pos.0, pos.1 - 99);
        let rl = (pos.0 + 99, pos.1);
        let ru = (pos.0 + 99, pos.1 - 99);

        if beam.check(ll) && beam.check(lu) && beam.check(rl) && beam.check(ru) {
            res2 = lu.0 * 10000 + lu.1;
            break;
        } else {
            pos.1 += 1;
            while !beam.check(pos) { pos.0 +=1;}
        }
    }


    let after2 = Instant::now();
    println!("Part 2: {}, in {:?}", res2, after2.duration_since(start2));
}

use super::intcodestepper::{IntCodeStepper, LoopStatus};
use super::tools;
use std::collections::BTreeMap;
use std::time::Instant;

struct Catergory6 {
    nodes: Vec<IntCodeStepper>,
    messages: BTreeMap<i64, Vec<(i64, i64)>>,
    nat: (i64, i64),
    nat_counter: i64,
}

impl Catergory6 {
    fn new(m: &BTreeMap<i64, i64>) -> Catergory6 {
        Catergory6 {
            nodes: vec![IntCodeStepper::new(m); 50],
            messages: BTreeMap::new(),
            nat: (0, 0),
            nat_counter: 0,
        }
    }

    fn run(&mut self, part: usize) -> Option<i64> {
        let mut i = 0;
        let mut prev_y_nat = 0;
        for n in &mut self.nodes {
            n.add_input(i);
            self.messages.insert(i, vec![]);
            i += 1;
        }

        let mut idle: Vec<bool> = vec![false; 50];

        loop {
            let mut i = 0;

            if part == 2 {
                let mqc = self.messages.iter().filter(|(_,v)| v.len() == 0).count();
                let idle_count = idle.iter().filter(|i| **i).count();
                if mqc == 50 {
                    let nat = self.nat;
                    if idle_count == 50 {
                        self.messages.entry(0).and_modify(|v| v.push(nat));
                        if self.nat_counter > 0 {
                            if nat.1 == prev_y_nat {
                                return Some(nat.1);
                            }
                        }
                        self.nat_counter+=1;
                        prev_y_nat = nat.1;
                    }
                }
            }

            for n in &mut self.nodes {

                if n.step() == LoopStatus::HaltedForInput {
                    if self.messages[&i].len() > 0 {
                        if self.messages[&i].len() > 0 {
                            idle[i as usize] = false;
                            let mut msg = (0, 0);
                            self.messages.entry(i).and_modify(|v| msg = v.remove(0));
                            n.add_input(msg.0);
                            n.add_input(msg.1);
                        }
                    } else {
                        idle[i as usize] = true;
                        n.add_input(-1);
                    }
                }


                if n.has_output() == 3 {
                    let address = n.get_output().unwrap();
                    let x = n.get_output().unwrap();
                    let y = n.get_output().unwrap();
                    if address == 255 {
                        if part == 1 {
                            return Some(y);
                        } else {
                            self.nat = (x, y);
                        }
                    } else {
                        self.messages.entry(address).and_modify(|v| v.push((x, y)));
                    }
                }
                i += 1;
            }
        }
    }
}

#[allow(dead_code)]
pub fn run() {
    println!("Day 23 of 2019");

    let start0 = Instant::now();

    let input_file = "./input/day23_19_real.txt";
    let input = tools::get_input(String::from(input_file));
    let commands = tools::get_commands_from_line(&input[0]);

    let mut cat6_1 = Catergory6::new(&commands);
    let mut cat6_2 = Catergory6::new(&commands);

    let after0 = Instant::now();
    println!("Init in {:?}", after0.duration_since(start0));

    let start1 = Instant::now();

    let res1 = cat6_1.run(1).unwrap();

    let after1 = Instant::now();
    println!("Part 1: {}, in {:?}", res1, after1.duration_since(start1));

    let start2 = Instant::now();

    let res2 = cat6_2.run(2).unwrap();

    let after2 = Instant::now();
    println!("Part 2: {}, in {:?}", res2, after2.duration_since(start2));
}

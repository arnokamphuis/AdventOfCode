use super::intcodestepper::{IntCodeStepper, LoopStatus};
use super::tools;
use std::collections::BTreeMap;
// use std::io::{stdin, stdout, Write};
use std::time::Instant;

#[allow(dead_code)]
fn from_ascii(c: char) -> i64 {
    (c as u8) as i64
}

fn to_ascii(c: i64) -> char {
    (c as u8) as char
}

struct AsciiComputer {
    computer: IntCodeStepper,
}

impl AsciiComputer {
    fn new(m: &BTreeMap<i64, i64>) -> AsciiComputer {
        AsciiComputer {
            computer: IntCodeStepper::new(m),
        }
    }

    fn drop_all(&mut self) {
        let inv = [
            "drop hologram",
            "drop space heater",
            "drop festive hat",
            "drop food ration",
            "drop spool of cat6",
            "drop fuel cell",
            "drop space law space brochure",
            "drop tambourine",
        ];

        for item in &inv {
            item.chars().for_each(|c| {
                if from_ascii(c) != 13 {
                    // print!("{}", c);
                    self.computer.add_input(from_ascii(c));
                }
            });
            // println!("");
            self.computer.add_input(10);
        }
    }

    fn hold(&mut self, option: u16) {
        let inv = [
            "take hologram",
            "take space heater",
            "take festive hat",
            "take food ration",
            "take spool of cat6",
            "take fuel cell",
            "take space law space brochure",
            "take tambourine",
        ];

        self.drop_all();
        let mut v = option;
        let mut index: usize = 0;
        while v > 0 {
            if (v & 1) == 1 {
                inv[index].chars().for_each(|c| {
                    if from_ascii(c) != 13 {
                        self.computer.add_input(from_ascii(c));
                    }
                });
                self.computer.add_input(10);
            }
            v = v >> 1;
            index += 1;
        }
    }

    fn go_south(&mut self) {
        "south".chars().for_each(|c| {
            if from_ascii(c) != 13 {
                self.computer.add_input(from_ascii(c));
            }
        });
        self.computer.add_input(10);
    }

    fn run(&mut self) -> String {
        let start_script = tools::get_input(String::from("./input/day25_19_startscript.txt"));
        for s in start_script {
            for c in s.chars() {
                // print!("{} ", from_ascii(c));
                self.computer.add_input(from_ascii(c));
            }
            // print!("{} ", 10);
            self.computer.add_input(10);
        }
        self.drop_all();

        let mut option = 0;
        let mut output: Vec<String> = vec![];

        let mut out = String::from("");
        loop {
            let mut status = self.computer.step();
            while status == LoopStatus::Success {
                status = self.computer.step();
            }

            while let Some(out_c) = self.computer.get_output() {
                out.push(to_ascii(out_c));
                if out_c == 10 {
                    // println!("{}   ({})", out, output.len());
                    if let Some(_) = out.clone().find("Oh, hello!") {
                        return out[51..58].to_string();
                    }
                    output.push(out);
                    out = String::from("");
                // print!("found return {}", to_ascii(out_c));
                } else {
                    // print!("{}", to_ascii(out_c));
                }
            }

            if status == LoopStatus::HaltedForInput {
                option += 1;
                if option < 1024 {
                    self.hold(option);
                    self.go_south();
                }

                // manual:
                // let mut s = String::new();
                // stdin().read_line(&mut s);
                // s.chars().for_each(|c| {
                //     if from_ascii(c) != 13 {
                //         self.computer.add_input(from_ascii(c));
                //     }
                // });
            }
        }
    }
}

#[allow(dead_code)]
pub fn run() {
    println!("Day 25 of 2019");

    let start0 = Instant::now();

    let input_file = "./input/day25_19_real.txt";
    let input = tools::get_input(String::from(input_file));
    let commands = tools::get_commands_from_line(&input[0]);

    let mut textadventure1 = AsciiComputer::new(&commands);

    let after0 = Instant::now();
    println!("Init in {:?}", after0.duration_since(start0));

    let start1 = Instant::now();

    let res1 = textadventure1.run();

    let after1 = Instant::now();
    println!("Part 1: {}, in {:?}", res1, after1.duration_since(start1));

    let start2 = Instant::now();

    let after2 = Instant::now();
    println!("Part 2: {}, in {:?}", 0, after2.duration_since(start2));
}

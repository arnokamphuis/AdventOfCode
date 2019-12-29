use std::collections::HashMap;
use std::time::Instant;

use super::tools;

struct Bot {
    id: usize,
    high_to: i64,
    low_to: i64,
    chips: Vec<u64>,
}

struct Bots {
    responsible_bot: usize,
    bots: HashMap<usize, Bot>,
    output: Vec<(usize, u64)>,
}

impl Bots {
    fn new() -> Bots {
        Bots {
            responsible_bot: std::usize::MAX,
            bots: HashMap::new(),
            output: vec![],
        }
    }
    fn add_bot(&mut self, b: Bot) {
        self.bots.insert(b.id, b);
    }

    fn add_value(&mut self, b_id: usize, value: u64) {
        let target_bot = self.bots.get_mut(&b_id).unwrap();
        target_bot.add_chip(value);
    }

    fn process(&mut self) -> bool {
        let mut mutations: Vec<(usize, u64)> = vec![];
        for (_id, b) in &mut self.bots {
            if b.needs_processing() {
                let high = b.get_high();
                let low = b.get_low();
                if high == 61 && low == 17 {
                    self.responsible_bot = b.id;
                }
                let to_high = b.high_to;
                let to_low = b.low_to;

                b.remove_chip(low);
                if to_low < 0 {
                    self.output.push((-to_low as usize - 1, low));
                } else {
                    mutations.push((to_low as usize - 1, low));
                }

                b.remove_chip(high);
                if to_high < 0 {
                    self.output.push((-to_high as usize - 1, high));
                } else {
                    mutations.push((to_high as usize - 1, high));
                }
            }
        }
        for m in &mutations {
            let target_bot = self.bots.get_mut(&m.0).unwrap();
            target_bot.add_chip(m.1);
        }
        mutations.len() != 0
    }

    fn multiply_outputs(&self, ids: Vec<usize>) -> u64 {
        let mut mult: u64 = 1;
        for id in ids {
            for o in &self.output {
                if o.0 == id {
                    mult *= o.1;
                }
            }
        }
        mult
    }
}

impl Bot {
    // Construct a bot
    fn new(identifier: usize, ht: i64, lt: i64) -> Bot {
        Bot {
            id: identifier,
            high_to: ht,
            low_to: lt,
            chips: vec![],
        }
    }

    fn needs_processing(&self) -> bool {
        self.chips.len() == 2
    }

    // add chips
    fn add_chip(&mut self, v: u64) {
        self.chips.push(v);
    }

    fn remove_chip(&mut self, v: u64) {
        self.chips.retain(|&x| x != v);
    }

    fn get_high(&self) -> u64 {
        if self.chips.len() == 0 {
            println!("ERROR");
            0
        } else if self.chips.len() == 1 {
            self.chips[0]
        } else {
            std::cmp::max(self.chips[0], self.chips[1])
        }
    }

    fn get_low(&self) -> u64 {
        if self.chips.len() == 0 {
            println!("ERROR");
            0
        } else if self.chips.len() == 1 {
            self.chips[0]
        } else {
            std::cmp::min(self.chips[0], self.chips[1])
        }
    }
}

#[allow(dead_code)]
pub fn run() {
    println!("Day 10 of 2016");

    // let input_file = "./input/day10_16_test.txt";
    let input_file = "./input/day10_16_real.txt";

    let start1 = Instant::now();

    let mut input = Vec::new();

    if let Ok(lines) = tools::read_lines(input_file) {
        for line in lines {
            if let Ok(value) = line {
                input.push(value);
            }
        }
    }

    let mut bots: Bots = Bots::new();
    let mut values: Vec<(usize, u64)> = vec![];
    for operation in &input {
        let mut tokens = operation.split_whitespace();
        let type_of_operation = tokens.next().unwrap();
        match type_of_operation {
            "value" => {
                let v = tokens.next().unwrap().parse::<u64>().unwrap();
                tokens.next();
                tokens.next();
                tokens.next();
                let b = tokens.next().unwrap().parse::<usize>().unwrap();
                values.push((b, v));
            }
            "bot" => {
                let b_id = tokens.next().unwrap().parse::<usize>().unwrap();
                tokens.next(); // gives
                tokens.next(); // low
                tokens.next(); // to
                let mut target = tokens.next().unwrap(); // output or bot
                let mut low_id = tokens.next().unwrap().parse::<i64>().unwrap() + 1;
                if target == "output" {
                    low_id = -low_id;
                }
                tokens.next(); // and
                tokens.next(); // high
                tokens.next(); // to
                target = tokens.next().unwrap(); // output or bot
                let mut high_id = tokens.next().unwrap().parse::<i64>().unwrap() + 1;
                if target == "output" {
                    high_id = -high_id;
                }
                // println!("Created bot {} with targets {}, {}", b_id, high_id, low_id);
                let b = Bot::new(b_id, high_id, low_id);
                bots.add_bot(b);
            }
            _ => {}
        }
    }
    for (b_id, value) in values {
        bots.add_value(b_id, value);
    }

    while bots.process() {}

    let after1 = Instant::now();
    println!(
        "Part 1: {}, in {:?}",
        &bots.responsible_bot,
        after1.duration_since(start1)
    );

    let start2 = Instant::now();

    let mult = bots.multiply_outputs(vec![0, 1, 2]);

    let after2 = Instant::now();
    println!("Part 2: {}, in {:?}", mult, after2.duration_since(start2));
}

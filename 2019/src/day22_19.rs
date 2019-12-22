use std::time::{Instant};
use super::tools;

struct Deck {
    cards: Vec<u32>,
}

impl Deck {
    fn new(length: usize) -> Deck {
        let mut d = Deck {
            cards: vec![],
        };
        for i in 0..length {
            d.cards.push(i as u32);
        }

        d
    }

    fn cut(&mut self, cutoff: i32) {
        let mut co_index: usize = 0;
        let mut res = self.cards.clone();
        if cutoff < 0 {
            let mut co = cutoff;
            while co < 0 { co += res.len() as i32; }
            co_index = co as usize;
        } else {
            co_index = cutoff as usize;
        }
        let last = &res[co_index..res.len()].to_vec();
        let first = &res[0..co_index].to_vec();

        self.cards = last.clone();
        self.cards.append(&mut first.clone());
    }

    fn newstack(&mut self) {
        self.cards.reverse();
    }

    fn increment(&mut self, increment: usize) {
        let mut res: Vec<u32> = vec![0; self.cards.len()];
        let mut index = 0;
        for i in 0..self.cards.len() {
            res[index] = self.cards[i];
            index += increment;
            index = index % self.cards.len();
        }
        self.cards = res.clone();
    }

    fn perform(&mut self, line: &String) {
        let mut iter = line.split_whitespace();
        let c1 = iter.next().unwrap();
        match c1 {
            "cut" => {
                let offset = iter.next().unwrap().parse::<i32>().unwrap();
                self.cut(offset);
            }
            "deal" => {
                let c2 = iter.next().unwrap();
                match c2 {
                    "with" => {
                        iter.next();
                        let increment = iter.next().unwrap().parse::<usize>().unwrap();
                        self.increment(increment);        
                    }
                    "into" => {
                        self.newstack();
                    }
                    _ => {}
                }
            }
            _ => {}
        }
    }
}


fn modinv(mut a: i128, mut base: i128) -> i128 {
    if base == 1 {
        return 0;
    }

    let orig = base;

    let mut x = 1;
    let mut y = 0;

    while a > 1 {
        let q = a / base;
        let tmp = base;
        base = a % base;
        a = tmp;
        let tmp = y;
        y = x - q * y;
        x = tmp;
    }

    if x < 0 {
        x + orig
    } else {
        x
    }
}

fn modp(b: i128, exp: i128, base: i128) -> i128 {
    let mut x = 1;
    let mut p = b % base;

    for i in 0..128 {
        if 1 & (exp >> i) == 1 {
            x = x * p % base;
        }

        p = p * p % base;
    }

    x
}


struct EfficientDeck {
    cards: i128,
    repeats: i128,
    increment_mul: i128,
    offset_diff: i128,
}

impl EfficientDeck {
    fn new(c: i128, r: i128) -> EfficientDeck {
        EfficientDeck {
            cards: c,
            repeats: r,
            increment_mul: 1,
            offset_diff: 0,
        }
    }

    fn final_calc(&self) -> i128 {
        let i1 = modp(self.increment_mul, self.repeats, self.cards) * 2020i128 % self.cards;
        let i2 = (modp(self.increment_mul, self.repeats, self.cards) + self.cards - 1) % self.cards;
        let i3 = self.offset_diff * i2 % self.cards;
        let i4 = modp(self.increment_mul - 1, self.cards - 2, self.cards);
        (i1 + i3 * i4) % self.cards
    }

    fn perform(&mut self, line: &String) {
        let mut iter = line.split_whitespace();
        let c1 = iter.next().unwrap();
        match c1 {
            "cut" => {
                let offset = iter.next().unwrap().parse::<i128>().unwrap();
                self.offset_diff += if offset < 0 { offset + self.cards } else { offset };
            }
            "deal" => {
                let c2 = iter.next().unwrap();
                match c2 {
                    "with" => {
                        iter.next();
                        let increment = iter.next().unwrap().parse::<i128>().unwrap();

                        let inv = modinv(increment, self.cards);
                        self.increment_mul = self.increment_mul * inv % self.cards;
                        self.offset_diff = self.offset_diff * inv % self.cards;
                    }
                    "into" => {
                        self.offset_diff += 1;
                        self.offset_diff *= -1;
                        self.increment_mul *= -1;
                    }
                    _ => {}
                }
            }
            _ => {}
        }

        self.increment_mul %= self.cards;
        self.offset_diff %= self.cards;

        if self.increment_mul < 0 {
            self.increment_mul += self.cards;
        }

        if self.offset_diff < 0 {
            self.offset_diff += self.cards;
        }
    }
}

#[allow(dead_code)]
pub fn run() {
    println!("Day 22 of 2019");

    let start0 = Instant::now();

    // let input_file = "./input/day22_19_test.txt";
    let input_file = "./input/day22_19_real.txt";
    let input = tools::get_input(String::from(input_file));

    // const DECKSIZE: usize = 10;
    const DECKSIZE: usize = 10007;

    let after0 = Instant::now();
    println!("Init in {:?}", after0.duration_since(start0));

    let start1 = Instant::now();

    let mut cards = Deck::new(DECKSIZE);

    for line in &input {
        cards.perform(&line);
    }
    
    let mut res1 = 0;
    if DECKSIZE > 10 {
        for (i,v) in cards.cards.iter().enumerate() {
            if *v == 2019 {
                res1 = i;
            }
        }
    } else {
        for i in cards.cards {
            print!("{} ", i);
        }
        println!("");
    }

    let after1 = Instant::now();
    println!("Part 1: {}, in {:?}", res1, after1.duration_since(start1));

    let start2 = Instant::now();

    const DECKSIZE2: i128 = 119315717514047;
    const REPEATS: i128 = 101741582076661;

    let mut deck2 = EfficientDeck::new(DECKSIZE2, REPEATS);
    for line in input.iter().rev() {
        deck2.perform(&line);
    }


    let res2 = deck2.final_calc();
    
    let after2 = Instant::now();
    println!("Part 2: {}, in {:?}", res2, after2.duration_since(start2));
}

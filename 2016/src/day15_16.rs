use std::time::Instant;
use super::tools;
use std::collections::HashSet;

struct Disc {
    level: i64,
    positions: i64,
    offset: i64,
}

impl Disc {
    fn new(init: &String) -> Disc {
        let mut disc = Disc{level: 0, positions: 0, offset: 0 };
        let mut iter = init.split_whitespace();
        iter.next(); 
        if let Some(number_str) = iter.next() {
            if let Ok(number) = number_str[1..].parse::<i64>() {
                disc.level = number;
            }
        }
        iter.next();
        if let Some(number_str) = iter.next() {
            if let Ok(number) = number_str.parse::<i64>() {
                disc.positions = number;
            }
        }
        iter.next(); // positions;
        iter.next(); // at
        iter.next(); // at time=0;
        iter.next(); iter.next(); iter.next(); iter.next(); // it is at position
        if let Some(number_str) = iter.next() {
            if let Ok(number) = number_str[..number_str.len()-1].parse::<i64>() {
                disc.offset = number;
            }
        }
        
        disc
    }

    fn gen_set(&self, duration: i64) -> HashSet<i64> {
        let mut set = HashSet::new();
        let mut x = self.positions - (self.offset + self.level);
        while x < 0 { x += self.positions; }
        set.insert(x);
        while x < duration {
            x += self.positions;
            set.insert(x);
        }
        set
    }
}

fn determine_time(discs: &Vec<Disc>) -> i64 {
    let mut res: i64 = std::i64::MAX;
    let mut found = false;
    let mut duration = 100;
    while !found {
        let mut sets = vec![];
        discs.iter().for_each(|d| {sets.push(d.gen_set(duration)); });

        let mut intersection: HashSet<i64> = sets[0].clone();
        for i in 1..sets.len() {
            let inter = intersection.intersection(&sets[i]);
            let mut newset: HashSet<i64> = HashSet::new();
            inter.for_each(|i| { newset.insert(*i); });
            intersection = newset.clone();
        }

        found = intersection.len() > 0;
        if !found {
            duration *= 2;
        } else {

            intersection.iter().for_each(|s| {
                if res > *s {
                    res = *s;
                }
            });
        
        }
    }
    res
}

#[allow(dead_code)]
pub fn run() {
    println!("Day 15 of 2016");

    let start0 = Instant::now();

    // let input_file = "./input/day15_16_test.txt";
    let input_file = "./input/day15_16_real.txt";
    let input = tools::get_input(String::from(input_file));

    let mut discs : Vec<Disc> = vec![];
    input.iter().for_each(|i| { discs.push(Disc::new(i)); }  );

    let after0 = Instant::now();
    println!(
        "Init: in {:?}",
        after0.duration_since(start0)
    );

    let start1 = Instant::now();

    let res1: i64 = determine_time(&discs);

    let after1 = Instant::now();
    println!(
        "Part 1: {}, in {:?}",
        res1,
        after1.duration_since(start1)
    );

    let start2 = Instant::now();

    discs.push(Disc{offset: 0, positions: 11, level: 1+discs.len() as i64 });
    // discs.iter().for_each(|d| { println!("{} {} {}", d.offset, d.positions, d.level);} );
    let res2: i64 = determine_time(&discs);

    let after2 = Instant::now();
    println!(
        "Part 2: {}, in {:?}",
        res2,
        after2.duration_since(start2)
    );
}

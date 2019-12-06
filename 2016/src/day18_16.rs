use std::time::Instant;
use super::tools;

struct Floor {
    last: Vec<bool>,
    safe_counter: usize,
    row_counter: usize
}

impl Floor {
    fn new(init: &String) -> Floor {
        let mut floor = Floor {
            last: vec![],
            safe_counter: 0,
            row_counter: 0
        };
        let mut row: Vec<bool> = vec![];
        init.chars().for_each(|c| { row.push( if c == '.' { false } else { true } ) } );
        floor.safe_counter += floor.count_safe(&row);
        floor.row_counter = 1;
        floor.last = row.clone();
        floor
    }

    fn next(&mut self) -> usize {
        let from: Vec<bool> = self.last.clone();
        let mut newline: Vec<bool> = vec![false; from.len()];
        let len = from.len();
        let last = len-1;


        if ( from[0]  && from[1]  ) || ( !from[0] && from[1]  ) {
            newline[0] = true;
        }

        if ( from[last-1] && from[last] ) || ( from[last-1]  && !from[last] ) {
            newline[last] = true;
        }

        for i in 1..len-1 {
            if ( from[i-1] && from[i] && !from[i+1] ) ||
            ( !from[i-1] && from[i]  && from[i+1]  ) ||
            ( from[i-1]  && !from[i] && !from[i+1] ) ||
            ( !from[i-1] && !from[i] && from[i+1]  ) {
                newline[i] = true;
            }
        }
        
        self.safe_counter += self.count_safe(&newline);
        self.last = newline.clone();
        self.row_counter += 1;
        self.row_counter
    }

    fn count_safe(&self, row: &Vec<bool>) -> usize {
        let mut count = 0;
        row.iter().for_each(|b| {
            count += if *b { 0 } else { 1 };
        });
        count
    }
}

#[allow(dead_code)]
pub fn run() {
    println!("Day 18 of 2016");

    let start0 = Instant::now();

    // let input_file = "./input/day18_16_test.txt";
    let input_file = "./input/day18_16_real.txt";
    let input = tools::get_input(String::from(input_file));

    let line: String = input[0].clone();

    let mut floor1 = Floor::new(&line);
    let mut floor2 = Floor::new(&line);

    let after0 = Instant::now();
    println!(
        "Init in {:?}",
        after0.duration_since(start0)
    );

    let start1 = Instant::now();

    while (&mut floor1).next() < 40 {}

    let c1 = floor1.safe_counter;

    let after1 = Instant::now();
    println!(
        "Part 1: {}, in {:?}",
        c1,
        after1.duration_since(start1)
    );

    let start2 = Instant::now();

    while (&mut floor2).next() < 400000 {}

    let c2 = floor2.safe_counter;

    let after2 = Instant::now();
    println!(
        "Part 2: {}, in {:?}",
        c2,
        after2.duration_since(start2)
    );
}

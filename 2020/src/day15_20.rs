use super::tools;
use std::time::Instant;
// use std::collections::HashMap;

// struct VanEck {
//     memory: HashMap<usize, usize>,
//     init: Vec<usize>,
//     last: usize,
//     turn: usize,
// }

// impl Iterator for VanEck {
//     type Item = usize;

//     fn next(&mut self) -> Option<usize> {
//         if self.turn < self.init.len() {
//             self.add(self.init[self.turn]);
//         } else {
//             let next;
//             if let Some(lt) = self.memory.get(&self.last) {
//                 next = self.turn - lt;
//             } else {
//                 next = 0;
//             }
//             self.add(next);
//         }
//         Some(self.last)
//     }
// }

// impl VanEck {
//     pub fn new(nums: &Vec<usize>) -> VanEck {
//         VanEck { last: 0, turn: 0, memory: HashMap::new(), init: nums.clone() }
//     }

//     fn add(&mut self, num: usize) {
//         if self.turn > 0 {
//             self.memory.insert(self.last, self.turn);
//             self.last = num;
//             self.turn += 1;
//         } else {
//             self.last = num;
//             self.turn = 1;
//         }
//     }
// }

#[allow(unused_must_use)]
pub fn get_number(numbers: &Vec<usize>, it: usize) -> usize {
    let mut memory = vec![0; it];
    let mut last: usize;

    last = numbers[0];
    for (i, &n) in numbers.iter().skip(1).enumerate() {
        memory[last] = i + 2;
        last = n;
    }

    let start: usize = numbers.len() + 1;
    let end: usize = it+1;
    for turn in start..end {
        let next: usize = match memory[last] {
            0 => 0,
            lt => turn - lt
        };
        memory[last] = turn;
        last = next;
    }

    last
}

#[allow(dead_code)]
pub fn run(real: bool, print_result: bool) -> (u128, u128, u128) {
    let start0 = Instant::now();

    let input_file: &str = if !real {
        "./input/day15_20_test.txt"
    } else {
        "./input/day15_20_real.txt"
    };
    let input = tools::get_input(String::from(input_file));
    let numbers: Vec<usize> = input[0]
        .split(',')
        .map(|s| s.parse::<usize>().unwrap())
        .collect();
    
    let after0 = Instant::now();

    let start1 = Instant::now();

    // let mut ve = VanEck::new(&numbers);
    // let res1 = ve.nth(2020-1).unwrap(); // turn is one more than n-th iteration
    let res1 = get_number(&numbers, 2020);

    let after1 = Instant::now();
    if print_result {
        println!("Part 1: {}", res1);
    }

    let start2 = Instant::now();

    // let mut ve2 = VanEck::new(&numbers);
    // let res2 = ve2.nth(30_000_000-1).unwrap(); // turn is one more than n-th iteration
    let res2 = get_number(&numbers, 30_000_000);

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

use super::tools;
use std::time::Instant;
use std::collections::HashMap;

#[allow(dead_code)]
pub fn run(real: bool, print_result: bool) -> (u128, u128, u128) {
    let start0 = Instant::now();

    let input_file: &str = if !real {
        "./input/day03_15_test.txt"
    } else {
        "./input/day03_15_real.txt"
    };
    let input = tools::get_input(String::from(input_file));

    let count = | persons: usize, moves: &String | -> usize {
        let mut counter: HashMap<(i32,i32),u32> = HashMap::new();
        let mut pos = vec![(0i32,0i32);persons];
        counter.insert(pos[0], persons as u32);
        let mut turn = 0usize;
        for c in moves.chars() {
            turn = (turn + 1) % persons;
            match c {
                '<' => { pos[turn].0 -= 1; },
                '>' => { pos[turn].0 += 1; },
                '^' => { pos[turn].1 -= 1; },
                'v' => { pos[turn].1 += 1; },
                _ => panic!("what!! where do I go??")
            }
            if let Some(x) = counter.get_mut(&pos[turn]) {
                *x += 1;
            } else {
                counter.insert(pos[turn], 1);
            }
        }
        counter.len()
    };

    let after0 = Instant::now();

    let start1 = Instant::now();

    let res1 = count(1, &input[0]);

    let after1 = Instant::now();
    if print_result {
        println!("Part 1: {}", res1);
    }

    let start2 = Instant::now();

    let res2 = count(2, &input[0]);

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

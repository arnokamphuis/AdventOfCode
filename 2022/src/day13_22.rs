use super::tools;
use std::time::Instant;

fn compare_vecs(v: &Vec<Vec<String>>) {
    println!("{:?}", v);
    
}

fn compare(lines: [&String;2]) -> bool {
    let mut list_depth = [0usize, 0usize];
    let mut index = [0usize, 0usize];
    let items = lines.iter().map(|l| l[1..l.len()-1].split(",").map(|s| s.to_string()).collect::<Vec<String>>()).collect::<Vec<_>>();

    compare_vecs(&items);
    println!("");
    true
}

#[allow(dead_code)]
pub fn run(real: bool, print_result: bool) -> (u128, u128, u128) {
    let start0 = Instant::now();

    let input_file: &str = if !real {
        "./input/day13_22_test.txt"
    } else {
        "./input/day13_22_real.txt"
    };
    let input = tools::get_input(String::from(input_file));

    let after0 = Instant::now();

    let start1 = Instant::now();

    let mut pair_count = 0;
    let nr_pairs = (input.len() as f32/3.0).ceil() as usize;
    let mut correct = 0;

    while pair_count < nr_pairs {
        if compare([&input[&pair_count * 3 + 0], &input[pair_count * 3 + 1]]) {
            correct += pair_count+1;
        }
        pair_count += 1;
    }
    
    let after1 = Instant::now();
    if print_result {
        println!("Part 1: {}", correct);
    }

    let start2 = Instant::now();

    let after2 = Instant::now();
    if print_result {
        println!("Part 2: {}", 0);
    }

    (
        after0.duration_since(start0).as_nanos(),
        after1.duration_since(start1).as_nanos(),
        after2.duration_since(start2).as_nanos(),
    )
}

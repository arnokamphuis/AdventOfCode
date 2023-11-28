use super::tools;
use std::time::Instant;

#[allow(dead_code)]
pub fn run(real: bool, print_result: bool) -> (u128, u128, u128) {
    let start0 = Instant::now();

    let input_file: &str = if !real {
        "./input/day06_22_test.txt"
    } else {
        "./input/day06_22_real.txt"
    };
    let input = tools::get_input(String::from(input_file));

    let processed_line = input[0].chars().map(|c| c as usize - 'a' as usize).collect::<Vec<usize>>();

    let different = | x: &[usize], l: usize, set: &mut Vec<usize> | -> bool {
        set[x[0]] -= 1;
        set[x[l]] += 1;
        set.iter().filter(|&&v| v >= 2).count() == 0
    };
    
    let find_different = | v: &Vec<usize>, l: usize | -> usize {
        let mut set = vec![0usize;26];
        v[0..l].iter().for_each(|&d| { set[d] += 1; });
        v.windows(l+1).position(|x| different(x, l, &mut set) ).unwrap() + l + 1
    };

    let after0 = Instant::now();

    let start1 = Instant::now();

    let res1 = find_different(&processed_line, 4);

    let after1 = Instant::now();
    if print_result {
        println!("Part 1: {}", res1);
    }

    let start2 = Instant::now();

    let res2 = find_different(&processed_line, 14);

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

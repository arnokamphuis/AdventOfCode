use super::tools;
use std::time::Instant;

#[allow(dead_code)]
pub fn run(real: bool, print_result: bool) -> (u128, u128, u128) {
    let start0 = Instant::now();

    let input_file: &str = if !real {
        "./input/day04_15_test.txt"
    } else {
        "./input/day04_15_real.txt"
    };
    let input = tools::get_input(String::from(input_file));

    let key = input[0].clone();

    let find = | mut num: u64, key: &String, zeros: usize | -> u64 {
        loop {
            let msg = &format!("{:x}", md5::compute(format!("{}{}",key,num).as_bytes()))[0..zeros];
            if let Ok(v) = msg.parse::<i32>() {
                if v == 0 {
                    return num;
                }
            }
            num += 1;
        }
    };

    let after0 = Instant::now();

    let start1 = Instant::now();

    let res1 = find(1, &key, 5);

    let after1 = Instant::now();
    if print_result {
        println!("Part 1: {}", res1);
    }

    let start2 = Instant::now();

    let res2 = find( res1, &key, 6);

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

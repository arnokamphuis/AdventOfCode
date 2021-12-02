use super::tools;
use std::time::Instant;

#[allow(dead_code)]
pub fn run(real: bool, print_result: bool) -> (u128, u128, u128) {
    let start0 = Instant::now();

    let input_file: &str = if !real {
        "./input/day02_21_test.txt"
    } else {
        "./input/day02_21_real.txt"
    };
    let input = tools::get_input(String::from(input_file));

    let after0 = Instant::now();

    let start1 = Instant::now();

    let endpos1 = input
        .iter()
        .map(|line| {
            let mut tokens = line.split_whitespace();
            let dir = tokens.next().unwrap();
            let amount: i64 = tokens.next().unwrap().parse().unwrap();
            (dir,amount)
        })
        .fold((0,0), |(x, d), (dir, amount)| -> (i64,i64) {
            match dir {
                "forward" => { (x+amount, d) },
                "down"    => { (x, d + amount) },
                "up"      => { (x, d - amount) },
                _ => panic!()
            }
        } );
    let res1 = endpos1.0 * endpos1.1;

    let after1 = Instant::now();
    if print_result {
        println!("Part 1: {}", res1);
    }

    let start2 = Instant::now();

    let endpos2 = input
        .iter()
        .map(|line| {
            let mut tokens = line.split_whitespace();
            let dir = tokens.next().unwrap();
            let amount: i64 = tokens.next().unwrap().parse().unwrap();
            (dir,amount)
        })
        .fold((0,0,0), |(x, d, a), (dir, amount)| -> (i64, i64, i64) {
            match dir {
                "forward" => { (x+amount, d+a*amount, a) },
                "down"    => { (x, d, a + amount) },
                "up"      => { (x, d, a - amount) },
                _ => panic!()
            }
        } );
    let res2 = endpos2.0 * endpos2.1;

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

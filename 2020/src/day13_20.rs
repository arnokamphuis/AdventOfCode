use super::tools;
use std::time::Instant;
use modinverse::modinverse;

fn chinese_remainder(constraints: &Vec<(i64,i64)>) -> Option<i64> {
    // product of the modulos
    let product = constraints.iter().map(|&v| v.1).product::<i64>(); 
 
    // x = SUM a_i M_i N_i, where a_i is the remainder, N_i is p as described below, and M_i should be the modular inverse
    // Bezout theorem: find integers u_i and v_i such as u_i n_i + v_i ^n_i = 1. Here, v_i is the modular inverse of ^n_i modulo n_i.
    // See https://www.dcode.fr/chinese-remainder for a more elaborate explanation
    let mut x = 0;
    for (remainder, modulus) in constraints {
        // p is product but one, a stated in the wiki page as N_i = N/n_i
        let p: i64 = product / modulus;
        x += remainder * modinverse(p, *modulus)? * p
    }
    // return the remainder of the sum by the product
    Some(x % product)
}

#[allow(dead_code)]
pub fn run(real: bool, print_result: bool) -> (u128, u128, u128) {
    let start0 = Instant::now();

    let input_file: &str = if !real {
        "./input/day13_20_test.txt"
    } else {
        "./input/day13_20_real.txt"
    };
    let input = tools::get_input(String::from(input_file));

    let departure_time = input[0].parse::<usize>().unwrap();
    let numbers = input[1].split(',').map(|s| if let Ok(v) = s.parse::<usize>() {v} else {0}).filter(|&v| v > 0).collect::<Vec<usize>>();

    // Create remainders and modulos for Chinese Remainder Theory 
    //   see https://en.wikipedia.org/wiki/Chinese_remainder_theorem
    let constraints = input[1].split(',').enumerate()
        .map(|s| if let Ok(v) = s.1.parse::<i64>() {
            ( (v-(s.0 as i64 % v))%v , v ) 
        } else {
            (-1,-1)
        }).filter(|&v| v.0 > -1 && v.1 > -1)
        .map(|v| (v.0 as i64, v.1 as i64)).collect::<Vec<(i64,i64)>>();

    let after0 = Instant::now();

    let start1 = Instant::now();

    let res1 = numbers.iter()
        .map(|&v| (v, v - departure_time % v) )
        .fold((0,1000), |acc, v| if v.1 < acc.1 { v } else { acc });

    let after1 = Instant::now();
    if print_result {
        println!("Part 1: {:?}", res1.0 * res1.1);
    }

    let start2 = Instant::now();

    let mut res2 = 0;
    if let Some(cr) = chinese_remainder(&constraints) {
        res2 = cr;
    }

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

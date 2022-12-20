use super::tools;
use std::time::Instant;
use std::collections::VecDeque;

fn hussle(original: &Vec<i64>, cycles: usize) -> i64 {
    let n = original.len();
    let mut dq = original
        .iter()
        .enumerate()
        .map(|(i,v)| (i as i64, *v))
        .collect::<VecDeque<_>>();

    for _ in 0..cycles {
        for i in 0i64..(n as i64) {
            for j in 0..n {
                if dq[j].0 == i {
                    break;
                }
            }

            while dq[0].0 != i {
                let v = dq.pop_front().unwrap();
                dq.push_back(v)
            }
            let val = dq.pop_front().unwrap();
            let to_pop = val.1.rem_euclid(dq.len() as i64);

            for _ in 0..to_pop {
                let v = dq.pop_front().unwrap();
                dq.push_back(v)
            }
            dq.push_back(val);
        }
    }
    let j = dq.iter().position(|&(_,v)| v==0).unwrap();
    dq[(j+1000)%n].1 + dq[(j+2000)%n].1 + dq[(j+3000)%n].1
}

#[allow(dead_code)]
pub fn run(real: bool, print_result: bool) -> (u128, u128, u128) {
    let start0 = Instant::now();

    let input_file: &str = if !real {
        "./input/day20_22_test.txt"
    } else {
        "./input/day20_22_real.txt"
    };
    let input = tools::get_input(String::from(input_file));
    let original = input.iter().map(|line| line.parse::<i64>().unwrap()).collect::<Vec<i64>>();
    let n = original.len();

    let mut dq = original.iter().enumerate().map(|(i,v)| (i as i64, *v)).collect::<VecDeque<_>>();

    let after0 = Instant::now();

    let start1 = Instant::now();

    let res1 = hussle(&original, 1);

    let after1 = Instant::now();
    if print_result {
        println!("Part 1: {}", res1);
    }

    let start2 = Instant::now();

    let res2 = hussle( & original.iter().map(|v| v * 811589153).collect::<Vec<i64>>(), 10);

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

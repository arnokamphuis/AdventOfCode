use super::tools;
use std::time::Instant;

#[allow(dead_code)]
pub fn run(real: bool, print_result: bool) -> (u128, u128, u128) {
    let start0 = Instant::now();

    let input_file: &str = if !real { "./input/day01_20_test.txt" } else { "./input/day01_20_real.txt" };
    let input = tools::get_input(String::from(input_file));

    let after0 = Instant::now();

    let start1 = Instant::now();

    let numbers: Vec<i64> = (&input).into_iter().map(|n| n.parse::<i64>().unwrap()).collect();

    let res1 = numbers
        .iter()
        .enumerate()
        .map(|(i,n1)| {
            numbers
                .iter()
                .skip(i)
                .filter(|n2| *n1 + *n2 == 2020)
                .map( move |n2| *n1 * *n2 )
                .fold(0, |t, v| t+v)
        })
        .fold(0, |t, v| t+v);
        
    // let mut res1 = 0;
    // for item1 in &numbers {
    //     for item2 in &numbers {
    //         if item1+item2==2020 {
    //             res1 = item1*item2;
    //         }
    //     }
    // }

    let after1 = Instant::now();
    if print_result { println!("Part 1: {:?}", res1); }

    let start2 = Instant::now();

    let res2 = numbers
        .iter()
        .enumerate()
        .map(|(i,n1)| {
            numbers
                .iter()
                .skip(i)
                .enumerate()
                .map(|(j,n2)| {
                    numbers
                        .iter()
                        .skip(i+j)
                        .filter(|n3| *n1 + *n2 + *n3 == 2020)
                        .map( move |n3| *n1 * *n2 * *n3 )
                        .fold(0, |t, v| t+v)
                    })
                .fold(0, |t, v| t+v)
        })
        .fold(0, |t, v| t+v);

    // let mut res2 = 0;
    // for item1 in &numbers {
    //     for item2 in &numbers {
    //         for item3 in &numbers {
    //             if item1+item2+item3==2020 {
    //                 res2 = item1*item2*item3;
    //             }
    //         }
    //     }
    // }

    let after2 = Instant::now();
    if print_result { println!("Part 2: {}", res2); }

    (after0.duration_since(start0).as_nanos(), after1.duration_since(start1).as_nanos(), after2.duration_since(start2).as_nanos())
}

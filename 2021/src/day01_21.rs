use super::tools;
// needed to make image
// use super::tools::Image;
use std::time::Instant;

#[allow(dead_code)]
pub fn run(real: bool, print_result: bool) -> (u128, u128, u128) {
    let start0 = Instant::now();

    let input_file: &str = if !real {
        "./input/day01_21_test.txt"
    } else {
        "./input/day01_21_real.txt"
    };
    let input = tools::get_input(String::from(input_file));

    let numbers: Vec<i64> = (&input)
        .into_iter()
        .map(|n| n.parse::<i64>().unwrap())
        .collect();
    
    // needed to make image
    // let depth = *numbers.iter().max().unwrap() as usize;
    // let t = numbers.len();

    let after0 = Instant::now();

    let start1 = Instant::now();

    let res1 = numbers.windows(2).filter(|v| v[1] > v[0]).count();

    let after1 = Instant::now();
    if print_result {
        println!("Part 1: {}", res1);
    }

    let start2 = Instant::now();

    let res2 = numbers
        .windows(3)
        .map(|v| v.iter().sum())
        .collect::<Vec<i64>>()
        .windows(2)
        .filter(|n| n[1] > n[0])
        .count();

    let after2 = Instant::now();
    if print_result {
        println!("Part 2: {}", res2);
    }

    // needed to make image
    // let s = 8;
    // let mut img = Image::new(t/s, depth/s+1, s);
    // img.clear((255,255,255,255));
    //
    // numbers
    //     .iter()
    //     .enumerate()
    //     .for_each(|(i,n)| {
    //         img.set_pixel(i/s,(*n) as usize / s,(0,0,0,255))
    //     });
    //
    // numbers
    //     .windows(3)
    //     .map(|v| (v[0] + v[1] + v[2])/3)
    //     .collect::<Vec<i64>>()
    //     .iter()
    //     .enumerate()
    //     .for_each(|(i,n)| {
    //         img.set_pixel(i/s,(*n) as usize / s,(255,0,0,255))
    //     });

    // img.save_png(&String::from("images/day01_2021.png"));


    (
        after0.duration_since(start0).as_nanos(),
        after1.duration_since(start1).as_nanos(),
        after2.duration_since(start2).as_nanos(),
    )
}

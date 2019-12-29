use super::tools;
use std::time::Instant;

fn process(inp_nums: &Vec<i64>) -> Vec<i64> {
    let index_base = [0, 1, 0, -1];
    let mut out_nums: Vec<i64> = vec![];
    for (i, _) in inp_nums.iter().enumerate() {
        let mut next_num = 0;

        for (j, m) in inp_nums.iter().enumerate() {
            let index = ((j + 1) / (i + 1)) % index_base.len();
            let factor = index_base[index];
            next_num += factor * m;
        }
        out_nums.push((next_num % 10).abs());
    }
    out_nums.clone()
}

#[allow(dead_code)]
pub fn run() {
    println!("Day 16 of 2019");

    let start0 = Instant::now();

    // let input_file = "./input/day16_19_test.txt";
    let input_file = "./input/day16_19_real.txt";
    let input = tools::get_input(String::from(input_file));
    let mut inp_nums: Vec<i64> = input[0]
        .chars()
        .map(|s| s.to_digit(10).unwrap() as i64)
        .collect();

    let inp_orig = inp_nums.clone();

    let after0 = Instant::now();
    println!("Init in {:?}", after0.duration_since(start0));

    let start1 = Instant::now();

    for _ in 0..100 {
        inp_nums = process(&inp_nums);

        // trying to find out what happens to the back
        //    with large sequences the [0,1,0,-1] the last part
        //    will only use [0,0,0,0,0,0,...,1,1,1,1,1,1,1,1]
        // thus, we only need to use the back, and reverse it

        // println!(
        //     "{}",
        //     &inp_nums
        //         .clone()
        //         .into_iter()
        //         .rev()
        //         .take(10)
        //         .rev()
        //         .map(|i| i.to_string())
        //         .collect::<String>()
        // );
    }

    let res1: String = inp_nums
        .iter()
        .map(|i| i.to_string())
        .take(8)
        .collect::<String>();

    let after1 = Instant::now();
    println!("Part 1: {}, in {:?}", res1, after1.duration_since(start1));

    let start2 = Instant::now();

    let offset = (String::from(&input[0])[..7]).parse::<usize>().unwrap();

    let len = inp_orig.len();
    let full_len = len * 10000;
    let phase_len = full_len - offset;

    let mut inp_nums2: Vec<i64> = (offset..full_len)
        .map(|i| inp_orig[i % len])
        .rev()
        .collect();

    for _ in 0..100 {
        // the digits are in reverse, therefore we can go from the front to the back
        // adding the sumbs (see print of number during part 1).
        for i in 1..phase_len {
            inp_nums2[i] += inp_nums2[i - 1];
            inp_nums2[i] %= 10;
        }
    }

    let res2: String = inp_nums2
        .iter()
        .rev()
        .map(|i| i.to_string())
        .take(8)
        .collect::<String>();

    let after2 = Instant::now();
    println!("Part 2: {}, in {:?}", res2, after2.duration_since(start2));
}

use super::tools;
use std::time::Instant;

#[allow(dead_code)]
pub fn run(real: bool, print_result: bool) -> (u128, u128, u128) {
    let start0 = Instant::now();

    let input_file: &str = if !real {
        "./input/day03_21_test.txt"
    } else {
        "./input/day03_21_real.txt"
    };
    let input = tools::get_input(String::from(input_file));

    let count_zeros = |nums: &Vec<u64>, nbs: usize| -> Vec<usize> {
        (0..nbs)
            .map(|i| 
                nums
                    .iter()
                    .fold(0, |zcs, &v| {
                        zcs + ((v & 1<<(nbs-1-i)) == 0) as usize
                }))
            .collect::<Vec<usize>>()
    };

    let mut bits = 0;
    let mut numbers : Vec<u64> = vec![];
    for line in &input {
        bits = line.len();
        numbers.push(line.chars().fold(0, |v, c| (v<<1) + (c=='1') as u64));
    }
    let nc = numbers.len() >> 1;

    let bw_not = | v:u64 | -> u64 { (!v) & ((1<<bits)-1) };

    let after0 = Instant::now();

    let start1 = Instant::now();

    let gamma = count_zeros(&numbers, bits)
        .iter()
        .fold(0, |g, &zero| {
            (g << 1) + (zero < nc) as u64
        });

    let res1 = gamma * bw_not(gamma);

    let after1 = Instant::now();
    if print_result {
        println!("Part 1: {}", res1);
    }

    let start2 = Instant::now();

    let most_ones  = |v: u64, bit: usize, bits: usize, nc: usize, zeros: &Vec<usize>| -> bool {
        !(((v & (1 << (bits-1-bit)) as u64) > 0) ^ (zeros[bit] <= nc))
    };

    let filter_numbers = | mut nums: Vec<u64>, inv: bool | -> u64 {
        let mut cb = 0;
        while nums.len() > 1 {
            let zeros = count_zeros(&nums, bits);
            let nc = nums.len()>>1;
            nums.retain(|&v| most_ones(v,cb,bits,nc,&zeros) ^ inv);
            cb += 1;
        };
        nums[0]
    };

    let res2 = filter_numbers(numbers.clone(), true) * filter_numbers(numbers.clone(), false);

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

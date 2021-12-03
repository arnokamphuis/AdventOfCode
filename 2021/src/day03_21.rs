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

    let count = |nums: &Vec<u64>, nbs: usize| -> (Vec<usize>,Vec<usize>) {
        let mut ocs: Vec<usize> = vec![0;nbs];
        let mut zcs: Vec<usize> = vec![0;nbs];
        nums
            .iter()
            .for_each(|&v| {
                (0..nbs).for_each(|i| {
                    ocs[i] += ((v & 1<<(nbs-1-i)) != 0) as usize;
                    zcs[i] += ((v & 1<<(nbs-1-i)) == 0) as usize;
                });
            });
        (zcs,ocs)
    };

    let mut bits = 0;
    let mut numbers : Vec<u64> = vec![];
    for line in &input {
        bits = line.len();
        numbers.push(line.chars().fold(0, |v, c| (v<<1) + (c=='1') as u64));
    }

    let after0 = Instant::now();

    let start1 = Instant::now();

    let (zeros, ones) = count(&numbers, bits);
    let res1 = zeros.iter().zip(ones.iter()).fold([0;2],|[gamma, epsilon], (&zero, &one)| {
        [(gamma << 1) + (zero < one) as u64, (epsilon << 1) + (zero > one) as u64 ]
    }).iter().fold(1, |res, v| res * v);

    let after1 = Instant::now();
    if print_result {
        println!("Part 1: {}", res1);
    }

    let start2 = Instant::now();

    let most_ones  = |v: u64, bit: usize, revbit: usize, zeros: &Vec<usize>, ones: &Vec<usize>| -> bool {
        let one = (v & (1 << revbit) as u64) > 0; 
        (one && (ones[bit] >= zeros[bit])) || (!one && (ones[bit] < zeros[bit]))
    };

    let least_ones = |v: u64, bit: usize, revbit: usize, zeros: &Vec<usize>, ones: &Vec<usize>| -> bool {
        let one = (v & (1 << revbit) as u64) > 0; 
        (one && (ones[bit] < zeros[bit])) || (!one && (ones[bit] >= zeros[bit]))
    };

    let filter_numbers = | mut nums: Vec<u64>, condition:  &dyn Fn(u64, usize, usize, &Vec<usize>, &Vec<usize>) -> bool | -> u64 {
        let mut cb = 0;
        let mut rb = bits-1-cb;
        while nums.len() > 1 {
            let (zeros, ones) = count(&nums, bits);
            nums = nums
                .iter()
                .filter(|&&v| {
                    condition(v, cb, rb, &zeros, &ones)
                })
                .map(|&v| v)
                .collect();
            if nums.len()>1 {
                cb += 1;
                rb -= 1;
            }
        };
        nums[0]
    };

    let res2 = filter_numbers(numbers.clone(), &most_ones) * filter_numbers(numbers.clone(), &least_ones);

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

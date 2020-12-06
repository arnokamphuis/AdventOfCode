use super::tools;
use std::time::Instant;

#[allow(dead_code)]
pub fn run(real: bool) {
    println!("Day 02 of 2020");

    let start0 = Instant::now();

    let input_file: &str = if !real { "./input/day02_20_test.txt" } else { "./input/day02_20_real.txt" };
    let input: Vec<String> = tools::get_input(String::from(input_file));

    let after0 = Instant::now();
    println!("Init in {:?}", after0.duration_since(start0));

    let start1 = Instant::now();

    let mut res1 = 0;
    let mut res2 = 0;
    for line in &input {
        // split on the :
        let pieces: Vec<&str> = line.split(':').collect();
        // split first part on space
        let ra: Vec<&str> = (&pieces[0]).split(' ').collect();
        // get the character which is in consideration
        let find_c = (&ra[1]).chars().next().unwrap();
        // get the range integers (low high)
        let range: Vec<usize> = (&ra[0])
            .split('-')
            .map(|v| v.parse::<usize>().unwrap())
            .collect();

        // get the count for the specific character
        let count: usize = (&pieces[1]).chars().filter(|c| *c == find_c).count();

        // check part 1 condition
        if range[0] <= count && count <= range[1] {
            res1 += 1;
        }

        // check part 2 condition
        let c1 = (&pieces[1]).chars().nth(range[0]).unwrap();
        let c2 = (&pieces[1]).chars().nth(range[1]).unwrap();
        if (c1 == find_c) ^ (c2 == find_c) {
            res2 += 1;
        }
    }

    let after1 = Instant::now();
    println!("Part 1: {}, in {:?}", res1, after1.duration_since(start1));

    let start2 = Instant::now();
    let after2 = Instant::now();
    println!("Part 2: {}, in {:?}", res2, after2.duration_since(start2));
}

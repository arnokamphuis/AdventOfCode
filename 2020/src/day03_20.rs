use super::tools;
use std::collections::BTreeMap;
use std::time::Instant;

pub fn count(
    grid: &BTreeMap<(usize, usize), bool>,
    size: (usize, usize),
    direction: (usize, usize),
) -> usize {
    let mut res: usize = 0;
    let mut coor: (usize, usize) = (0, 0);
    while coor.1 < size.1 {
        coor.0 += direction.0;
        coor.0 %= size.0;

        coor.1 += direction.1;

        if let Some(empty) = grid.get(&coor) {
            if !*empty {
                res += 1;
            }
        }
    }
    res
}

#[allow(dead_code)]
pub fn run() {
    println!("Day 03 of 2020");

    let start0 = Instant::now();

    // let input_file: &str = "./input/day03_20_test.txt";
    let input_file: &str = "./input/day03_20_real.txt";
    let input: Vec<String> = tools::get_input(String::from(input_file));

    let mut grid: BTreeMap<(usize, usize), bool> = BTreeMap::new();
    let mut size: (usize, usize) = (0, 0);
    for line in &input {
        size.0 = 0;
        for c in line.chars().into_iter() {
            grid.insert(size, c != '#');
            size.0 += 1;
        }
        size.1 += 1;
    }

    let after0 = Instant::now();
    println!("Init in {:?}", after0.duration_since(start0));

    let start1 = Instant::now();
    let res1 = count(&grid, size, (3, 1));

    let after1 = Instant::now();
    println!("Part 1: {}, in {:?}", res1, after1.duration_since(start1));

    let start2 = Instant::now();

    let mut res2 = 1;
    res2 *= count(&grid, size, (1, 1));
    res2 *= count(&grid, size, (3, 1));
    res2 *= count(&grid, size, (5, 1));
    res2 *= count(&grid, size, (7, 1));
    res2 *= count(&grid, size, (1, 2));

    let after2 = Instant::now();
    println!("Part 2: {}, in {:?}", res2, after2.duration_since(start2));
}

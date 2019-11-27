use std::time::{Instant};

fn main() {
    println!("Day 1 of 2019");

    let start1 = Instant::now();

    let after1 = Instant::now();
    println!("Part 1: {}, in {:?}", 0, after1.duration_since(start1));

    let start2 = Instant::now();

    let after2 = Instant::now();
    println!("Part 2: {}, in {:?}", 0, after2.duration_since(start2));
}

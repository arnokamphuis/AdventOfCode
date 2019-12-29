use std::time::Instant;
use super::tools;
use interval::interval_set::*;
use gcollections::ops::*;
use std::borrow::Cow;

#[allow(dead_code)]
pub fn run() {
    println!("Day 20 of 2016");

    let start0 = Instant::now();
    // let input_file = "./input/day20_16_test.txt";
    let input_file = "./input/day20_16_real.txt";
    let input = tools::get_input(String::from(input_file));

    let after0 = Instant::now();
    println!(
        "Init in {:?}",
        after0.duration_since(start0)
    );

    let start1 = Instant::now();    

    let mut intervals: Vec<IntervalSet<u32>> = vec![];
    input.iter().for_each(|line| {
        let mut iter = line.split("-").filter_map(|s| s.parse::<u32>().ok());
        if let Some(begin) = iter.next() {
            if let Some(end) = iter.next() {
                intervals.push(vec![(begin,end)].to_interval_set());
            }
        }
    });
    
    let universe: IntervalSet<u32> = vec![(0,std::u32::MAX-1)].to_interval_set();

    let result = &(intervals[0]);
    let mut result = Cow::Borrowed(result);

    intervals[1..].into_iter().for_each(|inter| {
        let temp = result.union(inter);
        result = Cow::Owned(temp);
    });

    let found = result.into_owned();
    
    let result = universe.difference(&found);

    let mut iter = (&result).into_iter();
    let lowest = iter.next().unwrap().lower();
    
    let after1 = Instant::now();
    println!(
        "Part 1: {}, in {:?}",
        lowest,
        after1.duration_since(start1)
    );

    let start2 = Instant::now();

    let mut total = 0;
    for r in (&result).into_iter() {
        let diff = r.upper() - r.lower() + 1;
        if diff == 1 {
            total += 1;
        }
    }

    let after2 = Instant::now();
    println!(
        "Part 2: {}, in {:?}",
        total,
        after2.duration_since(start2)
    );
}

use super::tools;
use std::time::Instant;
use itertools::Itertools;
use std::collections::HashSet;

fn map_update(map: &mut HashSet::<(i32,i32)>, time: usize, byte_locations: &Vec<(i32,i32)>) {
    let (x,y) = byte_locations[time-1];
    map.remove(&(x,y));
}

fn map_after_ns(map: &mut HashSet::<(i32,i32)>, time: usize, byte_locations: &Vec<(i32,i32)>, size: (i32,i32)) {
    for x in 0..size.0 {
        for y in 0..size.1 {
            map.insert((x,y));
        }
    }
    for t in 1..=time {
        map_update(map, t, byte_locations);
    }
}

fn find_path(map: &HashSet::<(i32,i32)>, start: (i32,i32), end: (i32,i32)) -> i32 {
    let mut current = start;
    let mut distance;
    let mut queue = Vec::<((i32,i32),i32)>::new();
    let mut seen: HashSet::<(i32,i32)> = HashSet::new();
    queue.push((current, 0));

    while !queue.is_empty() {
        (current, distance) = queue.remove(0);
        if seen.contains(&current) {
            continue;
        }
        seen.insert(current);
        if current == end {
            return distance;
        }
        let neighbours = vec![(current.0-1,current.1),(current.0+1,current.1),(current.0,current.1-1),(current.0,current.1+1)];
        for neighbour in neighbours {
            if map.contains(&neighbour) {
                queue.push((neighbour, distance+1));
            }
        }
    }

    i32::MAX
}

#[allow(dead_code)]
fn print_map(map: &HashSet::<(i32,i32)>, size: (i32,i32)) {
    for y in 0..size.1 {
        for x in 0..size.0 {
            if map.contains(&(x,y)) {
                print!(".");
            } else {
                print!("#");
            }
        }
        println!();
    }
}

#[allow(dead_code)]
pub fn run(real: bool, print_result: bool) -> (u128, u128, u128) {
    let start0 = Instant::now();

    let input_file: &str = if !real {
        "./input/day18-test.txt"
    } else {
        "./input/day18-real.txt"
    };
    let input = tools::get_input(String::from(input_file));
    let byte_locations = input.iter()
        .map(|x| x.split(","))
        .map(|x| x.map(|y| y.parse::<i32>().unwrap()).collect::<Vec<i32>>())
        .map(|x| x.iter().map(|v| *v).collect_tuple().unwrap())
        .collect::<Vec<(i32, i32)>>();

    let size = if !real { (7,7) } else { (71,71) };
    let last_time = if !real { 12 } else { 1024 };

    let after0 = Instant::now();

    let start1 = Instant::now();

    let mut map = HashSet::<(i32,i32)>::new();
    map_after_ns(&mut map, last_time, &byte_locations, size);

    let res1 = find_path(&map, (0,0), (size.0-1,size.1-1));

    let after1 = Instant::now();
    if print_result {
        println!("Part 1: {}", res1);
    }

    let start2 = Instant::now();

    let mut map = HashSet::<(i32,i32)>::new();
    
    let mut search_range = (0_i32, byte_locations.len() as i32);
    while search_range.1 - search_range.0 > 1 {
        let t = (search_range.0 + search_range.1) / 2;
        map_after_ns(&mut map, t as usize, &byte_locations, size);
        if find_path(&map, (0,0), (size.0-1,size.1-1)) != i32::MAX {
            search_range.0 = t;
        } else {
            search_range.1 = t;
        }
    }
    let byte_index = search_range.0 as usize;
    let byte = byte_locations[byte_index];

    let res2 = format!("{},{}", byte.0, byte.1);

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

use super::tools;
use std::time::Instant;
use itertools::Itertools;
use std::collections::{HashSet, HashMap};

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

fn gen_path(backtrack: &HashMap::<(i32,i32),(i32,i32)>, end: (i32,i32)) -> Vec<(i32,i32)> {
    let mut path = Vec::<(i32,i32)>::new();
    let mut current = end;
    while backtrack.contains_key(&current) {
        path.push(current);
        current = backtrack[&current];
    }
    path.push(current);
    path.reverse();
    path
}

fn find_path(map: &HashSet::<(i32,i32)>, start: (i32,i32), end: (i32,i32)) -> Option<Vec<(i32,i32)>> {
    let mut current = start;
    let mut queue = Vec::<(i32,i32)>::new();
    let mut backtrack = HashMap::<(i32,i32),(i32,i32)>::new();
    let mut distances = HashMap::<(i32,i32),i32>::new();
    queue.push(current);
    distances.insert(current, 0);

    while !queue.is_empty() {
        current = queue.remove(0);
        if current == end {
            return Some(gen_path(&backtrack, end));
        }
        let distance = distances[&current];
        let neighbours = vec![(current.0-1,current.1),(current.0+1,current.1),(current.0,current.1-1),(current.0,current.1+1)];
        for neighbour in neighbours {
            if map.contains(&neighbour) {
                if distance + 1 < *distances.get(&neighbour).unwrap_or(&i32::MAX) {
                    distances.insert(neighbour, distance+1);
                    queue.push(neighbour);
                    backtrack.insert(neighbour, current);
                }
            }
        }
    }

    None
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

    let mut res1 = 0;
    if let Some(path) = find_path(&map, (0,0), (size.0-1,size.1-1)) {
        res1 = path.len() - 1;
    }

    let after1 = Instant::now();
    if print_result {
        println!("Part 1: {}", res1);
    }

    let start2 = Instant::now();

    let mut t = 0;

    let mut map = HashSet::<(i32,i32)>::new();
    map_after_ns(&mut map, t, &byte_locations, size);
    let mut path = find_path(&map, (0,0), (size.0-1,size.1-1));

    while path != None {
        t += 1;
        map_update(&mut map, t, &byte_locations);
        if path.clone().unwrap().contains(&byte_locations[t-1]) {
            path = find_path(&map, (0,0), (size.0-1,size.1-1));
        }
    }
    let res2 = format!("{},{}", byte_locations[t-1].0, byte_locations[t-1].1);

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

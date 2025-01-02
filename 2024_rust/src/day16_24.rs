use super::tools;
use std::time::Instant;
use std::collections::{HashSet, HashMap};
use priority_queue::PriorityQueue;
use std::cmp::Reverse;

fn neighbors(map: &HashSet<(i64, i64)>, pos: (i64, i64), dir: (i64,i64)) -> Vec<((i64, i64), (i64, i64), i64)> {
    let mut res = Vec::new();
    let dirs = vec![(1,0), (0,1), (-1,0), (0,-1)];

    let d = dirs.iter().position(|&x| x == dir).unwrap() as i64;
    [-1_i64,1_i64].iter().for_each(|&i| {
        let new_dir = dirs[(d + i).rem_euclid(4) as usize];
        res.push((pos, new_dir, 1000));
    });

    let new_pos = (pos.0 + dir.0, pos.1 + dir.1);
    if map.contains(&new_pos) {
        res.push((new_pos, dir, 1));
    }

    res
}

fn search(map: &HashSet<(i64, i64)>, start: (i64, i64), start_dir: (i64, i64), end: (i64, i64), part: u8) -> i64 {
    let mut lowest_cost: HashMap<((i64,i64),(i64,i64)), i64> = HashMap::new();
    lowest_cost.insert((start, start_dir), 0);
    let mut backtrack: HashMap<((i64,i64),(i64,i64)), HashSet<((i64,i64),(i64,i64))>> = HashMap::new();
    let mut best_cost = i64::MAX;
    let mut end_states: HashSet<((i64,i64),(i64,i64))> = HashSet::new();

    let mut pq = PriorityQueue::new();
    pq.push((start, start_dir, 0), Reverse(0));
    while !pq.is_empty() {
        let ((pos, dir, c), _) = pq.pop().unwrap();
        if c <= *lowest_cost.get(&(pos, dir)).unwrap_or(&i64::MAX) {
            if pos == end {
                if c > best_cost {
                    continue;
                }
                best_cost = c;
                end_states.insert((pos, dir));
            }

            for (p, d, ec) in neighbors(map, pos, dir).iter(){
                if map.contains(p) {
                    let lowest = *lowest_cost.get(&(*p, *d)).unwrap_or(&i64::MAX);
                    if c + ec > lowest {
                        continue;
                    } 
                    if c + ec < lowest {
                        backtrack.insert((*p, *d), HashSet::new());
                        *lowest_cost.entry((*p, *d)).or_insert(c + ec) = c + ec;
                    }
                    backtrack.get_mut(&(*p, *d)).unwrap().insert((pos, dir));
                    pq.push((*p, *d, c + ec), Reverse(c + ec));
                }
            }
        }
    }

    if part == 1 {
        return best_cost;
    }

    let mut states: Vec<((i64,i64),(i64,i64))> = end_states.iter().map(|x| *x).collect();
    let mut visited_positions: HashSet<((i64,i64),(i64,i64))> = end_states.iter().fold( HashSet::<((i64,i64),(i64,i64))>::new(), |mut acc, x| { acc.insert(*x); acc });
    while states.len() > 0 {
        let (pos, dir) = states.pop().unwrap();
        for (p, d) in backtrack.get(&(pos, dir)).unwrap_or(&HashSet::new()).iter() {
            if visited_positions.contains(&(*p, *d)) {
                continue;
            }
            visited_positions.insert((*p, *d));
            states.push((*p, *d));
        }
    } 
    
    visited_positions
        .iter()
        .map(|(p, _)| p)
        .fold(HashSet::<(i64,i64)>::new(), |mut acc, x| { acc.insert(*x); acc })
        .into_iter()
        .count() as i64
}

#[allow(dead_code)]
pub fn run(real: bool, print_result: bool) -> (u128, u128, u128) {
    let start0 = Instant::now();

    let input_file: &str = if !real {
        "./input/day16-test.txt"
    } else {
        "./input/day16-real.txt"
    };
    let input = tools::get_input(String::from(input_file));

    let mut start = (-1,-1);
    let mut end = (-1,-1);
    let start_dir = (1,0);
    let mut map: HashSet<(i64, i64)> = HashSet::new();
    input.iter().enumerate().for_each(|(y, line)| {
        line.chars().enumerate().for_each(|(x, c)| {
            if c != '#' {
                map.insert((x as i64, y as i64));
            }
            if c == 'E' {
                end = (x as i64, y as i64);
            }
            if c == 'S' {
                start = (x as i64, y as i64);
            }
        });
    });

    // for y in 0..input.len() {
    //     for x in 0..input[y].len() {
    //         if !map.contains(&(x as i64, y as i64)) {
    //             print!("#");
    //         } else {
    //             print!(" ");
    //         }
    //     }
    //     println!();
    // }

    let after0 = Instant::now();

    let start1 = Instant::now();

    let res1 = search(&map, start, start_dir, end, 1);

    let after1 = Instant::now();
    if print_result {
        println!("Part 1: {}", res1);
    }

    let start2 = Instant::now();

    let res2 = search(&map, start, start_dir, end, 2);

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

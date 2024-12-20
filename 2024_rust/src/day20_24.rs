use super::tools;
use std::time::Instant;
use std::collections::{HashSet, HashMap};
use counter::Counter;

fn get_neighbors(pos: (i32,i32), map: &HashSet<(i32,i32)>) -> Vec<(i32,i32)> {
    let mut neighbors: Vec<(i32,i32)> = Vec::new();
    let (x,y) = pos;
    for (dx,dy) in [(-1,0),(1,0),(0,-1),(0,1)].iter() {
        let new_pos = (x+dx, y+dy);
        if map.contains(&new_pos) {
            neighbors.push(new_pos);
        }
    }
    neighbors
}

fn get_path(start: (i32,i32), end:(i32,i32), map: &HashSet<(i32,i32)>) -> (Vec<(i32,i32)>, HashMap<(i32,i32), usize>) {
    let mut path: Vec<(i32,i32)> = Vec::new();
    let mut distances: HashMap<(i32,i32), usize> = HashMap::new();
    let mut current = start;
    distances.insert(start, 0);
    while current != end {
        path.push(current);
        let neighbors = get_neighbors(current, map);
        for neighbor in neighbors {
            if !distances.contains_key(&neighbor) {
                distances.insert(neighbor, distances[&current] + 1);
                current = neighbor;
            }
        }
    }
    distances.insert(end, distances[path.last().unwrap()] + 1);
    path.push(end);
    return (path, distances)
}

fn get_all_cheats(path: &Vec<(i32,i32)>, distances: &HashMap<(i32,i32), usize>, max_cheats: usize) -> Vec<usize> {
    let mut cheats: Vec<usize> = Vec::new();

    for (p_index, p) in path.iter().enumerate() {
        let dist_p = distances[p];
        if p_index < path.len() - 1 {
            for q in path[p_index+2..].iter() {
                let dist_q = distances[q];
                let delta = dist_q - dist_p;
                let dist = ((p.0 - q.0).abs() + (p.1 - q.1).abs()) as usize;
                if delta > dist && dist <= max_cheats{
                    let saving = delta - dist as usize;
                    cheats.push(saving);
                }
            }
        }
    }
    return cheats;
}

fn savings(path: &Vec<(i32,i32)>, distances: &HashMap<(i32,i32), usize>, part: u8) -> usize {
    get_all_cheats(path, distances, if part == 1 { 2 } else { 20 })
        .iter()
        .map(|v| *v)
        .collect::<Counter<usize, usize>>()
        .iter()
        .filter(|(k,_)| **k >= 100)
        .map(|(_,v)| *v)
        .sum()
}

#[allow(dead_code)]
pub fn run(real: bool, print_result: bool) -> (u128, u128, u128) {
    let start0 = Instant::now();

    let input_file: &str = if !real {
        "./input/day20-test.txt"
    } else {
        "./input/day20-real.txt"
    };
    let input = tools::get_input(String::from(input_file));
    let mut map: HashSet<(i32,i32)> = HashSet::new();
    let mut start: (i32,i32) = (0,0);
    let mut end: (i32,i32) = (0,0);
    input.iter().enumerate().for_each(|(y, line)| {
        line.chars().enumerate().for_each(|(x, c)| {
            if c != '#' {
                map.insert((x as i32, y as i32));
            }
            if c == 'S' {
                start = (x as i32, y as i32);
            }
            if c == 'E' {
                end = (x as i32, y as i32);                
            }
        });
    });

    let (path, distances) = get_path(start, end, &map);

    let after0 = Instant::now();

    let start1 = Instant::now();

    let res1 = savings(&path, &distances, 1);

    let after1 = Instant::now();
    if print_result {
        println!("Part 1: {}", res1);
    }

    let start2 = Instant::now();

    let res2 = savings(&path, &distances, 2);

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

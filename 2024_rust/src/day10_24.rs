use super::tools;
use std::time::Instant;
use std::collections::HashMap;

fn flood_fill(map: &HashMap<(i32,i32), i32>, fill: &mut HashMap<(i32,i32), HashMap<i32, i32>>, pos: (i32,i32), value: i32, single: bool) {
    if single && fill.contains_key(&pos) && fill[&pos].contains_key(&value) && fill[&pos][&value] == 1 {
        return;
    }

    *fill.entry(pos).or_insert(HashMap::new()).entry(value).or_insert(0) += 1;

    let dirs = vec![(0,1), (0,-1), (1,0), (-1,0)];
    dirs
        .iter()
        .for_each(|(dr, dc)| {
            let r = pos.0 + dr;
            let c = pos.1 + dc;
            if map.contains_key(&(r,c)) && map[&(r,c)] - map[&pos] == 1 {
                flood_fill(map, fill, (r,c), value, single);
            }
        });
}

fn solve(map: &HashMap<(i32,i32), i32>, part: i32) -> i32 {
    let trailheads = map.iter().filter(|(_, &v)| v == 0).map(|(&k,_)| k ).collect::<Vec<(i32,i32)>>();
    let nines = map.iter().filter(|(_, &v)| v == 9).map(|(&k,_)| k ).collect::<Vec<(i32,i32)>>();

    let mut fill: HashMap<(i32,i32), HashMap<i32, i32>> = HashMap::new();
    trailheads
        .iter()
        .enumerate()
        .for_each(|(i, &pos)| {
            flood_fill(&map, &mut fill, pos, i as i32, part == 1);
        });

    nines.iter().map(|&pos| {
        fill[&pos].iter().map(|(_,v)| v).sum::<i32>()
    }).sum::<i32>()
}

#[allow(dead_code)]
pub fn run(real: bool, print_result: bool) -> (u128, u128, u128) {
    let start0 = Instant::now();

    let input_file: &str = if !real {
        "./input/day10-test.txt"
    } else {
        "./input/day10-real.txt"
    };
    let input = tools::get_input(String::from(input_file));

    // let (R, C) = (input.len(), input[0].len());

    let mut map: HashMap<(i32,i32), i32> = HashMap::new();
    input
        .iter()
        .enumerate()
        .for_each(|(r, line)| {
            line.chars().enumerate().for_each(|(c, ch)| {
                map.insert((r as i32, c as i32),
                    if ch != '.' {
                        ch.to_string().parse::<i32>().unwrap()                        
                    } else {
                        -1
                    }
                );
            });
        });

    let after0 = Instant::now();

    let start1 = Instant::now();

    let res1 = solve(&map, 1);

    let after1 = Instant::now();
    if print_result {
        println!("Part 1: {}", res1);
    }

    let start2 = Instant::now();

    let res2 = solve(&map, 2);

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

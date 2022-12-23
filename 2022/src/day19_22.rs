use super::tools;
use std::time::Instant;
use std::collections::{HashSet};
use priority_queue::PriorityQueue;

#[derive(Clone, Debug)]
struct Blueprint {
    costs: [[u32; 4]; 4],
}

fn find_max_geode(
    bp: &Blueprint,
    time_left: i8,
) -> u32 {

    let mut ans = 0;
    let mut pq = PriorityQueue::new();
    let mut visited: HashSet<(i8, [u32; 4], [u32; 4])> = HashSet::new();

    let prio = | a: [u32;4] | -> u64 {
        a[0] as u64 + a[1] as u64 * 10 + a[2] as u64 * 100 + a[3] as u64 * 1000
    };

    pq.push((time_left, [0,0,0,0], [1,0,0,0], None, [0,0,0,0]), prio([0,0,0,0]) );
    while let Some((s,_)) = pq.pop() {

        if s.0 > 0 {
            let mut new_produced = s.4.clone();
            (0..4).for_each(|index| {
                new_produced[index] += s.2[index];
            });

            let mut new_purse = s.1.clone();
            (0..4).for_each(|index| {
                new_purse[index] += s.2[index];
            });

            let mut new_robots = s.2.clone();
            if let Some(to_finish_build) = s.3 {
                new_robots[to_finish_build] += 1;
            }

            let state = (s.0, new_purse, new_robots);
            if !visited.contains(&state) {
                visited.insert(state.clone());

                pq.push( (s.0-1, new_purse, new_robots, None, new_produced), prio(new_produced) );

                (0..4)
                    .filter(|&robot| (0..4).all(|resource| new_purse[resource] >= bp.costs[robot][resource]))
                    .for_each(|robot| {
                        let mut next_purse = new_purse.clone();
                        (0..4).for_each(|resource| {
                            next_purse[resource] -= bp.costs[robot][resource];
                        });
                        pq.push( (s.0-1, next_purse, new_robots, Some(robot), new_produced), prio(new_produced));
                    });
            }
        } else {
            ans = ans.max(s.1[3]);
        }
    }
    println!("ans: {}", ans);
    ans
}

#[allow(dead_code)]
pub fn run(real: bool, print_result: bool) -> (u128, u128, u128) {
    let start0 = Instant::now();

    let input_file: &str = if !real {
        "./input/day19_22_test.txt"
    } else {
        "./input/day19_22_real.txt"
    };
    let input = tools::get_input(String::from(input_file));
    let blueprints = input
        .iter()
        .map(|line| {
            line.split_whitespace()
                .enumerate()
                .filter(|&(i, _)| i == 6 || i == 12 || i == 18 || i == 21 || i == 27 || i == 30)
                .map(|(_, s)| s.parse::<u32>().unwrap())
                .collect::<Vec<_>>()
        })
        .map(|v| Blueprint {
            costs: [
                [v[0], 0, 0, 0],
                [v[1], 0, 0, 0],
                [v[2], v[3], 0, 0],
                [v[4], 0, v[5], 0],
            ],
        })
        .collect::<Vec<_>>();

    let after0 = Instant::now();

    let start1 = Instant::now();

    let res1 = blueprints
        .iter()
        .enumerate()
        .map(|(index, bp)| (index as u32 + 1) * find_max_geode(&bp, 24) )
        .sum::<u32>();
    
    let after1 = Instant::now();
    if print_result {
        println!("Part 1: {}", res1);
    }

    let start2 = Instant::now();

        let res2 = blueprints
        .iter()
        .take(3)
        .map(|bp| { find_max_geode(&bp, 32) })
        .fold(1, |acc, v| acc*v);

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

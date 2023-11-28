use super::tools;
use std::time::Instant;
// use std::collections::HashMap;
use std::collections::{HashSet, VecDeque};
// use std::collections::{VecDeque};

#[derive(Clone, Debug)]
struct Blueprint {
    costs: [[u32; 4]; 4],
}

fn find_max_geode(bp: &Blueprint, time_left: i8, purse: [u32; 4], robots: [u32; 4]) -> u32 {
    let mut ans = 0;

    let mut seen: HashSet<(i8, [u32; 4], [u32; 4])> = HashSet::new();

    let cost_ore = bp.costs.iter().map(|c| c[0]).max().unwrap();

    let mut dq: VecDeque<(i8, [u32;4], [u32;4])> = VecDeque::new();
    let key = (time_left, purse, robots);
    dq.push_back(key);

    while let Some(state) = dq.pop_front() {
        ans = ans.max(state.1[3]);

        if state.0 > 0 {
            let mut reduced_state = state;

            // if there are more ore robots than costs for ore, no extra robots needed
            if reduced_state.2[0] >= cost_ore {
                reduced_state.2[0] = cost_ore;
            }

            // costs obsidian robots is ore and clay. If there is more clay robots than clay costs, reduce them
            if reduced_state.2[1] >= bp.costs[2][1] {
                reduced_state.2[1] = bp.costs[2][1];
            }

            // costs geode robots is ore and obsidian. If there is more obsidian robots than obsidian costs, reduce them
            if reduced_state.2[2] >= bp.costs[3][2] {
                reduced_state.2[2] = bp.costs[3][2];
            }

            // if purse for ore is larger than costs needed to pay for all future ore than purse is reduced
            if reduced_state.1[0] >= time_left as u32 * cost_ore - reduced_state.2[0] * (time_left as u32 - 1) {
                reduced_state.1[0] = time_left as u32 * cost_ore - reduced_state.2[0] * (time_left as u32 - 1);
            }

            // if purse for clay is larger than costs needed to pay for all future clay than purse is reduced            
            if reduced_state.1[1] >= time_left as u32 * bp.costs[2][1] - reduced_state.2[1] * (time_left as u32 - 1) {
                reduced_state.1[1] = time_left as u32 * bp.costs[2][1] - reduced_state.2[1] * (time_left as u32 - 1);
            }

            // if purse for obsidian is larger than costs needed to pay for all future obsidian than purse is reduced
            if reduced_state.1[2] >= time_left as u32 * bp.costs[3][2] - reduced_state.2[2] * (time_left as u32 - 1) {
                reduced_state.1[2] = time_left as u32 * bp.costs[3][2] - reduced_state.2[2] * (time_left as u32 - 1);
            }

            if !seen.contains(&reduced_state) {
                seen.insert(reduced_state);

                reduced_state.0 -= 1;

                let mut new_state = reduced_state;
                reduced_state.2.iter().enumerate().for_each(|(resource,robot_count)| {
                    new_state.1[resource] += robot_count;
                });
                dq.push_back(new_state);

                if reduced_state.1[0] >= bp.costs[0][0] { // buy ore robot
                    let mut new_state = reduced_state;
                    reduced_state.2.iter().enumerate().for_each(|(resource,robot_count)| {
                        new_state.1[resource] += robot_count - bp.costs[0][resource];
                    });
                    new_state.2[0] += 1;
                    dq.push_back(new_state);
                }

                if reduced_state.1[0] >= bp.costs[1][0] { // buy clay robot
                    let mut new_state = reduced_state;
                    reduced_state.2.iter().enumerate().for_each(|(resource,robot_count)| {
                        new_state.1[resource] += robot_count - bp.costs[1][resource];
                    });
                    new_state.2[1] += 1;
                    dq.push_back(new_state);
                }

                if reduced_state.1[0] >= bp.costs[2][0] && reduced_state.1[1] >= bp.costs[2][1] { // buy clay robot
                    let mut new_state = reduced_state;
                    reduced_state.2.iter().enumerate().for_each(|(resource,robot_count)| {
                        new_state.1[resource] += robot_count - bp.costs[2][resource];
                    });
                    new_state.2[2] += 1;
                    dq.push_back(new_state);
                }

                if reduced_state.1[0] >= bp.costs[3][0] && reduced_state.1[2] >= bp.costs[3][2] { // buy clay robot
                    let mut new_state = reduced_state;
                    reduced_state.2.iter().enumerate().for_each(|(resource,robot_count)| {
                        new_state.1[resource] += robot_count - bp.costs[3][resource];
                    });
                    new_state.2[3] += 1;
                    dq.push_back(new_state);
                }
            }
        }
    }

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
        .map(|(index, bp)| {
            (index as u32 + 1) * find_max_geode(&bp, 24, [0, 0, 0, 0], [1, 0, 0, 0])
        })
        .sum::<u32>();
    
    let after1 = Instant::now();
    if print_result {
        println!("Part 1: {}", res1);
    }

    let start2 = Instant::now();

    let res2 = blueprints
        .iter()
        .take(3)
        .map(|bp| {
            find_max_geode(&bp, 32, [0, 0, 0, 0], [1, 0, 0, 0])
        })
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

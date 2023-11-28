use super::tools;
use std::time::Instant;
use std::collections::HashMap;
// use std::collections::{HashSet, VecDeque};

#[derive(Clone, Debug)]
struct Blueprint {
    costs: [[u32; 4]; 4],
}

fn find_max_geode(
    bp: &Blueprint,
    time_left: i8,
    purse: [u32; 4],
    robots: [u32; 4],
    rtbb: Option<usize>,
    mem: &mut HashMap<(i8, [u32; 4], [u32; 4]), u32>,
) -> u32 {
    let mut ans = 0;

    if time_left == 0 {
        return purse[3];
    }

    let mut new_purse = purse.clone();
    (0..4).for_each(|index| {
        new_purse[index] += robots[index];
    });

    let mut new_robots = robots.clone();
    if let Some(to_finish_build) = rtbb {
        new_robots[to_finish_build] += 1;
    }

    let key = (time_left, new_purse, new_robots);
    if mem.contains_key(&key) {
        ans = mem[&key];
        return ans;
    }

    ans = ans.max(find_max_geode(
        bp,
        time_left - 1,
        new_purse,
        new_robots,
        None,
        mem,
    ));

    (0..4)
        .filter(|&robot| (0..4).all(|resource| new_purse[resource] >= bp.costs[robot][resource]))
        .for_each(|robot| {
            let mut next_purse = new_purse.clone();
            (0..4).for_each(|resource| {
                next_purse[resource] -= bp.costs[robot][resource];
            });
            ans = ans.max(find_max_geode(
                bp,
                time_left - 1,
                next_purse,
                new_robots,
                Some(robot),
                mem,
            ));
        });

    mem.insert(key, ans);
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
            let mut mem: HashMap<(i8, [u32; 4], [u32; 4]), u32> =
                HashMap::new();
            let ans = find_max_geode(&bp, 24, [0, 0, 0, 0], [1, 0, 0, 0], None, &mut mem);
            (index as u32 + 1) * ans
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
            let mut mem: HashMap<(i8, [u32; 4], [u32; 4]), u32> =
                HashMap::new();
            find_max_geode(&bp, 32, [0, 0, 0, 0], [1, 0, 0, 0], None, &mut mem)
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

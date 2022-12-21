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
    mem: &mut HashMap<(i8, [u32; 4], [u32; 4]), u32>,
) -> u32 {
    let mut ans = 0;

    // for _ in 0..time_left {
    //     print!(" ");
    // }
    // println!("-> {:?}", (time_left, purse, robots));
    // println!("START find_max_geode: {:?}", (time_left, purse, robots));

    if time_left == 0 {
        // println!("------------------ returning from {:?}", (time_left, purse, robots));
        return purse[3];
    }

    // println!("=========== FINDING NEXT ROBOT TO PRODUCE ===========");

    (0..4)
        .filter(|&robot| (0..4).all(|resource| purse[resource] >= bp.costs[robot][resource]))
        .filter(|&robot| robots[robot] < bp.costs.iter().map(|v| v[robot]).max().unwrap())
        .for_each(|robot| {
            // println!("   - - - - ROBOT {} - - - - ", robot);
            let mut next_purse = purse.clone();
            (0..4).for_each(|resource| {
                next_purse[resource] -= bp.costs[robot][resource];
            });

            let mut next_robots = robots.clone();
            next_robots[robot] += 1;

            let t = (0..4).map(|rbt| {

                if let Some(tt) = (0..4)
                    .filter(|&resource| bp.costs[rbt][resource] > 0)
                    .map(|resource| 
                        if next_robots[resource] > 0 {
                            (
                                1.0f32 + (bp.costs[rbt][resource] as f32 - next_purse[resource] as f32 - robots[resource] as f32) / next_robots[resource] as f32
                            ).ceil() as i64
                        } else {
                            i64::MAX
                        }
                    )
                    .filter(|&v| v>0)
                    .max() {
                        // println!("***** next t for rbt {} = {}", rbt, tt);
                        tt        
                    } else {
                        i64::MAX
                    }
            }).min().unwrap().max(1);
        
            // println!("    found t = {} as next event", t);
            
            if t <= time_left as i64 {

                // println!("next t: {} with robot {}", t, robot);
                (0..4).for_each(|resource| {
                    next_purse[resource] += t as u32 * robots[resource];
                });

                // println!("    going into recursion with {:?}", (time_left - t as i8, next_purse, next_robots));

                ans = ans.max(find_max_geode(
                    bp,
                    time_left - t as i8,
                    next_purse,
                    next_robots,
                    mem,
                ));
            }
        });


    let t = (0..4).map(|rbt| {
        if let Some(tt) = (0..4)
            .filter(|&resource| bp.costs[rbt][resource] > 0)
            .map(|resource| 
                if robots[resource] > 0 {
                    (
                        1.0f32 + (bp.costs[rbt][resource] as f32 - purse[resource] as f32 - robots[resource] as f32) / robots[resource] as f32
                    ).ceil() as i64
                } else {
                    i64::MAX
                }
            )
            .filter(|&v| v>0)
            .max() {
                tt        
            } else {
                i64::MAX
            }
    }).min().unwrap().max(1);

    if t <= time_left as i64 {
        // println!("next t: {}  with no robot", t);
        let mut next_purse = purse.clone();
        (0..4).for_each(|resource| {
            next_purse[resource] += t as u32 * robots[resource];
        });

        // println!("    going into recursion with {:?}", (time_left - t as i8, next_purse, robots));

        ans = ans.max(find_max_geode(
            bp,
            time_left - t as i8,
            next_purse,
            robots,
            mem,
        ));
    }
    
    // println!("DONE with ans: {}   --- {:?}", ans, (time_left, purse, robots));
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

            println!("BLUEPRINT: {:?}", bp);

            let ans = find_max_geode(&bp, 24, [0, 0, 0, 0], [1, 0, 0, 0], &mut mem);
            println!("ans: {}", ans);
            (index as u32 + 1) * ans
        })
        .sum::<u32>();
    
    let after1 = Instant::now();
    if print_result {
        println!("Part 1: {}", res1);
    }

    let start2 = Instant::now();

    let res2 = 0;
    // let res2 = blueprints
    //     .iter()
    //     .take(3)
    //     .map(|bp| {
    //         let mut mem: HashMap<(i8, [u32; 4], [u32; 4]), u32> =
    //             HashMap::new();
    //         find_max_geode(&bp, 32, [0, 0, 0, 0], [1, 0, 0, 0], &mut mem)
    //     })
    //     .fold(1, |acc, v| acc*v);

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

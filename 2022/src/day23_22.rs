use super::tools;
use std::time::Instant;
use std::collections::{HashSet, VecDeque, HashMap};

#[allow(dead_code)]
pub fn run(real: bool, print_result: bool) -> (u128, u128, u128) {
    let start0 = Instant::now();

    let input_file: &str = if !real {
        "./input/day23_22_test.txt"
    } else {
        "./input/day23_22_real.txt"
    };
    let input = tools::get_input(String::from(input_file));

    let mut elves: HashSet<(i64,i64)> = HashSet::new();
    input.iter().enumerate().for_each(|(y,line)| { line.chars().enumerate().for_each(|(x,c)| {
        if c == '#' { elves.insert((x as i64, y as i64)); }
    })});

    let mut test_actions: VecDeque<([(i64,i64);3] ,(i64,i64))> = VecDeque::from([
        ([(-1,-1), ( 1,-1), ( 0,-1)], ( 0,-1)),
        ([(-1, 1), ( 1, 1), ( 0, 1)], ( 0, 1)),
        ([(-1,-1), (-1, 1), (-1, 0)], (-1, 0)),
        ([( 1,-1), ( 1, 1), ( 1, 0)], ( 1, 0))
    ]);

    let all_eight = [(-1,-1), (-1,0), (-1,1), (0,-1), (0,1), (1,-1), (1,0), (1,1)];

    let rotate_actions = | dq: &mut VecDeque<([(i64,i64);3] ,(i64,i64))> | {
        if let Some(val) = dq.pop_front() {
            dq.push_back(val);
        }
    };

    let _print_elves = | elves: &HashSet<(i64,i64)> | {
        let min_field = elves.iter().fold((i64::MAX, i64::MAX), |acc, elf| (acc.0.min(elf.0), acc.1.min(elf.1)));
        let max_field = elves.iter().fold((i64::MIN, i64::MIN), |acc, elf| (acc.0.max(elf.0), acc.1.max(elf.1)));
        for y in min_field.1..=max_field.1 {
            for x in min_field.0..=max_field.0 {
                print!("{}", if elves.contains(&(x,y)) { '#' } else { '.' });
            }
            println!("");
        }
        println!("");
    };

    let round = | elves: &HashSet<(i64,i64)>, test_actions: &VecDeque<([(i64,i64);3] ,(i64,i64))> | -> (bool, HashSet<(i64,i64)>) {
        let mut new_elves: HashMap<(i64,i64), Vec<(i64,i64)>> = HashMap::new();
        let mut no_option: HashSet<(i64,i64)> = HashSet::new();
        for elf in elves {
            let mut found = false;
            if all_eight.iter().any(|delta| elves.contains( &(elf.0 + delta.0, elf.1 + delta.1) )  ) {
                'inner: for ta in test_actions {
                    if ta.0.iter().all(|delta| !elves.contains( &(elf.0 + delta.0, elf.1 + delta.1) )  ) {
                        let new_pos = (elf.0 + ta.1.0, elf.1 + ta.1.1);
                        if let Some(ne) = new_elves.get_mut(&new_pos) {
                            ne.push(*elf);
                        } else {
                            new_elves.insert(new_pos, vec![*elf]);
                        }
                        found = true;
                        break 'inner;
                    }
                }
            } 
            if !found {
                no_option.insert(*elf);
            }
        };

        let mut next_elves: HashSet<(i64,i64)> = new_elves.iter().fold(HashSet::new(), |mut acc, (pos, old_elves_pos)| {
            if old_elves_pos.len() == 1 { 
                acc.insert(*pos); 
            } else {
                old_elves_pos.iter().for_each(|oep| { acc.insert(*oep); });
            }
            acc
        });

        no_option.iter().for_each(|elf| {
            next_elves.insert(*elf);
        });

        let moved: bool = !next_elves.eq(&elves);

        assert!(elves.len() == next_elves.len(), "ELVES LOST");

        (moved, next_elves)
    };

    let after0 = Instant::now();

    let start1 = Instant::now();


    (1..=10).for_each(|_| {
        (_, elves) = round(&elves, &mut test_actions);
        rotate_actions(&mut test_actions);
    });

    let min_field = elves.iter().fold((i64::MAX, i64::MAX), |acc, elf| (acc.0.min(elf.0), acc.1.min(elf.1)));
    let max_field = elves.iter().fold((i64::MIN, i64::MIN), |acc, elf| (acc.0.max(elf.0), acc.1.max(elf.1)));

    let res1 = (max_field.0 - min_field.0 + 1) * (max_field.1 - min_field.1 + 1) - elves.len() as i64;
    let after1 = Instant::now();
    if print_result {
        println!("Part 1: {}", res1);
    }

    let start2 = Instant::now();

    let mut res2 = 0;
    let mut moved;
    for r in 11.. {
        (moved, elves) = round(&elves, &mut test_actions);
        rotate_actions(&mut test_actions);
        if !moved {
            res2 = r;
            break;
        }
    }

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

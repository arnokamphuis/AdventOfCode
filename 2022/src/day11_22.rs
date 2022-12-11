use super::tools;
use std::time::Instant;
use itertools::sorted;

#[derive(Clone, Debug)]
enum Operation {
    Plus,
    Times
}

#[derive (Clone, Debug)]
struct Monkey {
    items: Vec<i64>,
    op: Operation,
    opfactor: i64,
    divisible: i64,
    true_monkey: usize,
    false_monkey: usize,
}

impl Monkey {
    fn from(lines: &Vec<String>) -> Monkey {
        
       Monkey {
        items: lines[1][18..].split(", ").map(|s| s.parse::<i64>().unwrap()).collect(),
        op: if lines[2][23..24].chars().nth(0).unwrap() == '*' { Operation::Times } else { Operation::Plus },
        opfactor: lines[2][25..].parse::<i64>().unwrap_or(-1),
        divisible: lines[3][21..].parse::<i64>().unwrap(),
        true_monkey: lines[4].split_whitespace().rev().next().unwrap().parse::<usize>().unwrap(),
        false_monkey: lines[5].split_whitespace().rev().next().unwrap().parse::<usize>().unwrap(),
       } 
    }

}

fn do_monkey_business(monkeys: &mut Vec<Monkey>, runs: usize, part: usize) -> usize {
    let mut least_common_multiple = 1;
    monkeys.iter().for_each(|monkey| {
        least_common_multiple *= monkey.divisible;
    });

    let mut inspections: Vec<usize> = vec![0; monkeys.len()];
    (0..runs).for_each(|_| {
        for m in 0..monkeys.len() {
            let items = monkeys[m].items.clone();
            monkeys[m].items.clear();
            items.iter().for_each(|&item| {
                let mut new_item = item;
                inspections[m] += 1;
                let factor = if monkeys[m].opfactor == -1 { new_item } else { monkeys[m].opfactor };
                new_item = match monkeys[m].op {
                    Operation::Plus => { new_item + factor }
                    Operation::Times => { new_item * factor }
                };
                match part {
                    1 => new_item = (new_item as f64 / 3.0).floor() as i64,
                    2 => new_item %= least_common_multiple,
                    _ => panic!()
                }
                let target = if new_item % monkeys[m].divisible == 0 {
                    monkeys[m].true_monkey
                } else {
                    monkeys[m].false_monkey
                };
                monkeys[target].items.push(new_item);
            });
        }
    });
    sorted(&inspections).rev().take(2).fold(1, |acc, v| acc*v)
}

#[allow(dead_code)]
pub fn run(real: bool, print_result: bool) -> (u128, u128, u128) {
    let start0 = Instant::now();

    let input_file: &str = if !real {
        "./input/day11_22_test.txt"
    } else {
        "./input/day11_22_real.txt"
    };
    let input = tools::get_input(String::from(input_file));

    let initial_monkeys = input.chunks(7).map(|lines| Monkey::from(&lines.to_vec())).collect::<Vec<Monkey>>();
    let mut monkeys = initial_monkeys.clone();

    let after0 = Instant::now();

    let start1 = Instant::now();

    let res1 = do_monkey_business(&mut monkeys, 20, 1);
    // let mut inspections: Vec<usize> = vec![0; monkeys.len()];
    // (0..20).for_each(|_| {
    //     for m in 0..monkeys.len() {
    //         let items = monkeys[m].items.clone();
    //         monkeys[m].items.clear();
    //         items.iter().for_each(|&item| {
    //             let mut new_item = item;
    //             inspections[m] += 1;
    //             let factor = if monkeys[m].opfactor == -1 { new_item } else { monkeys[m].opfactor };
    //             new_item = match monkeys[m].op {
    //                 Operation::Plus => { new_item + factor }
    //                 Operation::Times => { new_item * factor }
    //             };
    //             new_item = (new_item as f64 / 3.0).floor() as i64;
    //             let target = if new_item % monkeys[m].divisible == 0 {
    //                 monkeys[m].true_monkey
    //             } else {
    //                 monkeys[m].false_monkey
    //             };
    //             monkeys[target].items.push(new_item);
    //         });
    //     }
    // });
    // println!("{:?}", sorted(&inspections).rev().collect::<Vec<_>>());
    // let res1 = sorted(&inspections).rev().take(2).fold(1, |acc, v| acc*v);

    let after1 = Instant::now();
    if print_result {
        println!("Part 1: {}", res1);
    }

    let start2 = Instant::now();

    monkeys = initial_monkeys.clone();
    let res2 = do_monkey_business(&mut monkeys, 10000, 2);

    // let mut least_common_multiple = 1;
    // monkeys.iter().for_each(|monkey| {
    //     least_common_multiple *= monkey.divisible;
    // });

    // inspections = vec![0; monkeys.len()];
    // (0..10000).for_each(|_| {
    //     for m in 0..monkeys.len() {
    //         let items = monkeys[m].items.clone();
    //         monkeys[m].items.clear();
    //         // println!("{} length items: {}", m, items.len());
    //         items.iter().for_each(|&item| {
    //             let mut new_item = item;
    //             inspections[m] += 1;
    //             let factor = if monkeys[m].opfactor == -1 { new_item } else { monkeys[m].opfactor };
    //             new_item = match monkeys[m].op {
    //                 Operation::Plus => { new_item + factor }
    //                 Operation::Times => { new_item * factor }
    //             };
    //             new_item %= least_common_multiple;
    //             let target = if new_item % monkeys[m].divisible == 0 {
    //                 monkeys[m].true_monkey
    //             } else {
    //                 monkeys[m].false_monkey
    //             };
    //             monkeys[target].items.push(new_item);
    //         });
    //     }
    // });
    // println!("{:?}", &inspections);//sorted(&inspections).rev().collect::<Vec<_>>());
    // let res2 = sorted(&inspections).rev().take(2).fold(1, |acc, v| acc*v);

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

use super::tools;
use std::time::Instant;
use super::linkedlist::LinkedList;
use std::rc::Rc;

#[allow(dead_code)]
pub fn run(real: bool, print_result: bool) -> (u128, u128, u128) {
    let start0 = Instant::now();

    let input_file: &str = if !real {
        "./input/day23_20_test.txt"
    } else {
        "./input/day23_20_real.txt"
    };
    let input = tools::get_input(String::from(input_file));
    let mut numbers = input[0].chars().map(|c| c.to_digit(10).unwrap() as usize).collect::<Vec<usize>>();
    let mut numbers2 = numbers.clone();
    (10..1_000_001).for_each(|v| numbers2.push(v));

    let max_n_part1: usize = 9usize;
    let max_n_part2: usize = 1_000_000usize;

    let mut ll_part1: LinkedList = LinkedList::new(numbers[0]);
    numbers.iter().skip(1).enumerate().for_each(|(index, &v)| ll_part1.append(v, numbers[index]));

    let mut ll_part2: LinkedList = LinkedList::new(numbers2[0]);
    numbers2.iter().skip(1).enumerate().for_each(|(index, &v)| ll_part2.append(v, numbers2[index]));

    let after0 = Instant::now();

    let start1 = Instant::now();

    let mut current_number = numbers[0];

    for _ in 0..100 {
        let mut pick_iter = ll_part1.iter_node(current_number).skip(1);
        let picks = (1..4usize).fold(vec![], |mut acc, _| {
            acc.push(pick_iter.next().unwrap().borrow().data); acc
        });

        picks.iter().for_each(|&v| ll_part1.remove(v));

        let mut destination = current_number-1;
        if destination < 1 {
            destination = max_n_part1;
        }
        while picks.contains(&destination) {
            destination-=1;
            if destination<1 {
                destination = max_n_part1;
            }
        }

        picks.iter().rev().for_each(|&n| {
            ll_part1.append(n, destination);
        });

        if let Some(ref current_node) = ll_part1.get(current_number).unwrap().borrow().next {
            current_number = current_node.borrow().data;
        }
    }

    let mut ans_iter = ll_part1.iter_node(1).skip(1);
    let res1 = (1..numbers.len()).map(|i| {
        ans_iter.next().unwrap().borrow().data.to_string()
    }).collect::<String>();

    let after1 = Instant::now();
    if print_result {
        println!("Part 1: {}", res1);
    }

    let start2 = Instant::now();

    let mut current_number = numbers2[0];

    for _ in 0..10_000_000 {
        let mut pick_iter = ll_part2.iter_node(current_number).skip(1);
        let picks = (1..4usize).fold(vec![], |mut acc, _| {
            acc.push(pick_iter.next().unwrap().borrow().data); acc
        });

        picks.iter().for_each(|&v| ll_part2.remove(v));

        let mut destination = current_number-1;
        if destination < 1 {
            destination = max_n_part2;
        }
        while picks.contains(&destination) {
            destination-=1;
            if destination<1 {
                destination = max_n_part2;
            }
        }

        picks.iter().rev().for_each(|&n| {
            ll_part2.append(n, destination);
        });

        if let Some(ref current_node) = ll_part2.get(current_number).unwrap().borrow().next {
            current_number = current_node.borrow().data;
        }
    }

    let mut ans_iter = ll_part2.iter_node(1).skip(1);
    let res2 = ans_iter.next().unwrap().borrow().data as u128 * ans_iter.next().unwrap().borrow().data as u128;

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

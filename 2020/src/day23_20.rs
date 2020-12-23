use super::tools;
use std::time::Instant;
use super::linkedlist::LinkedList;

fn play_cups(cups: &mut LinkedList, runs: usize) {
    let mut current_number = cups.start();
    let cup_count = cups.len();

    for _ in 0..runs {

        let mut pick_iter = cups.iter_node(current_number).skip(1);
        let picks = (1..4usize).fold(vec![], |mut acc, _| {
            acc.push(pick_iter.next().unwrap().borrow().data); acc
        });

        picks.iter().for_each(|&v| cups.remove(v));

        let mut destination = current_number-1;
        if destination < 1 {
            destination = cup_count;
        }

        while picks.contains(&destination) {
            destination-=1;
            if destination < 1 {
                destination = cup_count;
            }
        }

        picks.iter().rev().for_each(|&n| {
            cups.append(n, destination);
        });

        if let Some(ref next) = cups.get(current_number).unwrap().borrow().next {
            current_number = next.borrow().data;
        }
    }
}

#[allow(dead_code)]
pub fn run(real: bool, print_result: bool) -> (u128, u128, u128) {
    let start0 = Instant::now();

    let input_file: &str = if !real {
        "./input/day23_20_test.txt"
    } else {
        "./input/day23_20_real.txt"
    };
    let input = tools::get_input(String::from(input_file));
    let numbers1 = input[0].chars().map(|c| c.to_digit(10).unwrap() as usize).collect::<Vec<usize>>();

    let mut numbers2 = numbers1.clone();
    (10..1_000_001).for_each(|v| numbers2.push(v));

    let mut ll_part1: LinkedList = LinkedList::new(numbers1[0]);
    numbers1.iter().skip(1).enumerate().for_each(|(index, &v)| ll_part1.append(v, numbers1[index]));

    let mut ll_part2: LinkedList = LinkedList::new(numbers2[0]);
    numbers2.iter().skip(1).enumerate().for_each(|(index, &v)| ll_part2.append(v, numbers2[index]));

    let after0 = Instant::now();

    let start1 = Instant::now();

    play_cups(&mut ll_part1, 100);
    
    let mut ans_iter = ll_part1.iter_node(1).skip(1);
    let res1 = (1..numbers1.len()).map(|_| {
        ans_iter.next().unwrap().borrow().data.to_string()
    }).collect::<String>();

    let after1 = Instant::now();
    if print_result {
        println!("Part 1: {}", res1);
    }

    let start2 = Instant::now();

    play_cups(&mut ll_part2, 10_000_000);

    let mut ans_iter = ll_part2.iter_node(1).skip(1);
    let res2 = ans_iter.next().unwrap().borrow().data * ans_iter.next().unwrap().borrow().data;

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

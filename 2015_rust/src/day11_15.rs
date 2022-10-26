use super::tools;
use std::time::Instant;

#[allow(dead_code)]
pub fn run(real: bool, print_result: bool) -> (u128, u128, u128) {
    let start0 = Instant::now();

    let input_file: &str = if !real {
        "./input/day11_15_test.txt"
    } else {
        "./input/day11_15_real.txt"
    };
    let input = tools::get_input(String::from(input_file));

    let initial_password: Vec<u8> = input[0].chars().map(|c| c as u8).collect();

    let check_increasing_triple = | pw: &Vec<u8> | -> bool {
        pw.windows(3).filter(|nums| nums[0]+1 == nums[1] && nums[1]+1 == nums[2]).count() > 0
    };

    let check_difficult_characters = | pw: &Vec<u8> | -> bool {
        !(pw.contains(&('i' as u8)) || pw.contains(&('o' as u8)) || pw.contains(&('l' as u8)))
    };

    let check_two_nonoverlapping_pairs = | pw: &Vec<u8> | -> bool {
        let mut pairs = pw.windows(2).filter(|nums| nums[0]==nums[1]).map(|nums| nums[0]).collect::<Vec<u8>>();
        pairs.dedup();
        pairs.len() >= 2
    };

    let upgrade = | pw: &Vec<u8> | -> Vec<u8> {
        let mut new_pw = pw.clone();

        let mut index = 0;
        while index < 8 {
            new_pw[7-index] += 1;
            if new_pw[7-index] > 'z' as u8 {
                new_pw[7-index] = 'a' as u8;
                index += 1;
            } else {
                index = 8;
            }
        }

        new_pw
    };

    let find_next = | pw: &Vec<u8> | -> Vec<u8> {
        let mut current_pw = pw.clone();
        while !(
            check_increasing_triple(&current_pw) && 
            check_difficult_characters(&current_pw) && 
            check_two_nonoverlapping_pairs(&current_pw)) {
            current_pw = upgrade(&current_pw);
        }
        current_pw
    };

    let after0 = Instant::now();

    let start1 = Instant::now();

    let mut current_pw = initial_password.clone();
    current_pw = find_next(&current_pw);

    let res1 = current_pw.iter().map(|&n| n as char).collect::<String>();

    let after1 = Instant::now();
    if print_result {
        println!("Part 1: {}", res1);
    }

    let start2 = Instant::now();

    current_pw = find_next(&upgrade(&current_pw));

    let res2 = current_pw.iter().map(|&n| n as char).collect::<String>();

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

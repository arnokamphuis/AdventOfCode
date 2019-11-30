use std::collections::HashMap;
use std::time::Instant;

use super::tools;

#[allow(dead_code)]
pub fn run() {
    println!("Day 4 of 2016");

    // let input_file = "./input/day04_16_test.txt";
    let input_file = "./input/day04_16_real.txt";

    let start1 = Instant::now();

    let mut input = Vec::new();

    if let Ok(lines) = tools::read_lines(input_file) {
        for line in lines {
            if let Ok(value) = line {
                input.push(value);
            }
        }
    }

    let mut therooms = Vec::new();

    let mut count = 0;
    for line in &input {
        let len = line.len() - 6;
        let checksum = &line[len..][..5];
        let rooms = &line[..len - 1];
        let lastdash: usize = rooms.rfind('-').unwrap();
        let sector_id = &rooms[lastdash + 1..].parse::<i32>().unwrap();
        let realrooms = &rooms[..lastdash];
        let words = realrooms.split('-');

        let mut frequency: HashMap<char, u32> = HashMap::new();
        for word in words {
            for c in word.chars() {
                *frequency.entry(c).or_insert(0) += 1;
            }
        }
        let mut count_vec: Vec<(&char, &u32)> = frequency.iter().collect();
        count_vec.sort_by(|b, a| b.0.cmp(a.0));
        count_vec.sort_by(|a, b| b.1.cmp(a.1));
        if count_vec.len() >= 5 {
            let topfive = &count_vec[..5];
            let mut csum = String::from("");
            for t in topfive {
                csum.push(*t.0);
            }
            if csum == String::from(checksum) {
                count += sector_id;
                therooms.push((*sector_id, realrooms));
                // println!("freq: {:?}", csum);
            }
        }
    }

    let after1 = Instant::now();
    println!("Part 1: {}, in {:?}", count, after1.duration_since(start1));

    let start2 = Instant::now();

    let mut northpolesector = 0;
    for id_room in &therooms {
        let roomname = id_room.1;
        let rotate = id_room.0 % 26;
        let mut newname = String::from("");
        for rnc in roomname.chars() {
            let mut nc = ' ';
            if rnc != '-' {
                nc = std::char::from_u32(
                    ((((rnc as u8) - ('a' as u8) + rotate as u8) % 26) + ('a' as u8)) as u32,
                )
                .unwrap();
            }
            newname.push(nc);
        }
        if newname.contains("northpole") {
            northpolesector = id_room.0;
        }
    }

    let after2 = Instant::now();
    println!(
        "Part 2: {}, in {:?}",
        northpolesector,
        after2.duration_since(start2)
    );
}

// use std::collections::HashSet;
use md5;
use std::time::Instant;
extern crate hex;
// use super::tools;

#[allow(dead_code)]
pub fn run() {
    println!("Day 5 of 2016");

    let door_id = "ffykfhsq";
    // let door_id = "abc";

    let start1 = Instant::now();

    let mut code = String::from("");
    let mut count = 0;
    let mut number = 100000;
    while count < 8 {
        loop {
            let door = String::from(door_id) + &number.to_string();
            let hash = md5::compute(door.as_bytes());
            let hex = hex::encode(hash);
            let mut found = true;
            for c in hex[0..5].chars() {
                if c != '0' {
                    found = false;
                }
            }
            number += 1;
            if found {
                // println!("hash is {:?}", hex);
                code.push(hex.chars().nth(5).unwrap());
                break;
            }
        }
        count += 1;
    }

    let after1 = Instant::now();
    println!("Part 1: {}, in {:?}", code, after1.duration_since(start1));

    let start2 = Instant::now();

    let mut newcode = vec![' ', ' ', ' ', ' ', ' ', ' ', ' ', ' '];
    count = 0;
    number = 100000;
    while count < 8 {
        loop {
            let door = String::from(door_id) + &number.to_string();
            let hash = md5::compute(door.as_bytes());
            let hex = hex::encode(hash);
            let mut found = true;
            for c in hex[0..5].chars() {
                if c != '0' {
                    found = false;
                }
            }
            number += 1;
            if found {
                // println!("hash is {:?}", hex);
                let pos = (hex.chars().nth(5).unwrap() as u8 - '0' as u8) as usize;
                let val = hex.chars().nth(6).unwrap();
                // println!("pos is {}", pos);
                if pos < 8 && newcode[pos] == ' ' {
                    newcode[pos] = val;
                    break;
                }
            } else {
                //println!("number is {} and hash is {:?}", number, hash);
            }
        }
        count += 1;
    }
    code = String::from("");
    for c in newcode {
        code.push(c);
    }

    let after2 = Instant::now();
    println!("Part 2: {}, in {:?}", code, after2.duration_since(start2));
}

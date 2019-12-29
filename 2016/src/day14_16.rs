use std::time::Instant;
use md5;

fn gen_hash(index: usize, salt: &str) -> String {
    format!("{:x}", md5::compute(format!("{}{}", salt, index).to_string()))
}

fn gen_hash_2016(index: usize, salt: &str) -> String {
    let mut h = gen_hash(index,salt);
    for _ in 0..2016 {
        h = format!("{:x}",md5::compute(h));
    }
    h
}

fn find_triplet(hash: &String) -> char {
    let chars: Vec<char> = hash.chars().collect();
    for i in 2..chars.len() {
        if chars[i-2]==chars[i-1] && chars[i-1]==chars[i] {
            return chars[i];
        }
    }
    ' '
}

fn run_part(part: usize) -> usize {
    let salt = "yjdafjpo";
    let mut hashes: Vec<String> = vec![];
    let mut index = 0;
    let mut validkeys: Vec<String> = vec![];
    loop {
        let mut i = hashes.len();
        while i < index+1001 { 
            match part {
                1 => { hashes.push(gen_hash(i, salt)); }
                2 => { hashes.push(gen_hash_2016(i, salt)); }
                _ => {}
            }
            i+=1; 
        }

        let currenthash = &hashes[index];
        let found_char = find_triplet(&currenthash);
        let mut check_index = index+1;
        while check_index < index+1001 {
            let hash = &hashes[check_index];
            if hash.contains(&((0..5).map(|_| found_char).collect::<String>())) {
                break;
            }
            check_index+=1;
        }
        if check_index != index+1001 {
            validkeys.push(currenthash.clone());
        }

        if validkeys.len() == 64 {
            return index;
        }
        index += 1;
    }
}

#[allow(dead_code)]
pub fn run() {
    println!("Day 14 of 2016");

    let start1 = Instant::now();

    let res1 = run_part(1);

    let after1 = Instant::now();
    println!(
        "Part 1: {}, in {:?}",
        res1,
        after1.duration_since(start1)
    );

    let start2 = Instant::now();

    let res2 = run_part(2);

    let after2 = Instant::now();
    println!(
        "Part 2: {}, in {:?}",
        res2,
        after2.duration_since(start2)
    );
}

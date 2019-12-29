use std::time::Instant;
use super::tools;

use permutohedron::heap_recursive;

fn process(current: String, operation: String) -> String {
    let mut result = current.clone();

    let mut op_iter = operation.split_whitespace();
    let op = op_iter.next().unwrap();

    // println!("{}", op);

    match op {
        "swap" => {
            let swaptype = op_iter.next().unwrap();
            match swaptype {
                "position" => {
                    let pos1 = op_iter.next().unwrap().parse::<usize>().unwrap();
                    op_iter.next(); op_iter.next();
                    let pos2 = op_iter.next().unwrap().parse::<usize>().unwrap();
                    
                    let minpos = std::cmp::min(pos1,pos2);
                    let maxpos = std::cmp::max(pos1,pos2);
                    result = current[0..minpos].to_string();
                    result.push(current.chars().nth(maxpos).unwrap());
                    result.push_str(&current[minpos+1..maxpos].to_string());
                    result.push(current.chars().nth(minpos).unwrap());
                    result.push_str(&current[maxpos+1..].to_string());
                }
                "letter" => {
                    let letter1 = op_iter.next().unwrap();
                    op_iter.next(); op_iter.next();
                    let letter2 = op_iter.next().unwrap();
                    result = current.replace(letter1,"_");
                    result = result.replace(letter2,letter1);
                    result = result.replace("_",letter2);
                }
                _ => {}
            }
        }
        "reverse" => {
            op_iter.next();
            let pos1 = op_iter.next().unwrap().parse::<usize>().unwrap();
            op_iter.next(); 
            let pos2 = op_iter.next().unwrap().parse::<usize>().unwrap();
            let minpos = std::cmp::min(pos1,pos2);
            let maxpos = std::cmp::max(pos1,pos2);            
            result = current[0..minpos].to_string();
            let target_to_reverse: String = current[minpos..maxpos+1].to_string();
            let revstr: String = target_to_reverse.chars().rev().collect();
            result.push_str(&revstr.to_string());
            result.push_str(&current[maxpos+1..].to_string());
        }
        "rotate" => {
            let rotate_type = op_iter.next().unwrap();
            match rotate_type {
                "left" | "right" => {
                    let mut dist = op_iter.next().unwrap().parse::<usize>().unwrap();
                    if rotate_type == "right" { dist = current.len() - dist; }
                    result = current[dist..].to_string();
                    result.push_str( &current[0..dist].to_string() );
                }
                "based" => {
                    op_iter.next(); op_iter.next(); op_iter.next(); op_iter.next();
                    let character = op_iter.next().unwrap();
                    if let Some(index) = current.find(character) {
                        let mut dist = (1 + index + if index >= 4 { 1 } else { 0 }) % current.len();
                        dist = current.len() - dist;
                        result = current[dist..].to_string();
                        result.push_str( &current[0..dist].to_string() );
                    }
                }
                _ => {}
            }
        }
        "move" => {
            op_iter.next();
            let pos1 = op_iter.next().unwrap().parse::<usize>().unwrap();
            let char1 = current.chars().nth(pos1).unwrap();

            op_iter.next(); op_iter.next(); 
            let pos2 = op_iter.next().unwrap().parse::<usize>().unwrap();

            let mut temp = current[0..pos1].to_string();
            temp.push_str( &current[pos1+1..].to_string() );

            result = temp[0..pos2].to_string();
            result.push(char1);
            result.push_str(&temp[pos2..].to_string());
        }
        _ => {}
    }

    result
}

fn perform_scramble(current: String, operations: &Vec<String>) -> String {
    let mut result: String = current.clone();
    operations.iter().for_each( |op| {
        result = process(result.clone(), op.to_string());
    });
    result
}

#[allow(dead_code)]
pub fn run() {
    println!("Day 21 of 2016");

    let start0 = Instant::now();
    // let input_file = "./input/day21_16_test.txt";
    let input_file = "./input/day21_16_real.txt";
    let input = tools::get_input(String::from(input_file));

    // let start = String::from("abcde");
    let start = String::from("abcdefgh");

    let after0 = Instant::now();
    println!(
        "Init in {:?}",
        after0.duration_since(start0)
    );

    let start1 = Instant::now();    

    let scrambled = perform_scramble(start.to_string(), &input);

    let after1 = Instant::now();
    println!(
        "Part 1: {}, in {:?}",
        scrambled,
        after1.duration_since(start1)
    );

    let start2 = Instant::now();

    let target = String::from("fbgdceah");

    let mut data = ['a','b','c','d','e','f','g','h'];
    let mut permutations = Vec::new();
    heap_recursive(&mut data, |permutation| {
        let perm: String = permutation.to_vec().into_iter().collect();
        permutations.push(perm);
    });

    let mut original = String::from("");
    for p in permutations {
        let scrambled = perform_scramble(p.clone(), &input);
        if scrambled == target {
            original = p.clone();
            break;
        }
    }

    let after2 = Instant::now();
    println!(
        "Part 2: {}, in {:?}",
        original,
        after2.duration_since(start2)
    );
}

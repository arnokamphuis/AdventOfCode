use super::tools;
use std::time::Instant;

fn is_num(s: &String) -> bool { return s.chars().all(char::is_numeric); }

fn split(number: &Vec<String>) -> (bool, Vec<String>) {
    let mut split_number = number.clone();

    let mut i = 0;
    while i < split_number.len() {
        let s = &split_number[i];
        if is_num(&s) {
            let left_val = s.parse::<u8>().unwrap();
            if left_val > 9 {
                // split
                let l = (left_val as f64 / 2.0).floor() as u8;
                let r = left_val - l;
                split_number.remove(i);
                split_number.insert(i, "[".to_string()); i+=1; 
                split_number.insert(i, l.to_string()); i+=1;
                split_number.insert(i, ",".to_string()); i+=1;
                split_number.insert(i, r.to_string()); i+=1;
                split_number.insert(i, "]".to_string());

                return (true, split_number.clone());
            } 
        }
        i += 1;
    }
    return (false, split_number.clone());
}

fn explode(number: &Vec<String>) -> (bool, Vec<String>) {
    let mut split_number = number.clone();

    let mut level = 0;
    let mut i = 0;
    while i < split_number.len() {
        let s = &split_number[i];
        if s == "[" { level += 1; }
        else if s == "]" { level -= 1; }
        else if is_num(&s) {
            let left_val = s.parse::<u8>().unwrap();
            if level > 4 { 
                // explode
                let right_val = split_number[i+2].parse::<u8>().unwrap();

                (0..5).for_each(|_| { split_number.remove(i-1); } );
                split_number.insert(i-1, 0.to_string());
                let mut j = i-2;
                loop {
                    if is_num(&split_number[j]) {
                        // to add together
                        split_number[j] = (split_number[j].parse::<u8>().unwrap() + left_val).to_string();
                        break;
                    }
                    if j == 0 { break; }
                    j -= 1; 
                }

                let mut j = i;
                loop {
                    if is_num(&split_number[j]) {
                        // to add together
                        split_number[j] = (split_number[j].parse::<u8>().unwrap() + right_val).to_string();
                        break;
                    }
                    if j == split_number.len()-1 { break; }
                    j += 1; 
                }
                return (true, split_number.clone());
            }
        }
        i += 1;
    }
    return (false, split_number.clone());
}


fn reduce(number: &Vec<String>) -> Vec<String> {
    let reduced_number = number.clone();

    let (explode_performed, reduced_number) = explode(&reduced_number);
    if explode_performed {
        return reduce(&reduced_number);
    } else {
        let (split_performed, reduced_number) = split(&reduced_number);
        if split_performed {
            return reduce(&reduced_number);
        } else {
            return reduced_number.clone();
        }
    }
}

fn score(number: &Vec<String>) -> u64 {
    let mut processed_number = number.clone();
    let mut i = 0;
    while i < processed_number.len()-2 {
        if is_num(&processed_number[i]) && is_num(&processed_number[i+2]) {
            let val = (3 * processed_number[i].parse::<u64>().unwrap() + 2 * processed_number[i+2].parse::<u64>().unwrap()).to_string();
            (0..5).for_each(|_| {
                processed_number.remove(i-1);
            });
            processed_number.insert(i-1, val);
            if processed_number.len() == 1 {
                break;
            }
        }
        i+=1;
    }

    if processed_number.len() != 1 { 
        return score(&processed_number);
    } else { 
        return processed_number[0].parse::<u64>().unwrap(); 
    }
}

#[allow(dead_code)]
pub fn run(real: bool, print_result: bool) -> (u128, u128, u128) {
    let start0 = Instant::now();

    let input_file: &str = if !real {
        "./input/day18_21_test.txt"
    } else {
        "./input/day18_21_real.txt"
    };
    let input = tools::get_input(String::from(input_file));
    let mut split_number: Vec<String> = reduce(&input[0].chars().map(|c| String::from(c)).collect());

    let after0 = Instant::now();

    let start1 = Instant::now();

    input.iter().skip(1).for_each(|line| {
        let mut new_number: Vec<String> = reduce(&line.chars().map(|c| String::from(c)).collect());
        split_number.insert(0,"[".to_string());
        split_number.push(",".to_string());
        split_number.append(&mut new_number);
        split_number.push("]".to_string());
        split_number = reduce(&split_number);
    });
    
    let res1 = score(&split_number);

    let after1 = Instant::now();
    if print_result {
        println!("Part 1: {}", res1);
    }

    let start2 = Instant::now();

    let mut max_score = 0;
    input.iter().for_each(|line1| {
        input.iter().for_each(|line2| {
            if line1 != line2 {
                let mut combination: Vec<String> = vec![];
                combination.push("[".to_string());
                combination.append(&mut line1.chars().map(|c| String::from(c)).collect());
                combination.push(",".to_string());
                combination.append(&mut line2.chars().map(|c| String::from(c)).collect());
                combination.push("]".to_string());
                max_score = max_score.max(score(&reduce(&combination)));        
            }
        });
    });

    let after2 = Instant::now();
    if print_result {
        println!("Part 2: {}", max_score);
    }

    (
        after0.duration_since(start0).as_nanos(),
        after1.duration_since(start1).as_nanos(),
        after2.duration_since(start2).as_nanos(),
    )
}

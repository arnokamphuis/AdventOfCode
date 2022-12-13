use super::tools;
use std::time::Instant;
use std::cmp::Ordering;

fn create_vec_from_string(s: &String) -> Vec<String> {
    let mut res: Vec<String> = vec![];
    let l = s.chars().collect::<Vec<char>>();
    let mut pos = 0;
    let mut start_array = 0;
    let mut brace_counter = 0;
    for i in 1..l.len() {
        if l[i] == '[' {
            if brace_counter == 0 {
                start_array = i;
            }
            brace_counter += 1;
            pos = i;
        } else if l[i] == ']' {
            if brace_counter == 1 {
                res.push(s[start_array..=i].to_string());
                pos = i;
            } else if brace_counter == 0 {
                if i-pos > 1 {
                    res.push(format!("{}", s[pos+1..i].chars().collect::<String>()));
                    pos = i;    
                }
            }
            brace_counter -= 1;            
        } else if l[i] == ',' && brace_counter == 0 {
            if i-pos > 1 {
                res.push(format!("{}", s[pos+1..i].chars().collect::<String>()));
            }
            pos = i;
        }
    }
    res
}

fn compare_arrays(arr: &Vec<&String>) -> Ordering {
    let is_array = | s: &String | -> bool {
        s.chars().nth(0).unwrap() == '['
    };

    let s1 = &arr[0];
    let s2 = &arr[1];

    if is_array(s1) && is_array(s2) {
        let v1 = create_vec_from_string(s1);
        let v2 = create_vec_from_string(s2);
        let mut index = 0;
        while index < v1.len() && index < v2.len() {
            let res = compare_arrays(&vec![&v1[index], &v2[index]]);
            if res != Ordering::Equal {
                return res; 
            }
            index += 1;
        }
        if index == v1.len() && index < v2.len() {
            return Ordering::Less;
        } else if index == v2.len() && index < v1.len() {
            return Ordering::Greater;
        } else {
            return Ordering::Equal;
        }
    } else if is_array(s1) && !is_array(s2) {
        let new_s2 = &format!("[{}]", s2);
        return compare_arrays(&vec![s1, new_s2]);
    } else if !is_array(s1) && is_array(s2) {
        let new_s1 = &format!("[{}]", s1);
        return compare_arrays(&vec![new_s1, s2]);
    } else {
        if s1.parse::<usize>().unwrap() > s2.parse::<usize>().unwrap() {
            return Ordering::Greater;
        } else if s1.parse::<usize>().unwrap() < s2.parse::<usize>().unwrap() {
            return Ordering::Less;
        } else {
            return Ordering::Equal;
        }
    }
}

fn compare(lines: [&String;2]) -> Ordering {
    compare_arrays(
        &lines
            .iter()
            .map(|&l| l)
            .collect::<Vec<&String>>()
        )
}

#[allow(dead_code)]
pub fn run(real: bool, print_result: bool) -> (u128, u128, u128) {
    let start0 = Instant::now();

    let input_file: &str = if !real {
        "./input/day13_22_test.txt"
    } else {
        "./input/day13_22_real.txt"
    };
    let input = tools::get_input(String::from(input_file));
    let mut packets: Vec<&String> = vec![];

    let after0 = Instant::now();

    let start1 = Instant::now();

    let mut pair_count = 0;
    let nr_pairs = (input.len() as f32/3.0).ceil() as usize;
    let mut correct = 0;

    while pair_count < nr_pairs {
        packets.push(&input[&pair_count * 3 + 0]);
        packets.push(&input[&pair_count * 3 + 1]);
        if compare([&input[&pair_count * 3 + 0], &input[pair_count * 3 + 1]]) == Ordering::Less {
            correct += pair_count+1;
        }
        pair_count += 1;
    }
    
    let after1 = Instant::now();
    if print_result {
        println!("Part 1: {}", correct);
    }

    let start2 = Instant::now();

    let extra_packet1 = "[2]".to_string();
    let extra_packet2 = "[6]".to_string();
    packets.push(&extra_packet1);
    packets.push(&extra_packet2);

    packets.sort_by(|a,b| compare([a,b]) );

    let res2 = packets
        .iter()
        .enumerate()
        .filter(|(_,s)| s.as_str() == "[2]" || s.as_str() == "[6]")
        .fold(1, |acc, (i,_)| acc * (i+1));

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

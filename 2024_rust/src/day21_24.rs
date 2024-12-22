use super::tools;
use std::time::Instant;
use std::collections::HashMap;
use itertools::Itertools;
use std::collections::VecDeque;

fn compute_sequences(keypad: &HashMap<char, (i8,i8)>) -> HashMap<(char,char), Vec<String>> {
    let mut sequences: HashMap<(char,char), Vec<String>> = HashMap::new();

    keypad.keys().for_each(|c| {
        keypad.keys().for_each(|d| {
            if c == d {
                sequences.insert((*c,*d), vec!['A'.to_string()]);
            } else {
                let mut possibilities: Vec<String> = vec![];
                let mut optimal: usize = usize::MAX;

                let pos = keypad.get(c).unwrap();

                let mut queue: VecDeque<((i8,i8), String)> = VecDeque::new();
                queue.push_back((*pos, "".to_string()));

                let mut stop: bool = false;
                while queue.len() > 0 && !stop {
                    let (pos, path) = queue.pop_front().unwrap();
                    vec![ ((pos.0-1, pos.1), '<'), ((pos.0+1, pos.1), '>'), ((pos.0, pos.1-1), '^'), ((pos.0, pos.1+1), 'v') ]
                        .iter()
                        .for_each(|(new_pos, dir)| {
                            if keypad.values().any(|&x| x == *new_pos) {
                                if *new_pos == *keypad.get(d).unwrap() {
                                    if optimal >= path.len() + 1 {
                                        optimal = path.len() + 1;
                                        possibilities.push(path.clone() + &dir.to_string() + "A");
                                    } else {
                                        stop = true;
                                        return;
                                    }
                                } else {
                                    queue.push_back((*new_pos, path.clone() + &dir.to_string()));
                                }
                            }
                        });
                }
                sequences.insert((*c,*d), possibilities);
            }
        });
    });
    sequences
}

fn solve(s: &String, sequences: &HashMap<(char,char), Vec<String>>) -> Vec<String> {

    let options = ("A".to_owned()+s)
        .chars()
        .zip(s.chars())
        .map(|(c, d)| { sequences.get(&(c, d)).unwrap().clone() })
        .collect::<Vec<Vec<String>>>();

    options.iter().multi_cartesian_product()
        .map(|x| x.into_iter().join(""))
        .collect::<Vec<String>>()
}

fn compute_length(sequence: &String, depth: usize, dir_sequences: &HashMap<(char,char), Vec<String>>, dir_lengths: &HashMap<(char,char), usize>, cache: &mut HashMap<(String, usize), usize>) -> usize {
    if cache.contains_key(&(sequence.clone(), depth)) {
        return *cache.get(&(sequence.clone(), depth)).unwrap();
    }

    let binding = "A".to_owned() + sequence;
    let pairs = binding.chars().zip(sequence.chars());

    if depth == 1 {
        return pairs
            .map(|(x, y)| { dir_lengths.get(&(x, y)).unwrap() })
            .sum();
    }
    
    let length = pairs.map(|(x, y)| {
        dir_sequences.get(&(x, y)).unwrap().iter().map(|subseq| {
            compute_length(subseq, depth - 1, dir_sequences, dir_lengths, cache)
        }).min().unwrap()
    }).sum();

    cache.insert((sequence.clone(), depth), length);
    return length;
}

#[allow(dead_code)]
pub fn run(real: bool, print_result: bool) -> (u128, u128, u128) {
    let start0 = Instant::now();

    let input_file: &str = if !real {
        "./input/day21-test.txt"
    } else {
        "./input/day21-real.txt"
    };
    let input = tools::get_input(String::from(input_file));
    let codes = input.iter().map(|x| x.chars().collect::<Vec<char>>()).collect::<Vec<Vec<char>>>();

    let mut num_keypad: HashMap<char, (i8,i8)> = HashMap::new();
    num_keypad.insert('0', (1,3));
    num_keypad.insert('A', (2,3));
    num_keypad.insert('1', (0,2));
    num_keypad.insert('2', (1,2));
    num_keypad.insert('3', (2,2));
    num_keypad.insert('4', (0,1));
    num_keypad.insert('5', (1,1));
    num_keypad.insert('6', (2,1));
    num_keypad.insert('7', (0,0));
    num_keypad.insert('8', (1,0));
    num_keypad.insert('9', (2,0));

    let mut dir_keypad: HashMap<char, (i8,i8)> = HashMap::new();
    dir_keypad.insert('^', (1,0));
    dir_keypad.insert('v', (1,1));
    dir_keypad.insert('<', (0,1));
    dir_keypad.insert('>', (2,1));
    dir_keypad.insert('A', (2,0));

    let mut dir_map: HashMap<(i8,i8), char> = HashMap::new();
    dir_map.insert((1,0), '>');
    dir_map.insert((0,1), 'v');
    dir_map.insert((-1,0), '<');
    dir_map.insert((0,-1), '^');

    let num_sequences:  HashMap<(char,char), Vec<String>> = compute_sequences(&num_keypad);
    let dir_sequences:  HashMap<(char,char), Vec<String>> = compute_sequences(&dir_keypad);
    let dir_lengths: HashMap<(char,char), usize> = dir_sequences.iter().map(|(key, value)| (*key, value[0].len())).collect();

    let after0 = Instant::now();

    let start1 = Instant::now();

    let mut res1 = 0;
    codes.iter().for_each(|code| {
        let inputs = solve(&code.iter().collect::<String>(), &num_sequences);
        let length = inputs.iter().map(|sequence| compute_length(sequence, 2, &dir_sequences, &dir_lengths, &mut HashMap::new())).min().unwrap();
        res1 += length * code[0..code.len()-1].iter().collect::<String>().parse::<usize>().unwrap();
    });

    let after1 = Instant::now();
    if print_result {
        println!("Part 1: {}", res1);
    }

    let start2 = Instant::now();

    let mut res2 = 0;
    codes.iter().for_each(|code| {
        let inputs = solve(&code.iter().collect::<String>(), &num_sequences);
        let length = inputs.iter().map(|sequence| compute_length(sequence, 25, &dir_sequences, &dir_lengths, &mut HashMap::new())).min().unwrap();
        res2 += length * code[0..code.len()-1].iter().collect::<String>().parse::<usize>().unwrap();
    });

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

use super::tools;
use std::time::Instant;
use std::collections::HashMap;

#[allow(dead_code)]
pub fn run(real: bool, print_result: bool) -> (u128, u128, u128) {
    let start0 = Instant::now();

    let input_file: &str = if !real {
        "./input/day07_22_test.txt"
    } else {
        "./input/day07_22_real.txt"
    };
    let input = tools::get_input(String::from(input_file));

    let mut list_mode = false;
    let mut current_dir = "".to_string();
    let mut parent: HashMap<String,String> = HashMap::new();
    let mut files: HashMap<String, Vec<(usize,String)>> = HashMap::new();

    for line in &input {
        if line.contains("$") {
            list_mode = false;
            let cmd = line[2..].split_whitespace().map(|s| s.to_string()).collect::<Vec<String>>();
            match cmd[0].as_str() {
                "cd" => {
                    if cmd[1] == "/".to_string() {
                        if !files.contains_key(&cmd[1]) {
                            files.insert(cmd[1].clone(),vec![]);
                            parent.insert(cmd[1].clone(), current_dir.clone());
                        }
                        current_dir = cmd[1].to_string();
                    } else if cmd[1] == ".." {
                        current_dir = parent[&current_dir].clone();
                    } else {
                        if !files.contains_key(&cmd[1]) {
                            files.insert(cmd[1].clone(),vec![]);
                            parent.insert(cmd[1].clone(), current_dir.clone());
                        }
                        current_dir = cmd[1].to_string();
                    }
            },
                "ls" => { list_mode = true; },
                _ => panic!()
            }
        } else if list_mode {
            let file = line.split_whitespace().map(|s| s.to_string()).collect::<Vec<String>>();
            if file[0].chars().nth(0).unwrap().is_numeric() {
                let size = file[0].parse::<usize>().unwrap();
                let name = file[1].to_string();

                files.get_mut(&current_dir).unwrap().push((size,name.clone()));
            } else {
                // let name = file[1].to_string();
            }
            // dirs.entry(current_dir).or_insert() Node::F((size, name.to_string()))

        } else {
            panic!();
        }
    }

    println!("dirs: {:?}", parent);
    println!("files: {:?}", files);

    let after0 = Instant::now();

    let start1 = Instant::now();

    let sizes = files.iter().map(|(dir, f)| (dir.clone(), 
        f.iter().fold(0, |acc, (s,_)| acc + s))).collect::<Vec<(String,usize)>>();
    println!("sizes: {:?}", sizes);

    let after1 = Instant::now();
    if print_result {
        println!("Part 1: {}", 0);
    }

    let start2 = Instant::now();

    let after2 = Instant::now();
    if print_result {
        println!("Part 2: {}", 0);
    }

    (
        after0.duration_since(start0).as_nanos(),
        after1.duration_since(start1).as_nanos(),
        after2.duration_since(start2).as_nanos(),
    )
}

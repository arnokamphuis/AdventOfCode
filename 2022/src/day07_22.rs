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
    let mut path: Vec<String> = vec![];
    let mut all_sizes: HashMap<String,usize> = HashMap::new();

    for line in &input {
        if line.contains("$") {
            list_mode = false;
            let cmd = line[2..].split_whitespace().map(|s| s.to_string()).collect::<Vec<String>>();
            match cmd[0].as_str() {
                "cd" => {
                    if cmd[1] == ".." {
                        path.pop();
                    } else {
                        path.push(cmd[1].clone());
                    }
                },
                "ls" => { list_mode = true; },
                _ => panic!()
            }
        } else if list_mode {
            let file = line.split_whitespace().map(|s| s.to_string()).collect::<Vec<String>>();
            if !file[0].eq("dir") {
                let size = file[0].parse::<usize>().unwrap();
                (0..path.len()).for_each(|i| {
                    *all_sizes.entry(path[..=i].join("/")).or_insert_with(|| 0) += size;
                });
            }
        }
    }

    let after0 = Instant::now();

    let start1 = Instant::now();

    let res1: usize = all_sizes.iter().map(|(_,&size)| size).filter(|&size| size <= 100_000).sum();

    let after1 = Instant::now();
    if print_result {
        println!("Part 1: {}", res1);
    }

    let start2 = Instant::now();

    let need_to_free = all_sizes["/"] -  (70_000_000 - 30_000_000);

    let res2: usize = all_sizes.iter().map(|(_,&size)| size).filter(|&size| size >= need_to_free).min().unwrap();

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

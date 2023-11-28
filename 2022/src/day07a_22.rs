use super::tools;
use std::time::Instant;
use std::collections::HashMap;

fn calculate_sizes(path: &String, parent: &HashMap<String,String>,sizes:&mut HashMap<String,usize>) -> usize {
    let mut s = 0;

    let children = parent.iter().filter(|(_,t)| t.eq(&path)).map(|(f,_)| f.clone()).collect::<Vec<String>>();
    children.iter().for_each(|child| {
        s += calculate_sizes(&child, parent, sizes);
    });
    *sizes.entry(path.clone()).or_insert_with(||0) += s;
    sizes[path]
}

#[allow(dead_code)]
pub fn run(real: bool, print_result: bool) -> (u128, u128, u128) {
    let start0 = Instant::now();

    let input_file: &str = if !real {
        "./input/day07_22_test.txt"
    } else {
        "./input/day07_22_real.txt"
    };
    let input = tools::get_input(String::from(input_file));

    let mut path: Vec<&str> = vec![];
    let mut sizes: HashMap<String,usize> = HashMap::new();
    let mut parent: HashMap<String,String> = HashMap::new();

    for line in &input {
        if line.contains("$") {
            if &line[2..4] == "cd" {
                if &line[5..] == ".." {
                    path.pop();
                } else {
                    let was = path.join("/");
                    path.push(&line[5..]);
                    parent.insert(path.join("/"), was);
                }
            }
        } else {
            let file = line.split_whitespace().collect::<Vec<&str>>();
            if let Ok(size) = file[0].parse::<usize>() {
                *sizes.entry(path.join("/")).or_insert_with(|| 0) += size;
            }
        }
    }

    calculate_sizes(&"/".to_string(),&parent, &mut sizes);

    let after0 = Instant::now();

    let start1 = Instant::now();

    let res1: usize = sizes.iter().map(|(_,&size)| size).filter(|&size| size <= 100_000).sum();

    let after1 = Instant::now();
    if print_result {
        println!("Part 1: {}", res1);
    }

    let start2 = Instant::now();

    let need_to_free = sizes["/"] -  (70_000_000 - 30_000_000);

    let res2: usize = sizes.iter().map(|(_,&size)| size).filter(|&size| size >= need_to_free).min().unwrap();

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

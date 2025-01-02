use super::tools;
use std::time::Instant;
use std::collections::HashMap;

#[allow(dead_code)]
pub fn run(real: bool, print_result: bool) -> (u128, u128, u128) {
    let start0 = Instant::now();

    let input_file: &str = if !real {
        "./input/day09-test.txt"
    } else {
        "./input/day09-real.txt"
    };
    let input = tools::get_input(String::from(input_file));
    let numbers = input[0].chars().map(|c| c.to_string().parse::<i64>().unwrap()).collect::<Vec<i64>>();

    let after0 = Instant::now();

    let start1 = Instant::now();

    let mut file_id = 0;
    let mut disk = numbers.iter().enumerate().fold(vec![], |mut acc, (i, v)| {
        if i%2 == 0 {
            for _ in 0..(*v as usize) { acc.push(file_id) }
            file_id += 1;
        } else {
            for _ in 0..(*v as usize) { acc.push(-1) }
        }
        acc
    });

    let free_indices = disk.iter().enumerate().filter(|(_, v)| **v == -1).map(|(i, _)| i).collect::<Vec<usize>>();

    free_indices.iter().for_each(|fi| {
        while disk.last().unwrap() == &-1 { disk.pop(); }
        if disk.len() <= *fi { return; }
        disk[*fi] = disk.pop().unwrap();
    });

    let res1 = disk.iter().enumerate().fold(0, |acc, (i, v)| {
        acc + *v * i as i64
    });

    let after1 = Instant::now();
    if print_result {
        println!("Part 1: {}", res1);
    }

    let start2 = Instant::now();

    let mut files: HashMap<i64, (i64, i64)> = HashMap::new();
    let mut free = vec![];
    let mut file_id = 0;
    let mut pos = 0;
    numbers.iter().enumerate().for_each(|(i, v)| {
        if i%2 == 0 {
            files.insert(file_id, (pos, *v));
            file_id += 1;
        } else {
            free.push((pos, *v));
        }
        pos += v;
    });

    while file_id > 0 {
        file_id -= 1;
        let (pos, size) = *files.get(&file_id).unwrap();
        for i in 0..free.len() {
            let (fpos, fsize) = free[i];
            if fpos >= pos {
                free[i] = (-1, -1);
                break;
            }
            if size <= fsize {
                files.insert(file_id,(fpos, size));
                free[i] = (fpos + size, (fsize - size));
                break;
            }
        }
    }

    let res2 = files.iter().fold(0, |acc, (file_id, (pos, size))| {
        let mut res = acc;
        for v in (*pos as usize)..(*pos as usize + *size as usize) {
            res += *file_id * v as i64;
        }
        res
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

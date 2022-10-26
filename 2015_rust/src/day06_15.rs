use super::tools;
use std::time::Instant;

#[allow(dead_code)]
pub fn run(real: bool, print_result: bool) -> (u128, u128, u128) {
    let start0 = Instant::now();

    let input_file: &str = if !real {
        "./input/day06_15_test.txt"
    } else {
        "./input/day06_15_real.txt"
    };
    let input = tools::get_input(String::from(input_file));

    let mut lights1: Vec<Vec<bool>> = vec![vec![false;1000];1000];
    let mut lights2: Vec<Vec<i32>> = vec![vec![0;1000];1000];


    let after0 = Instant::now();

    let start1 = Instant::now();

    for line in &input {
        let mut cmd = line.split_whitespace();

        let op = cmd.next().unwrap();
        let mut onoff = "";
        if op == "turn" {
            onoff = cmd.next().unwrap();
        }

        let from = cmd.next().unwrap(); cmd.next();
        let to = cmd.next().unwrap();

        let mut from_coords = from.split(",");
        let mut to_coords = to.split(",");

        let x1 = from_coords.next().unwrap().parse::<usize>().unwrap();
        let y1 = from_coords.next().unwrap().parse::<usize>().unwrap();
        let x2 = to_coords.next().unwrap().parse::<usize>().unwrap();
        let y2 = to_coords.next().unwrap().parse::<usize>().unwrap();

        match op {
            "toggle" => {
                for i in x1..=x2 { for j in y1..=y2 {
                    lights1[i][j] = !lights1[i][j];   
                    lights2[i][j] += 2;   
                }}
            },
            "turn" => {
                for i in x1..=x2 { for j in y1..=y2 {
                    lights1[i][j] = onoff == "on";   
                    lights2[i][j] += if onoff == "on" {1} else {-1};   
                    if lights2[i][j] < 0 { lights2[i][j] = 0; }
                }}
            },
            _ => panic!("oops"),
        }
    }

    let res1 = lights1.iter().flatten().collect::<Vec<&bool>>().iter().filter(|&&l| *l).count();

    let after1 = Instant::now();
    if print_result {
        println!("Part 1: {}", res1);
    }

    let start2 = Instant::now();

    let res2: i32 = lights2.iter().flatten().collect::<Vec<&i32>>().iter().map(|&&v| v).sum();

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

use super::tools;
use std::time::Instant;
use std::collections::BTreeSet;

#[allow(dead_code)]
pub fn run(real: bool, print_result: bool) -> (u128, u128, u128) {
    let start0 = Instant::now();

    let input_file: &str = if !real {
        "./input/day13_21_test.txt"
    } else {
        "./input/day13_21_real.txt"
    };
    let input = tools::get_input(String::from(input_file));

    let mut pixels: BTreeSet<(i64,i64)> = BTreeSet::new();
    let mut folds: Vec<(String, i64)> = vec![];

    let mut lc = 0;
    loop {
        if input[lc] == "" { lc+=1; break; }
        let coor = input[lc].split(",").map(|s| s.parse::<i64>().unwrap()).collect::<Vec<i64>>();
        pixels.insert((coor[0], coor[1]));
        lc += 1;
    }

    while lc < input.len() {
        let axis = input[lc][11..].to_string();
        let mut tokens = axis.split("=");
        folds.push((tokens.next().unwrap().to_string().clone(), tokens.next().unwrap().parse::<i64>().unwrap())); 
        lc += 1;
    }

    let fold = | pxls: &BTreeSet<(i64,i64)>, (axis, value): &(String, i64) | -> BTreeSet<(i64,i64)> {
        pxls
            .iter()
            .map(|pxl| {
                match axis.as_str() {
                    "x" => {
                        if pxl.0 > *value { return (2 * value - pxl.0, pxl.1); } else { return *pxl; }
                    }
                    "y" => {
                        if pxl.1 > *value { return (pxl.0, 2 * value - pxl.1); } else { return *pxl; }
                    }
                    _ => panic!()
                }
            }).collect::<BTreeSet<(i64,i64)>>()
    };
    
    let after0 = Instant::now();

    let start1 = Instant::now();

    pixels = fold(&pixels, &folds[0]);

    let res1 = pixels.len();

    let after1 = Instant::now();
    if print_result {
        println!("Part 1: {}", res1);
    }

    let start2 = Instant::now();

    for i in 1..folds.len() {
        pixels = fold(&pixels, &folds[i]);        
    }

    if print_result {
        for i in 0..6i64 { for j in 0..39i64 { 
            print!("{}", if pixels.contains(&(j,i)) { "#" } else { " " });
        } println!(""); }
    }

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

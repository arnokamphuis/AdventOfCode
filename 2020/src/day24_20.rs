use super::tools;
use std::time::Instant;
use std::collections::HashSet;

fn step(floor: &HashSet<(i32,i32)>) -> HashSet<(i32,i32)>  {
    let mut new_floor: HashSet<(i32,i32)> = HashSet::new();
    let mut to_check: HashSet<(i32,i32)> = HashSet::new();

    let directions = vec![(-1,0),(1,0),(0,1),(0,-1),(-1,-1),(1,1)];
    for tile in floor {
        for &d in directions.iter() {
            to_check.insert((tile.0+d.0, tile.1+d.1));
        }
    }
    for tile in to_check {
        let mut c = 0;
        'inner: for &d in directions.iter() {
            if floor.contains(&(tile.0+d.0, tile.1+d.1)) {
                c+=1; 
            }
            if c > 2 { break 'inner; }
        }

        let current_black = floor.contains(&tile);
        let mut new_black = current_black;
        match current_black {
            true =>  { if c == 0 || c > 2 { new_black = false; } },
            false => { if c==2            { new_black = true; } }
        }

        if new_black {
            new_floor.insert(tile);
        }
    }

    new_floor
}

#[allow(dead_code)]
pub fn run(real: bool, print_result: bool) -> (u128, u128, u128) {
    let start0 = Instant::now();

    let input_file: &str = if !real {
        "./input/day24_20_test.txt"
    } else {
        "./input/day24_20_real.txt"
    };
    let input = tools::get_input(String::from(input_file));


    let after0 = Instant::now();

    let start1 = Instant::now();

    let mut floor: HashSet<(i32,i32)> = HashSet::new();
    for line in &input {
        let mut c_iter = line.chars();
        let mut x = 0;
        let mut y = 0;
        while let Some(c) = c_iter.next() {
            match c {
                'e' => { x -= 1; }
                'w' => { x += 1; }
                _ => {
                    let nc = c_iter.next().unwrap();
                    match format!("{}{}",c,nc).as_str() {
                        "ne" => { y += 1; }
                        "se" => { x -= 1; y -= 1; }
                        "nw" => { x += 1; y += 1; }
                        "sw" => { y -= 1; }
                        _ => {}
                    }
                }
            }
        }
        if floor.contains(&(x,y)) {
            floor.remove(&(x,y));
        } else {
            floor.insert((x,y));
        }
    }

    let res1 = floor.len();

    let after1 = Instant::now();
    if print_result {
        println!("Part 1: {}", res1);
    }

    let start2 = Instant::now();

    let mut new_floor = floor.clone();
    for _ in 0..100 {
        new_floor = step(&new_floor);
    }
    let res2 = new_floor.len();


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

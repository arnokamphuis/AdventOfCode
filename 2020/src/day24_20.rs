use super::tools;
use std::time::Instant;
use std::collections::HashMap;

fn step(floor: &HashMap<(i32,i32), bool>) -> HashMap<(i32,i32), bool>  {
    let minmax = floor
        .iter()
        .fold(((i32::MAX, i32::MAX), (i32::MIN, i32::MIN)), 
            |acc, (&coor,_)| {
                (( std::cmp::min(acc.0.0, coor.0), std::cmp::min(acc.0.1, coor.1)  ),  
                 ( std::cmp::max(acc.1.0, coor.0), std::cmp::max(acc.1.1, coor.1)  ))
        });

    let mut new_floor = HashMap::new();

    for x in minmax.0.0-1..minmax.1.0+2 {
        for y in minmax.0.1-1..minmax.1.1+2 {
            let mut c = 0;
            'inner: for &d in vec![(-1,0),(1,0),(0,1),(0,-1),(-1,-1),(1,1)].iter() {
                if *floor.get(&(x+d.0, y+d.1)).or(Some(&false)).unwrap() {
                    c+=1; 
                }
                if c > 2 { break 'inner; }
            }

            let current_black = *floor.get(&(x,y)).or(Some(&false)).unwrap();
            let mut new_black = current_black;
            match current_black {
                true =>  { if c == 0 || c > 2 { new_black = false; } },
                false => { if c==2            { new_black = true; } }
            }

            if new_black {
                *new_floor.entry((x,y)).or_insert(new_black) = new_black;
            }
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

    let mut floor: HashMap<(i32,i32), bool> = HashMap::new();
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
        let tile = floor.entry((x,y)).or_insert(false);
        *tile = !*tile;
    }


    let count_black = |floor: &HashMap<(i32,i32),bool>| -> i32 {
        floor.iter().fold(0, |acc, v| acc + *v.1 as i32 )        
    };

    let res1 = count_black(&floor);

    let after1 = Instant::now();
    if print_result {
        println!("Part 1: {}", res1);
    }

    let start2 = Instant::now();

    let mut new_floor = floor.clone();
    for _ in 0..100 {
        new_floor = step(&new_floor);
    }
    let res2 = count_black(&new_floor);


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

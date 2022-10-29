use super::tools;
use std::time::Instant;
use std::collections::HashMap;

#[allow(dead_code)]
pub fn run(real: bool, print_result: bool) -> (u128, u128, u128) {
    let start0 = Instant::now();

    let input_file: &str = if !real {
        "./input/day18_15_test.txt"
    } else {
        "./input/day18_15_real.txt"
    };
    let input = tools::get_input(String::from(input_file));

    let mut size = 0i32;
    let mut lights: HashMap<(i32,i32), bool> = HashMap::new();
    input.iter().enumerate().for_each(|(y,line)| {
        size = line.len() as i32;
        line.chars().enumerate().for_each(|(x,c)| {
            lights.insert((x as i32,y as i32),c=='#');
        });
    });

    let mut lights2 = lights.clone();

    let break_lights = | size: i32, lights: &mut HashMap<(i32,i32),bool> | {
        *lights.get_mut(&(0,0)).unwrap() = true;
        *lights.get_mut(&(0,size-1)).unwrap() = true;
        *lights.get_mut(&(size-1,size-1)).unwrap() = true;
        *lights.get_mut(&(size-1,0)).unwrap() = true;
    };

    let update = | size: i32, lights: &HashMap<(i32,i32),bool>, broken: bool | -> HashMap<(i32,i32),bool> {
        let mut new_lights = lights.clone();

        for x in 0i32..size {
            for y in 0i32..size {
                let mut count = 0;
                let current_light: bool = *lights.get(&(x,y)).unwrap();
                
                for dx in -1..=1 {
                    for dy in -1..=1 {
                        if !(dx == 0 && dy == 0) {
                            if let Some(neighbor) = lights.get(&(x+dx, y+dy)) {
                                count += *neighbor as i32;
                            }
                        }
                    }
                }

                *new_lights.get_mut(&(x,y)).unwrap() = match (current_light, count) {
                    (true,2) | (true,3) | (false,3) => { true },
                    _ => { false }
                }
            }
        }

        if broken {
            break_lights(size, &mut new_lights);
        }
        new_lights
    };

    let after0 = Instant::now();

    let start1 = Instant::now();

    (0..100).for_each(|_| {
        lights = update(size, &lights, false);
    });

    let res1 = lights.iter().filter(|(_,&n)| n ).count();

    let after1 = Instant::now();
    if print_result {
        println!("Part 1: {}", res1);
    }

    let start2 = Instant::now();

    break_lights(size, &mut lights2);
    (0..100).for_each(|_| {
        lights2 = update(size, &lights2, true);
    });

    let res2 = lights2.iter().filter(|(_,&n)| n ).count();

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

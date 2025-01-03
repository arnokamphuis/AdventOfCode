use super::tools;
use std::time::Instant;
use std::collections::{HashSet,HashMap};

#[cfg(feature = "make_movie")]
use crate::tools::Image;

fn not_in_map(pos: &(i32, i32), map: &Vec<Vec<char>>) -> bool {
    pos.0 < 0 || pos.1 < 0 || pos.0 >= map[0].len() as i32 || pos.1 >= map.len() as i32
}

#[allow(dead_code)]
pub fn run(real: bool, print_result: bool) -> (u128, u128, u128) {
    let start0 = Instant::now();

    let input_file: &str = if !real {
        "./input/day06-test.txt"
    } else {
        "./input/day06-real.txt"
    };
    let input = tools::get_input(String::from(input_file));

    let mut obstacles: HashSet<(i32, i32)> = HashSet::new();
    let mut map: Vec<Vec<char>> = vec![];
    let mut start_pos = (0, 0);
    let mut start_dir = 3;
    for (r, line) in input.iter().enumerate() {
        let mut v = vec![];
        for (c, ch) in line.chars().enumerate() {
            match ch {
                '^' => { start_dir = 3; start_pos = (c as i32, r as i32); },
                '>' => { start_dir = 0; start_pos = (c as i32, r as i32); },
                'v' => { start_dir = 1; start_pos = (c as i32, r as i32); },
                '<' => { start_dir = 2; start_pos = (c as i32, r as i32); },
                '#' => { obstacles.insert((c as i32, r as i32)); },
                _ => {}
            };
            v.push(ch);
        }
        map.push(v);
    }

    let dirs = HashMap::from([
        (0, (1,0)), (1, (0,1)), (2, (-1,0)), (3, (0,-1))
    ]);

    let after0 = Instant::now();

    let start1 = Instant::now();

    #[cfg(feature = "make_movie")]
    let mut img: Image = Image::new(input.len(), input[0].len(), 4);
    #[cfg(feature = "make_movie")]
    let mut img_count = 0;
    #[cfg(feature = "make_movie")]
    img.clear((0, 0, 0, 255));

    let mut visited: HashSet<(i32, i32)> = HashSet::new();
    let mut guard_pos = start_pos;
    let mut guard_dir = start_dir;

    #[cfg(feature = "make_movie")]
    {
        img.set_pixel(guard_pos.0 as usize, guard_pos.1 as usize, (255, 255, 255, 255));
        img.save_png(&format!("images/day06_24_{:06}.png", img_count));
    }

    loop {
        let new_pos = (guard_pos.0 + dirs[&guard_dir].0, guard_pos.1 + dirs[&guard_dir].1);
        if obstacles.contains(&new_pos) {
            guard_dir = (guard_dir + 1) % 4;
            #[cfg(feature = "make_movie")]
            {
                img_count += 1;
                img.set_pixel(new_pos.0 as usize, new_pos.1 as usize, (255, 0, 0, 255));
                img.save_png(&format!("images/day06_24_{:06}.png", img_count));
            }
        } else {
            if not_in_map(&new_pos, &map) {
                break;
            }
            guard_pos = new_pos;
            visited.insert(guard_pos);
            #[cfg(feature = "make_movie")]
            {
                img_count += 1;
                img.set_pixel(guard_pos.0 as usize, guard_pos.1 as usize, (255, 255, 255, 255));
                img.save_png(&format!("images/day06_24_{:06}.png", img_count));
            }
        }
    }
    let res1 = visited.len();

    let after1 = Instant::now();
    if print_result {
        println!("Part 1: {}", res1);
    }

    let start2 = Instant::now();

    let mut res2 = 0;
    for &(o_r, o_c) in visited.iter() {
        if obstacles.contains(&(o_r, o_c)) || (o_r, o_c) == start_pos {
            continue;
        }
        let mut guard_pos = start_pos;
        let mut guard_dir = start_dir;
        let mut visited: HashSet<((i32, i32), i32)> = HashSet::new();
        loop {
            if visited.contains(&(guard_pos, guard_dir)) {
                #[cfg(feature = "make_movie")]
                {
                    img.set_pixel(o_r as usize, o_c as usize, (0, 0, 255, 255));
                    (0..10).for_each(|_| { img_count += 1; img.save_png(&format!("images/day06_24_{:06}.png", img_count)); });
                }
                res2 += 1;
                break;
            }
            visited.insert((guard_pos, guard_dir));
            let new_pos = (guard_pos.0 + dirs[&guard_dir].0, guard_pos.1 + dirs[&guard_dir].1);
            if not_in_map(&new_pos, &map) {
                break;
            }
            if obstacles.contains(&new_pos) || new_pos == (o_r, o_c) {
                guard_dir = (guard_dir + 1) % 4;
            } else {
                guard_pos = new_pos;
            }
        }
    }

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

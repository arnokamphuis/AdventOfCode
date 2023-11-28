use super::tools;
use std::time::Instant;
use std::collections::HashMap;
use tools::Image;

#[allow(dead_code)]
pub fn run(real: bool, print_result: bool) -> (u128, u128, u128) {
    let start0 = Instant::now();

    let input_file: &str = if !real {
        "./input/day14_22_test.txt"
    } else {
        "./input/day14_22_real.txt"
    };
    let input = tools::get_input(String::from(input_file));

    let mut cave = input
        .iter()
        .map(|line| line
            .split(" -> ")
            .map(|s| s
                .split(',')
                .map(|v| v.parse::<i64>().unwrap())
                .collect::<Vec<i64>>()
            )
            .collect::<Vec<_>>()
            .windows(2)
            .map(|p| {
                let dx = if p[1][0]-p[0][0] > 0 { 1 } else if p[1][0]-p[0][0] < 0 { -1 } else { 0 };
                let dy = if p[1][1]-p[0][1] > 0 { 1 } else if p[1][1]-p[0][1] < 0 { -1 } else { 0 };
                let d = (p[0][0]-p[1][0]).abs() + (p[0][1]-p[1][1]).abs();
                (0..=d).map(|i|
                    ( (p[0][0] + i * dx, p[0][1] + i * dy), '#'  )
                ).collect::<Vec<((i64,i64),char)>>()
            })
            .collect::<Vec<_>>()
            .into_iter()
            .flatten()
            .collect::<Vec<_>>()
        )
        .collect::<Vec<_>>()
        .into_iter()
        .flatten()
        .collect::<HashMap<(i64,i64),char>>();

    let mut counter = 0;
    let min_x = cave.iter().min_by(|a,b| a.0.0.cmp(&b.0.0)).unwrap().0.0;
    let max_x = cave.iter().max_by(|a,b| a.0.0.cmp(&b.0.0)).unwrap().0.0;
    let min_y = -1;//cave.iter().min_by(|a,b| a.0.1.cmp(&b.0.1)).unwrap().0.1;
    let max_y = cave.iter().max_by(|a,b| a.0.1.cmp(&b.0.1)).unwrap().0.1;
    let width = max_x - min_x + 20;
    let height = max_y - min_y + 20;

    let draw_cave = | cave: &HashMap<(i64,i64),char>, cur_pos: (i64,i64), counter: usize | {
        let mut img: Image = Image::new(width as usize, height as usize, 4);
        img.clear((0,0,0,255));
        cave.iter().for_each(|((x,y),c)| {
            if 0 <= (x-min_x+10) && (x-min_x+10) < width && 0 <= (y-min_y+10) && (y-min_y+10) <= height {
            img.set_pixel((x-min_x+10) as usize, (y-min_y+10) as usize,
                match c {
                    '#' => (255,255,255,255),
                    'o' => (255,255,  0,255),
                    _ => {(0,0,0,255)}
                })
            }
        });
        if 0 <= (cur_pos.0-min_x+10) && (cur_pos.0-min_x+10) < width && 0 <= (cur_pos.1-min_y+10) && (cur_pos.1-min_y+10) <= height {
            img.set_pixel((cur_pos.0-min_x+10) as usize, (cur_pos.1-min_y+10) as usize,(255,  0,255,255));
        }
        img.set_pixel((500-min_x+10) as usize, (0-min_y+10) as usize,(255,0,0,255));
        img.save_png(&format!("images/day14_22/cave_{:07}.png", counter));    
    };

    draw_cave(&cave,(500,0),counter); counter+=1;

    let initial_cave = cave.clone();

    let after0 = Instant::now();

    let start1 = Instant::now();

    let move_grain = | p: (i64,i64), cave: &mut HashMap<(i64,i64),char> | -> Option<(i64,i64)> {
        let mut new_pos = (p.0, p.1+1);
        if cave.get(&new_pos) != None { // directly below is taken
            new_pos.0 -= 1;
            if cave.get(&new_pos) != None { // down and left is taken
                new_pos.0 += 2;
                if cave.get(&new_pos) != None { // down and left is taken
                    cave.insert(p, 'o');
                    return None;
                }
            }
        }

        if new_pos.1 == max_y+1 {
            cave.insert(new_pos,'o');
            return None;
        }
        Some(new_pos)
    };

    let mut grain = 1;
    'outer: loop {
        let mut pos = (500i64,0i64);
        while let Some(p) = move_grain(pos, &mut cave) {
            draw_cave(&cave, p, counter); counter+=1;
            pos = p;
            if pos.1 >= max_y {
                break 'outer;
            }
        }
        grain+=1;
    }
    let res1 = grain-1;

    let after1 = Instant::now();
    if print_result {
        println!("Part 1: {}", res1);
    }

    let start2 = Instant::now();

    cave = initial_cave.clone();
    let mut grain = 1;
    'outer: loop {
        let mut pos = (500i64,0i64);
        while let Some(p) = move_grain(pos, &mut cave) {
            pos = p;
        }
        if pos == (500,0) {
            break 'outer;
        }
        grain+=1;
    }
    let res2 = grain;

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

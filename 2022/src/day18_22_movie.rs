use super::tools;
use std::time::Instant;
use std::collections::HashSet;
use queues::*;
use tools::Image;

#[allow(dead_code)]
pub fn run(real: bool, print_result: bool) -> (u128, u128, u128) {
    let start0 = Instant::now();

    let input_file: &str = if !real {
        "./input/day18_22_test.txt"
    } else {
        "./input/day18_22_real.txt"
    };
    let input = tools::get_input(String::from(input_file));
    
    let cubes = input
        .iter()
        .map(|line| {
            line.split(",").map(|s| s.parse::<i64>().unwrap()).collect::<Vec<_>>()
        }).collect::<HashSet<Vec<i64>>>();
    let mut minimums = vec![i64::MAX;3];
    cubes.iter().for_each(|cube| for i in 0..3 { minimums[i] = minimums[i].min(cube[i]-1); });
    let mut maximums = vec![0;3];
    cubes.iter().for_each(|cube| for i in 0..3 { maximums[i] = maximums[i].max(cube[i]+1); });

    let width_layers = 7;
    let height_layers = 3;
    let width = 21;
    let height = 22;
    let margin = 4;

    let total_width = width * width_layers + margin * (width_layers + 1);
    let total_height = height * height_layers + margin * (height_layers + 1);
    let mut img: Image = Image::new(total_width, total_height, 8);

    let draw_image = | img: &mut Image, shape: &HashSet<Vec<i64>>, search_shape: &HashSet<Vec<i64>>, counter: &mut usize | {
        img.clear((0,0,0,255));
        for x in 0..=width_layers {  for y in 0..total_height { for m in 0..margin { img.set_pixel(x * (width+margin) + m ,y,(255,255,255,255)); } } }
        for y in 0..=height_layers { for x in 0..total_width { for m in 0..margin { img.set_pixel(x, y * (height+margin) + m,(255,255,255,255)); } } }

        search_shape.iter().for_each(|cube| {
            let x_z = margin + (margin + width)  * ((cube[2] - minimums[2]) as usize % width_layers);
            let y_z = margin + (margin + height) * ((cube[2] - minimums[2]) as usize / width_layers);

            let x_l = (cube[0] - minimums[0]) as usize;
            let y_l = (cube[1] - minimums[1]) as usize;

            img.set_pixel(x_l + x_z, y_l + y_z, (0,0,255,255));
        });

        shape.iter().for_each(|cube| {
            let x_z = margin + (margin + width)  * ((cube[2] - minimums[2]) as usize % width_layers);
            let y_z = margin + (margin + height) * ((cube[2] - minimums[2]) as usize / width_layers);

            let x_l = (cube[0] - minimums[0]) as usize;
            let y_l = (cube[1] - minimums[1]) as usize;

            img.set_pixel(x_l + x_z, y_l + y_z, (255,0,0,255));
        });

        img.save_png(&format!("images/day18_22/lava_{:07}.png", counter)); *counter += 1;
    };

    let after0 = Instant::now();

    let start1 = Instant::now();

    let dirs = vec![(0,0,1),(0,0,-1),(0,1,0),(0,-1,0),(1,0,0),(-1,0,0)];

    let mut counter = 0;
    draw_image(&mut img, &cubes, &cubes, &mut counter);

    let res1 = cubes.iter().fold(0, |resacc, cube| {
        resacc + dirs.iter().fold(0, |acc, d| {
            acc + (!cubes.contains(&vec![cube[0]+d.0, cube[1]+d.1, cube[2]+d.2]) as usize)
        })
    });

    assert!(res1 == if !real {64} else {3610}, "Part 1 if not correct!");

    let after1 = Instant::now();
    if print_result {
        println!("Part 1: {}", res1);
    }

    let start2 = Instant::now();

    let mut q: Queue<Vec<i64>> = queue![];
    let mut all: HashSet<Vec<i64>> = HashSet::new();
    for x in minimums[0]..=maximums[0] {
        for y in minimums[1]..=maximums[1] {
            for z in minimums[2]..=maximums[2] {
                let pos = vec![x,y,z];
                if !cubes.contains(&pos) { all.insert(pos); }
            }                
        }
    }

    let current = vec![minimums[0],minimums[1],minimums[2]];
    assert!(!cubes.contains(&current));
    assert_eq!(q.add(current), Ok(None));
    while let Ok(next) = q.remove() {
        draw_image(&mut img, &cubes, &all, &mut counter);
        for d in &dirs {
            let loc = vec![next[0]+d.0, next[1]+d.1, next[2]+d.2];
            if loc.iter().enumerate().all(|(i,&v)| v >= minimums[i] && v <= maximums[i] ) {
                if all.contains(&loc) {
                    all.remove(&loc);
                    assert_eq!(q.add(loc), Ok(None));
                }
            }
        }
    }

    let res2 = res1 - all.iter().fold(0, |resacc, cube| {
        resacc + dirs.iter().fold(0, |acc, d| {
            acc + (!all.contains(&vec![cube[0]+d.0, cube[1]+d.1, cube[2]+d.2]) as usize)
        })
    });

    assert!(res2 == if !real {58} else {2082}, "Part 2 if not correct!");

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

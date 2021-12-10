use super::tools;
use std::time::Instant;
use queues::*;
use std::collections::BTreeSet;
use std::collections::HashMap;
use itertools::Itertools;
use tools::Image;

#[allow(dead_code)]
pub fn run(real: bool, print_result: bool) -> (u128, u128, u128) {
    let start0 = Instant::now();

    let input_file: &str = if !real {
        "./input/day09_21_test.txt"
    } else {
        "./input/day09_21_real.txt"
    };
    let input = tools::get_input(String::from(input_file));

    let heightmap: HashMap<(i32,i32),i32> = input.iter().enumerate().fold(HashMap::new(),|mut hm, (i,line)| {
        line.chars().enumerate().for_each(|(j,c)| { hm.insert((i as i32, j as i32), c.to_digit(10).unwrap() as i32);} ); hm 
    });

    let mut lowpoints: Vec<(i32,i32)> = vec![];
    let w = *heightmap.keys().map(|(_,y)| y).max().unwrap() as i32;
    let h = *heightmap.keys().map(|(x,_)| x).max().unwrap() as i32;

    let mut img: Image = Image::new(w as usize + 1, h as usize + 1, 10);
    img.clear((0,0,0,255));
    heightmap.iter().filter(|p| *p.1 == 9).for_each(|((x,y),_)| {
        img.set_pixel(*x as usize, *y as usize, (0,0,255,255));
    });
    let mut imgcount = 0;

    let after0 = Instant::now();

    let start1 = Instant::now();

    let mut res1: i32 = 0;
    for i in 0..=h {
        for j in 0..=w {
            if let Some(height) = heightmap.get(&(i,j)) {
                let dirs: [(i32,i32);4] = [(0,-1), (0,1), (1,0), (-1,0)];
                let mut count = 0;
                for d in &dirs {
                    if let Some(nbheight) = heightmap.get(&(i+d.0, j+d.1)) {
                        count += if nbheight > height { 1 } else { 0 };
                    } else {
                        count += 1;
                    }
                }
                if count == 4 {
                    lowpoints.push((i,j));
                    res1 += height+1;
                }         
            }
        }
    }

    let after1 = Instant::now();
    if print_result {
        println!("Part 1: {}", res1);
    }

    let start2 = Instant::now();

    let inside = |p: (i32,i32), hm: &HashMap<(i32,i32),i32>| -> bool {
        if let Some(&h) = hm.get(&p) { h < 9 } else { false }
    };

    let res2 = lowpoints.iter().map(|point| {
        let mut q: Queue<(i32,i32)> = queue![];
        let mut visited: BTreeSet<(i32,i32)> = BTreeSet::new();

        let current = (point.0 as i32, point.1 as i32);
        assert_eq!(q.add(current), Ok(None));
        visited.insert(current);

        img.set_pixel(point.0 as usize, point.1 as usize, (255,255,0,255));
        img.save_png(&format!("images/day09-{:05}.png",imgcount));
        imgcount += 1;

        while let Ok(next) = q.remove() {
            let dirs: [(i32,i32);4] = [(0,-1), (0,1), (1,0), (-1,0)];
            for d in &dirs {
                let p = ( next.0 + d.0 , next.1 + d.1 );
                if inside(p, &heightmap) && !visited.contains(&p) {
                    assert_eq!(q.add(p), Ok(None));
                    visited.insert(p);
                    let pointheight = heightmap.get(&p).unwrap();
                    let color:u8 = (255*pointheight/8) as u8;
                    img.set_pixel(p.0 as usize, p.1 as usize, (255,255-color,0,255));
                    img.save_png(&format!("images/day09-{:05}.png",imgcount));
                    imgcount += 1;
                }   
            }
        }
        visited.len()
    })
    .sorted_by(|a, b| Ord::cmp(&b, &a))
    .collect::<Vec<usize>>()[0..3]
    .into_iter()
    .fold(1, |v, s| v*s);

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

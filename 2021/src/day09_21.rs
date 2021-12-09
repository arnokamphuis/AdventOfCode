use super::tools;
use std::time::Instant;
use queues::*;
use std::collections::BTreeSet;

#[allow(dead_code)]
pub fn run(real: bool, print_result: bool) -> (u128, u128, u128) {
    let start0 = Instant::now();

    let input_file: &str = if !real {
        "./input/day09_21_test.txt"
    } else {
        "./input/day09_21_real.txt"
    };
    let input = tools::get_input(String::from(input_file));

    let heightmap: Vec<Vec<u8>> = input.iter().fold(vec![],|mut v, line| {
        v.push(line.chars().fold(vec![], |mut lv, c| {lv.push(c.to_digit(10).unwrap() as u8); lv})); v
    });
    
    let after0 = Instant::now();

    let start1 = Instant::now();

    let mut res1: u32 = 0;
    let w = heightmap[0].len();
    let h = heightmap.len();

    let mut lowpoints: Vec<(usize,usize)> = vec![];

    for i in 0..h {
        for j in 0..w {
            let mut count = 0;
            let height = heightmap[i][j];
            if i > 0 && i < (h-1) as usize && j > 0 && j < (w-1) as usize {
                let dirs: [(i8,i8);4] = [(0,-1), (0,1), (1,0), (-1,0)];
                for d in &dirs {
                    count += if heightmap[(i as i8 + d.0) as usize][(j as i8 + d.1) as usize] > height { 1 } else { 0 }
                }
            } else if i > 0 && i < (h-1) && j == 0 {
                count += 1;
                let dirs: [(i8,i8);3] = [(0,1), (1,0), (-1,0)];
                for d in &dirs {
                    count += if heightmap[(i as i8 + d.0) as usize][(j as i8 + d.1) as usize] > height { 1 } else { 0 }
                }
            } else if i > 0 && i < (h-1) && j == (w-1) as usize {
                count += 1;
                let dirs: [(i8,i8);3] = [(0,-1), (1,0), (-1,0)];
                for d in &dirs {
                    count += if heightmap[(i as i8 + d.0) as usize][(j as i8 + d.1) as usize] > height { 1 } else { 0 }
                }
            } else if j > 0 && j < (w-1) && i == 0 {
                count += 1;
                let dirs: [(i8,i8);3] = [(0,1), (0,-1), (1,0)];
                for d in &dirs {
                    count += if heightmap[(i as i8 + d.0) as usize][(j as i8 + d.1) as usize] > height { 1 } else { 0 }
                }
            } else if j > 0 && j < (w-1) && i == (h-1) as usize {
                count += 1;
                let dirs: [(i8,i8);3] = [(0,1), (0,-1), (-1,0)];
                for d in &dirs {
                    count += if heightmap[(i as i8 + d.0) as usize][(j as i8 + d.1) as usize] > height { 1 } else { 0 }
                }
            } else if j == 0 &&  i == 0 {
                count += 2;
                let dirs: [(i8,i8);2] = [(0,1), (1,0)];
                for d in &dirs {
                    count += if heightmap[(i as i8 + d.0) as usize][(j as i8 + d.1) as usize] > height { 1 } else { 0 }
                }
            } else if j == (w-1) as usize && i == 0 {
                count += 2;
                let dirs: [(i8,i8);2] = [(0,-1), (1,0)];
                for d in &dirs {
                    count += if heightmap[(i as i8 + d.0) as usize][(j as i8 + d.1) as usize] > height { 1 } else { 0 }
                }
            } else if j == (w-1) as usize && i == (h-1) as usize {
                count += 2;
                let dirs: [(i8,i8);2] = [(0,-1), (-1,0)];
                for d in &dirs {
                    count += if heightmap[(i as i8 + d.0) as usize][(j as i8 + d.1) as usize] > height { 1 } else { 0 }
                }
            } else if j == 0 && i == (h-1) as usize {
                count += 2;
                let dirs: [(i8,i8);2] = [(0,1), (-1,0)];
                for d in &dirs {
                    count += if heightmap[(i as i8 + d.0) as usize][(j as i8 + d.1) as usize] > height { 1 } else { 0 }
                }
            }
            

            if count == 4 {
                lowpoints.push((i,j));
                res1 += (height+1) as u32;
            }

        }
    }
    let after1 = Instant::now();
    if print_result {
        println!("Part 1: {}", res1);
    }

    let start2 = Instant::now();

    let inside = |(i,j): (i32,i32), hm: &Vec<Vec<u8>>| -> bool {
        i >= 0 && i < h as i32 && j >= 0 && j < w as i32 && hm[i as usize][j as usize] < 9
    };

    let mut sizes: Vec<usize> = vec![];
    for point in &lowpoints {
        let mut q: Queue<(i32,i32)> = queue![];
        let mut visited: BTreeSet<(i32,i32)> = BTreeSet::new();
        let current = (point.0 as i32, point.1 as i32);
        assert_eq!(q.add(current), Ok(None));
        visited.insert(current);
        while let Ok(next) = q.remove() {
            let dirs: [(i32,i32);4] = [(0,-1), (0,1), (1,0), (-1,0)];
            for d in &dirs {
                let p = ( next.0 + d.0 , next.1 + d.1 );
                if inside(p, &heightmap) && !visited.contains(&p) {
                    assert_eq!(q.add(p), Ok(None));
                    visited.insert(p);
                }   
            }
        }
        sizes.push(visited.len());
    }

    sizes.sort();
    let res2 = sizes[sizes.len()-3..].to_vec().iter().fold(1, |v, s| v*s);

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

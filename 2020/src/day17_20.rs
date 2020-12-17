use super::tools;
use std::time::Instant;

fn step1(grid: &Vec<Vec<Vec<bool>>>) -> Vec<Vec<Vec<bool>>> {
    let sz = grid.len() as i16 +2;
    let sy = grid[0].len() as i16 +2;
    let sx = grid[0][0].len() as i16 +2;

    let mut new_grid: Vec<Vec<Vec<bool>>> = vec![
        vec![
            vec![
                false; sx as usize
            ]; sy as usize
        ]; sz as usize
    ];

    for z in 0i16..sz {
        for y in 0i16..sy {
            for x in 0i16..sx {
                let mut count: usize = 0;
                for dx in -1i16..2 {
                    for dy in -1i16..2 {
                        for dz in -1i16..2 {
                            if !(dx==0 && dy==0 && dz==0) && ( (x+dx) > 0 && (x+dx) < (sx-1) && (y+dy) > 0 && (y+dy) < (sy-1) && (z+dz) > 0 && (z+dz) < (sz-1)) {
                                if grid[(z-1+dz) as usize][(y-1+dy) as usize][(x-1+dx) as usize] {
                                    count += 1;
                                }
                            }
                        }                
                    }  
                }
                let mut current = false;
                if x > 0 && x < sx-1 && y > 0 && y < sy-1 && z > 0 && z < sz-1 { current = grid[(z-1) as usize][(y-1) as usize][(x-1) as usize] }
                new_grid[z as usize][y as usize][x as usize] = (current && (count == 2 || count == 3)) || (!current && count == 3);
            }
        }
    }

    new_grid
}

fn step2(grid: &Vec<Vec<Vec<Vec<bool>>>>) -> Vec<Vec<Vec<Vec<bool>>>> {
    let sw = grid.len() as i16 +2;
    let sz = grid[0].len() as i16 +2;
    let sy = grid[0][0].len() as i16 +2;
    let sx = grid[0][0][0].len() as i16 +2;

    let mut new_grid: Vec<Vec<Vec<Vec<bool>>>> = vec![
        vec![
            vec![
                vec![
                    false; sx as usize
                ]; sy as usize
            ]; sz as usize
        ]; sw as usize
    ];

    for w in 0i16..sw {
        for z in 0i16..sz {
            for y in 0i16..sy {
                for x in 0i16..sx {
                    let mut count: usize = 0;
                    for dx in -1i16..2 {
                        for dy in -1i16..2 {
                            for dz in -1i16..2 {
                                for dw in -1i16..2 {
                                    if !(dx==0 && dy==0 && dz==0 && dw==0) && ( (x+dx) > 0 && (x+dx) < (sx-1) && (y+dy) > 0 && (y+dy) < (sy-1) && (z+dz) > 0 && (z+dz) < (sz-1) && (w+dw) > 0 && (w+dw) < (sw-1)) {
                                        if grid[(w-1+dw) as usize][(z-1+dz) as usize][(y-1+dy) as usize][(x-1+dx) as usize] {
                                            count += 1;
                                        }
                                    }
                                }
                            }                
                        }  
                    }
                    let mut current = false;
                    if x > 0 && x < sx-1 && y > 0 && y < sy-1 && z > 0 && z < sz-1 && w > 0 && w < sw-1 { current = grid[(w-1) as usize][(z-1) as usize][(y-1) as usize][(x-1) as usize] }
                    new_grid[w as usize][z as usize][y as usize][x as usize] = (current && (count == 2 || count == 3)) || (!current && count == 3);
                }
            }
        }
    }

    new_grid
}

#[allow(dead_code)]
pub fn run(real: bool, print_result: bool) -> (u128, u128, u128) {
    let start0 = Instant::now();

    let input_file: &str = if !real {
        "./input/day17_20_test.txt"
    } else {
        "./input/day17_20_real.txt"
    };
    let input = tools::get_input(String::from(input_file));

    let mut grid1: Vec<Vec<Vec<bool>>> = vec![vec![]];
    let mut grid2: Vec<Vec<Vec<Vec<bool>>>> = vec![vec![vec![]]];

    input.iter().for_each(|line| {
        let mut r: Vec<bool> = vec![];
        line.chars().for_each(|c| {
            r.push( c == '#' );
        });
        grid1[0].push(r.clone());
        grid2[0][0].push(r.clone());
    });
    
    let after0 = Instant::now();

    let start1 = Instant::now();

    let mut result1 = grid1.clone();
    (0..6).for_each(|_| {
        result1 = step1(&result1);
    });

    let res1 = result1.iter().fold(0, | acc, layer| 
        acc + layer.iter().fold(0, | acc2, row | 
            acc2 + row.iter().fold(0, |acc3, v| acc3 + if *v {1} else {0} )
        )
    );

    let after1 = Instant::now();
    if print_result {
        println!("Part 1: {}", res1);
    }

    let start2 = Instant::now();

    let mut result2 = grid2.clone();
    (0..6).for_each(|_| {
        result2 = step2(&result2);
    });

    let res2 = result2.iter().fold(0, | acc, layer| 
        acc + layer.iter().fold(0, | acc2, row | 
            acc2 + row.iter().fold(0, |acc3, col| 
                acc3 + col.iter().fold(0, |acc4, v| acc4 + if *v {1} else {0} ) 
            )
        )
    );

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

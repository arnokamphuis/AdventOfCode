use super::tools;
use std::time::Instant;
use std::collections::BTreeMap;

use tools::Image;

fn flip_code(v: u16) -> u16 {
    let mut res = 0;
    let mut t = v;
    for _ in 0..10 {
        res = (res << 1) + (t & 1);
        t = t >> 1;
    }
    res
}

fn save_puzzle_as_image(current_puzzle: &Vec<Vec<(u16,u16,u16,u16,i8)>>, counter: &usize, tiles: &BTreeMap<(u16,u16,u16,u16,i8), u32>, full_tiles: &BTreeMap<u32, Vec<Vec<char>>>) -> Image {
    let img_h = current_puzzle.len() * full_tiles.iter().next().unwrap().1.len();
    let img_w = current_puzzle[0].len() * full_tiles.iter().next().unwrap().1[0].len();
    let mut img = Image::new(img_w, img_h, 8);
    img.clear((0,0,0,255));
    for (row_nr, row) in current_puzzle.iter().enumerate() {
        for (col_nr, col) in row.iter().enumerate() {
            if let Some(&tile_id) = tiles.get(col) {
                let tile = get_correct_tile(tile_id, col.4, full_tiles);
                let h = tile.len();
                let w = tile[0].len();
                for (y,r) in tile.iter().enumerate() {
                    for (x, &c) in r.iter().enumerate() {
                        img.set_pixel(col_nr * w + x, row_nr * h + y, if c == '#' {(0,0,255,255)} else {(0,0,0,255)});
                    }
                }
            }
        }
    }
    img.save_png(&format!("images/puzzle-{:05}.png", counter).to_string());
    img
}

fn find_next(current_puzzle: &Vec<Vec<(u16,u16,u16,u16,i8)>>, options: &BTreeMap<(u16,u16,u16,u16,i8), u32>, x: usize, y: usize, target: usize, movie: bool, counter: &mut usize, tiles: &BTreeMap<(u16,u16,u16,u16,i8), u32>, full_tiles: &BTreeMap<u32, Vec<Vec<char>>>) -> Option<Vec<Vec<(u16,u16,u16,u16,i8)>>> {
    if y*target+x == target*target {
        if movie {
            save_puzzle_as_image(current_puzzle, counter, tiles, full_tiles);
            *counter += 1;
        }
        return Some(current_puzzle.clone());
    }

    let new_x = (x + 1)%target;
    let new_y = y + (x+1)/target;
    for (&tile, &id) in options {
        let fit_left = x==0 || ((x > 0) && (flip_code(tile.3) == current_puzzle[x-1][y].1));
        let fit_top  = y==0 || ((y > 0) && (flip_code(tile.0) == current_puzzle[x][y-1].2));

        if fit_left && fit_top {
            let mut new_puzzle = current_puzzle.clone();
            new_puzzle[x][y] = tile;

            let mut new_options = options.clone();
            let to_be_removed: BTreeMap<(u16,u16,u16,u16,i8),u32> = new_options.iter().filter(|(_,&tid)| tid==id).map(|(&v,&vid)| (v,vid)).collect();
            to_be_removed.iter().for_each(|(t,_)| { new_options.remove(t); } );

            if movie {
                save_puzzle_as_image(current_puzzle, counter, tiles, full_tiles);
                *counter += 1;
            }

            if let Some(res) = find_next(&new_puzzle, &new_options, new_x, new_y, target, movie, counter, tiles, full_tiles) {
                return Some(res);
            }
        }
    }
    None
}

fn get_correct_coor(coor: &(usize, usize), tile_orientation: i8, size: usize) -> (usize, usize) {
    let mut new_coor = (coor.0, coor.1);

    match tile_orientation {
        2  => { new_coor.0 = coor.1;      new_coor.1 = size-coor.0 } // ccw 90
        3  => { new_coor.0 = size-coor.0; new_coor.1 = size-coor.1 } // ccw 180
        4  => { new_coor.0 = size-coor.1; new_coor.1 = coor.0      } // ccw 270
        -1 => { new_coor.0 = size-coor.0; new_coor.1 = coor.1      } // flip
        -2 => { new_coor.0 = coor.1;      new_coor.1 = coor.0      } // flip & ccw 90
        -3 => { new_coor.0 = coor.0;      new_coor.1 = size-coor.1 } // flip & ccw 180
        -4 => { new_coor.0 = size-coor.1; new_coor.1 = size-coor.0 } // flip & ccw 270
        _ => {}
    }
    new_coor
}

fn get_correct_tile(tile_id: u32, tile_orientation: i8, tiles: &BTreeMap<u32, Vec<Vec<char>>> ) -> Vec<Vec<char>> {
    let mut res = vec![vec![' '; 8]; 8];
    let original = tiles.get(&tile_id).unwrap();
    for r in 0..8usize {
        for c in 0..8usize {
            let (nr,nc) = get_correct_coor(&(r,c), tile_orientation, 8-1);
            res[nr][nc] = original[r][c];
        }
    }
    res
}

fn find_monster_in_image(sea_monster: &Vec<Vec<char>>, image: &Vec<Vec<char>>, tile_orientation: i8, movie: bool, counter: &mut usize) -> usize {
    let mut count = 0;

    let inner_height = sea_monster.len();
    let inner_width = sea_monster[0].len();
    let height = image.len() - sea_monster.len();
    let width = image[0].len() - sea_monster[0].len();

    let mut img: Image = Image::new(0,0,0);
    if movie {
        let w = image.len();
        let h = image[0].len();
        img = Image::new(w, h, 8);
        for r in 0..h {
            for c in 0..w {
                let coor = (r,c);
                img.set_pixel(coor.1, coor.0, if image[coor.0][coor.1] == '#' {(0,0,255,255)} else {(0,0,100,255)});
            }
        }            
    }

    for r in 0..height {
        for c in 0..width {
            let mut is_match = true;
            'outer: for ir in 0..inner_height {
                for ic in 0..inner_width {
                    if sea_monster[ir][ic] == '#' {
                        let (nr,nc) = get_correct_coor(&(r+ir,c+ic), tile_orientation, image.len()-1);
                        is_match &= image[nr][nc] == '#';
                        if !is_match && !movie { break 'outer; }
                    }
                }
            }

            if is_match { 
                if movie {
                    for ir in 0..inner_height {
                        for ic in 0..inner_width {
                            if sea_monster[ir][ic] == '#' {
                                let (nr, nc) = (r+ir,c+ic);
                                img.set_pixel(nc,nr, (255,0,0,255));
                            }
                        }
                    }        
                    for _ in 0..5 {
                        img.save_png(&format!("images/puzzle-{:05}.png", counter).to_string());
                        *counter += 1;
                    }
                }
                count += 1; 
            }
        }
    }

    if movie {
        let frames = 3 + 10 * count;
        for _ in 0..frames  {
            img.save_png(&format!("images/puzzle-{:05}.png", counter).to_string());
            *counter += 1;
        }
    }

    count
}

#[allow(dead_code)]
pub fn run(real: bool, print_result: bool) -> (u128, u128, u128) {
    let start0 = Instant::now();

    let input_file: &str = if !real {
        "./input/day20_20_test2.txt"
    } else {
        "./input/day20_20_real.txt"
    };
    let input = tools::get_input(String::from(input_file));

    let movie: bool = false;

    let mut tiles: BTreeMap<(u16,u16,u16,u16,i8), u32> = BTreeMap::new();
    let mut full_tiles: BTreeMap<u32, Vec<Vec<char>>> = BTreeMap::new();
    let mut current_tile = vec![ vec![ ' '; 10]; 10];
    let mut tile: (u16,u16,u16,u16);

    let tilecount = (input.len()+1) / 12;   

    for t in 0..tilecount {
        let line = &input[12*t + 0];
        let tile_id = line[5..9].parse::<u32>().unwrap();
        for l in 0..10 {
            current_tile[l] = input[12*t+1+l].chars().collect();
        }

        let mut left: u16 = 0;
        let mut right: u16 = 0;
        let mut top: u16 = 0;
        let mut bottom: u16 = 0;
        for l in 0..10 {
            top     = (top << 1)     + if current_tile[l][0] == '#' {1} else {0};
            right   = (right << 1)   + if current_tile[9][l] == '#' {1} else {0};
            bottom  = (bottom << 1)  + if current_tile[9-l][9] == '#' {1} else {0};
            left    = (left << 1)    + if current_tile[0][9-l] == '#' {1} else {0};
        }

        tile = (top, right, bottom, left);

        tiles.insert((tile.0,tile.1,tile.2,tile.3, 1), tile_id);
        tiles.insert((tile.1,tile.2,tile.3,tile.0, 2), tile_id);
        tiles.insert((tile.2,tile.3,tile.0,tile.1, 3), tile_id);
        tiles.insert((tile.3,tile.0,tile.1,tile.2, 4), tile_id);

        tile = (flip_code(top), flip_code(left), flip_code(bottom), flip_code(right)); 
        tiles.insert((tile.0,tile.1,tile.2,tile.3, -1), tile_id);
        tiles.insert((tile.1,tile.2,tile.3,tile.0, -2), tile_id);
        tiles.insert((tile.2,tile.3,tile.0,tile.1, -3), tile_id);
        tiles.insert((tile.3,tile.0,tile.1,tile.2, -4), tile_id);


        let mut full_tile = vec![vec![' '; 8]; 8];
        for r in 1..9 {
            for c in 1..9 {
                full_tile[r-1][c-1] = current_tile[r][c];
            }
        }
        full_tiles.insert(tile_id, full_tile);
    }

    let after0 = Instant::now();

    let start1 = Instant::now();

    let puzzle_size = (tilecount as f64).sqrt() as usize;
    let current_puzzle: Vec<Vec<(u16,u16,u16,u16,i8)>> = vec![vec![(0,0,0,0,0);puzzle_size];puzzle_size];

    let mut tile_ids: Vec<Vec<(u32,i8)>> = vec![vec![(0,0); puzzle_size]; puzzle_size];
    let mut res1 = 0;
    let mut counter = 0;
    if let Some(res) = find_next(&current_puzzle, &tiles, 0, 0, puzzle_size, movie, &mut counter, &tiles, &full_tiles) {

        // in preparation for part 2...
        for r in 0..puzzle_size { for c in 0..puzzle_size { 
            tile_ids[r][c] = (*tiles.get(&res[r][c]).unwrap(), res[r][c].4); 
        }}

        // determine result for part 1
        res1 = tile_ids[0][0].0 as u128 * 
           tile_ids[0][puzzle_size-1].0 as u128 *
           tile_ids[puzzle_size-1][0].0 as u128 * 
           tile_ids[puzzle_size-1][puzzle_size-1].0 as u128 ;
    }

    let after1 = Instant::now();
    if print_result {
        println!("Part 1: {}", res1);
    }

    let start2 = Instant::now();

    // create sea-monster
    let sea_monster: Vec<Vec<char>> = vec![
        "..................#.".chars().collect(),
        "#....##....##....###".chars().collect(),
        ".#..#..#..#..#..#...".chars().collect()
    ];

    // create full image (with correct orientation of tiles determined in part 1)
    let mut image = vec![ vec![' '; puzzle_size*8];puzzle_size*8];
    for tr in 0..puzzle_size {
        for tc in 0..puzzle_size {
            let tile_tuple = tile_ids[tr][tc];
            let correct_tile = get_correct_tile(tile_tuple.0, tile_tuple.1, &full_tiles);
            for r in 0..8usize {
                for c in 0..8usize {
                    image[tr*8+r][tc*8+c] = correct_tile[r][c];
                }
            }
        }
    }

    // count the pixels that are 'on'
    let pixel_count = image
        .iter()
        .fold(0, |acc, line| 
            acc + line
                .iter()
                .fold(0, |acc2, c| acc2 + if *c=='#' {1} else {0}
        )
    );

    let mut res2 = 0;
    // for all orientations of the image check if the sea-monster is visible
    for o in -4..4 {
        let monster_count = find_monster_in_image(&sea_monster, &image, o, movie, &mut counter);
        if monster_count > 0 {
            let monster_pixels = 15 * monster_count; // assume monsters don't overlap !!
            res2 = pixel_count - monster_pixels;
            break;
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

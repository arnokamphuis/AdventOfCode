use super::tools;
use std::time::Instant;
use std::collections::HashSet;
use tools::Image;

fn step(floor: &HashSet<(i32,i32)>, _iteration: usize) -> HashSet<(i32,i32)>  {
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

        let mut new_black = floor.contains(&tile);
        match new_black {
            true =>  { if c == 0 || c > 2 { new_black = false; } },
            false => { if c==2            { new_black = true; } }
        }

        if new_black {
            new_floor.insert(tile);
        }
    }
    // make_image(&new_floor, iteration, None);
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

    // let mut image_count = 0;

    let mut floor: HashSet<(i32,i32)> = HashSet::new();
    for line in &input {
        let mut c_iter = line.chars();
        let mut x = 0;
        let mut y = 0;
        // let mut path: HashSet<(i32,i32)> = HashSet::new();
        // path.insert((0,0));
        // make_image(&floor, image_count, Some(&path));
        // image_count += 1;
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
            // path.insert((x,y));
            // make_image(&floor, image_count, Some(&path));
            // image_count += 1;
        }
        if floor.contains(&(x,y)) {
            floor.remove(&(x,y));
        } else {
            floor.insert((x,y));
        }

        // make_image(&floor, image_count, None);
        // image_count += 1;
    }

    let res1 = floor.len();

    let after1 = Instant::now();
    if print_result {
        println!("Part 1: {}", res1);
    }

    // for _ in 0..10 {
    //     make_image(&floor, image_count, None);
    //     image_count += 1;
    // }

    let start2 = Instant::now();

    let mut new_floor = floor.clone();
    for _ in 0..100 {
        new_floor = step(&new_floor, 0 /* image_count */);
        // image_count += 1;
    }
    let res2 = new_floor.len();

    // for _ in 0..10 {
    //     make_image(&new_floor, image_count, None);
    //     image_count += 1;
    // }

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

//----------------------------------------------------------------------------------------------
//----------------------------------------------------------------------------------------------
//----------------------------------------------------------------------------------------------
//----------------------------------------------------------------------------------------------
//----------------------------------------------------------------------------------------------
//----------------------------------------------------------------------------------------------

#[allow(dead_code)]
fn make_image(floor: &HashSet<(i32,i32)>, counter: usize, path: Option<&HashSet<(i32,i32)>>) {
    let size = (160usize,160usize);
    let full_offset = (80i32, 80i32);

    let center_tile = (8,7);
    let tile_size = (15,13);

    let tile_template: Vec<Vec<char>> = vec![
        "               ".chars().collect::<Vec<char>>(), 
        "       #       ".chars().collect::<Vec<char>>(), 
        "     #####     ".chars().collect::<Vec<char>>(),
        "   #########   ".chars().collect::<Vec<char>>(),
        " ############# ".chars().collect::<Vec<char>>(),
        " ############# ".chars().collect::<Vec<char>>(),
        " ######0###### ".chars().collect::<Vec<char>>(),
        " ############# ".chars().collect::<Vec<char>>(),
        " ############# ".chars().collect::<Vec<char>>(),
        "   #########   ".chars().collect::<Vec<char>>(),
        "     #####     ".chars().collect::<Vec<char>>(),
        "       #       ".chars().collect::<Vec<char>>(),
        "               ".chars().collect::<Vec<char>>()  ];

    let image_size = ( size.0 * 15, size.1 * 8 );
    let mut img = Image::new(image_size.0, image_size.1, 1);
    img.clear((255,255,255,255));

    for tile in floor {

        let center = (
            (full_offset.0 + tile.0) * 15 - tile.1 * 8
            ,
            (full_offset.1 - tile.1) * 8
        );

        let topleft = (center.0 as usize - center_tile.0, center.1 as usize - center_tile.1);
        for dx in 0..tile_size.0 {
            for dy in 0..tile_size.1 {
                if tile_template[dy][dx] != ' ' {
                    img.set_pixel( topleft.0 + dx, topleft.1 + dy, (0,0,0,255));
                }
            }
        }
    }

    if path != None {
        for tile in path.unwrap() {

            let center = (
                (full_offset.0 + tile.0) * 15 - tile.1 * 8
                ,
                (full_offset.1 - tile.1) * 8
            );
        
            // let to = if tile.1 % 2 == 0 { 0 } else { offset.1 };
            // let center = ((tile.0 as usize + full_offset.0) * offset.0 - to , (tile.1 as usize + full_offset.1)  * offset.1);
            let topleft = (center.0 as usize - center_tile.0, center.1 as usize - center_tile.1);
            for dx in 0..tile_size.0 {
                for dy in 0..tile_size.1 {
                    if tile_template[dy][dx] != ' ' {
                        img.set_pixel( topleft.0 + dx, topleft.1 + dy, (255,0,0,255));
                    }
                }
            }
        }    
    }

    img.save_png(&format!("images/floor-{:010}.png", counter));
}


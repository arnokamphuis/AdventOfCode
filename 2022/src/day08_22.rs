use super::tools;
use std::time::Instant;
// use tools::Image;

#[allow(dead_code)]
pub fn run(real: bool, print_result: bool) -> (u128, u128, u128) {
    let start0 = Instant::now();

    let input_file: &str = if !real {
        "./input/day08_22_test.txt"
    } else {
        "./input/day08_22_real.txt"
    };
    let input = tools::get_input(String::from(input_file));

    let h = input.len();
    let w = input[0].len();

    let grid: Vec<Vec<u8>> = input.iter().map(|line| {
        line.chars().map(|c| c.to_digit(10).unwrap() as u8).collect()
    }).collect();

    // ***********************************************************
    // let mut img: Image = Image::new(w,h,8);
    // img.clear((0,0,0,255));
    // grid.iter().enumerate().for_each(|(y,line)| {
        // line.iter().enumerate().for_each(|(x,h)| img.set_pixel(x,y,(0,30+20*h,0,255)));
    // });
    // img.save_png(&format!("images/day08_22/trees.png"));
    // ***********************************************************

    let after0 = Instant::now();

    let start1 = Instant::now();

    let mut count = 2 * w + 2 * h - 4;

    // ***********************************************************
    // let mut img_step = img.clone();
    // let mut counter = 0;
    // for x in 0..w { img_step.set_pixel(x,0,(255,0,0,255)); }
    // for x in 0..w { img_step.set_pixel(x,h-1,(255,0,0,255)); }
    // for y in 0..h { img_step.set_pixel(0,y,(255,0,0,255)); }
    // for y in 0..h { img_step.set_pixel(w-1,y,(255,0,0,255)); }
    // img_step.save_png(&format!("images/day08_22/visible_{:05}.png", counter)); counter += 1;
    // ***********************************************************

    for y in 1..h-1 {
        for x in 1..w-1 {
            let mut visible = vec![true;4];
            for dx in (0..x).rev() { visible[0] &= grid[y][x] > grid[y][dx]; }
            for dx in  x+1..w      { visible[1] &= grid[y][x] > grid[y][dx]; }
            for dy in (0..y).rev() { visible[2] &= grid[y][x] > grid[dy][x]; }
            for dy in  y+1..h      { visible[3] &= grid[y][x] > grid[dy][x]; }
            if visible.iter().fold(false, |acc, &v| acc || v) { 
                count+=1;
                // ***********************************************************
                // img_step.set_pixel(x,y,(255,0,0,255));
                // img_step.save_png(&format!("images/day08_22/visible_{:05}.png", counter)); counter += 1;
                // ***********************************************************
            };
        }
    }

    let after1 = Instant::now();
    if print_result {
        println!("Part 1: {}", count);
    }

    let start2 = Instant::now();

    let score = | x: usize, y: usize /*, img_place: &mut Image */ | -> i64 {
        let mut scores = vec![0i64;4];
        for dx in (0..x).rev() { scores[0] += 1; /* img_place.set_pixel(dx,y,(0,255,255,255)); */ if grid[y][x] <= grid[y][dx] { break; } }
        for dx in  x+1..w      { scores[1] += 1; /* img_place.set_pixel(dx,y,(0,255,255,255)); */ if grid[y][x] <= grid[y][dx] { break; } }
        for dy in (0..y).rev() { scores[2] += 1; /* img_place.set_pixel(x,dy,(0,255,255,255)); */ if grid[y][x] <= grid[dy][x] { break; } }
        for dy in  y+1..h      { scores[3] += 1; /* img_place.set_pixel(x,dy,(0,255,255,255)); */ if grid[y][x] <= grid[dy][x] { break; } }
        scores[0] * scores[1] * scores[2] * scores[3]
    };

    // ***********************************************************
    // let mut img_score = img.clone();
    // ***********************************************************

    let mut all_scores = vec![];
    // counter = 0;
    (0..h).for_each(|y| (0..w).for_each(|x| {
        // ***********************************************************
        // let mut img_place = img_score.clone();
        // img_place.set_pixel(x,y,(255,0,255,155));
        // ***********************************************************

        let s = score(x, y /*,&mut img_place*/);
        // let s_i = (255.0f64 * s as f64 / 671160.0f64) as u8;
        // img_score.set_pixel(x,y,(0,s_i,s_i,255));
        all_scores.push(s);
        
        // ***********************************************************
        // img_place.save_png(&format!("images/day08_22/place_{:05}.png", counter)); counter += 1;
        // ***********************************************************
    }));
    // ***********************************************************
    // img_score.save_png(&format!("images/day08_22/score.png"));
    // for _ in 1..120 { img_score.save_png(&format!("images/day08_22/place_{:05}.png", counter)); counter += 1; }
    // ***********************************************************

    let res2: i64 = *all_scores.iter().max().unwrap();

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

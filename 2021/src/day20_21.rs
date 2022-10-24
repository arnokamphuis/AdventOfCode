use super::tools;
use std::time::Instant;
use std::collections::HashMap;
use tools::Image;


#[allow(dead_code)]
pub fn run(real: bool, print_result: bool) -> (u128, u128, u128) {
    let start0 = Instant::now();

    let input_file: &str = if !real {
        "./input/day20_21_test.txt"
    } else {
        "./input/day20_21_real.txt"
    };
    let input = tools::get_input(String::from(input_file));

    let algo:Vec<bool> = input[0].chars().fold(vec![], |mut v, b| { v.push(b == '#'); v } );


    let new_pixel = | i: i64, j: i64, img: &HashMap<(i64,i64),bool>, iter: usize | -> bool {
        let mut index: usize = 0;
        (-1..=1).for_each(|x| {
            (-1..=1).for_each(|y| {
                index = index << 1;
                if let Some(b) = img.get(&(i+x,j+y)) {
                    index += *b as usize;
                } else {
                    index += (algo[0] && (iter % 2 == 1)) as usize;
                }
            });
        });
        algo[index]
    };

    let mut image: HashMap<(i64,i64),bool> = input[2..]
        .iter()
        .enumerate()
        .fold(HashMap::new(), |mut map, (x,line)| {
            line.chars().enumerate().for_each(|(y,c)| { 
                map.insert((x as i64, y as i64), c=='#');
            });
            map
        });

    let update = | img: &HashMap<(i64,i64),bool>, iteration: usize | -> HashMap<(i64,i64),bool> {
        let min_x = *img.keys().map(|(x,_)| *x).collect::<Vec<i64>>().iter().min().unwrap();
        let max_x = *img.keys().map(|(x,_)| *x).collect::<Vec<i64>>().iter().max().unwrap();
        let min_y = *img.keys().map(|(_,y)| *y).collect::<Vec<i64>>().iter().min().unwrap();
        let max_y = *img.keys().map(|(_,y)| *y).collect::<Vec<i64>>().iter().max().unwrap();

        let mut new_image: HashMap<(i64,i64),bool> = HashMap::new();
        (min_x-1..=max_x+1).for_each(|x| {
            (min_y-1..=max_y+1).for_each(|y| {
                new_image.insert( (x,y), new_pixel(x, y, &img, iteration));
            });    
        });
        new_image
    };

    let mut scans: Vec<HashMap<(i64,i64), bool>> = vec![image.clone()];
    let img_min_x: i64 = -50; let img_max_x: i64 = 150;
    let img_min_y: i64 = -50; let img_max_y: i64 = 150;    

    let after0 = Instant::now();

    let start1 = Instant::now();

    (0..2).for_each(|iteration| {
        image = update(&image, iteration);
        if print_result {
            scans.push(image.clone());
        }
    });

    let res1 = image.values().filter(|&v| *v).count();

    let after1 = Instant::now();
    if print_result {
        println!("Part 1: {}", res1);
    }

    let start2 = Instant::now();

    (2..50).for_each(|iteration| {
        image = update(&image, iteration);
        if print_result {
            scans.push(image.clone());
        }
    });

    let res2 = image.values().filter(|&v| *v).count();

    let after2 = Instant::now();
    if print_result {
        println!("Part 2: {}", res2);
    }

    if print_result {
        scans.iter().enumerate().for_each(|(iteration, scan)| {
            let mut img = Image::new( (img_max_x - img_min_x + 2) as usize, (img_max_y - img_min_y + 2) as usize, 4);
            img.clear( if iteration % 2 == 1 { (255,255,255,255) } else { (0,0,0,255) }  );
            scan.iter().for_each(|((x,y), &b)| {
                img.set_pixel( (x - img_min_x + 1) as usize, (y - img_min_y + 1) as usize, if b {(255,255,255,255)} else {(0,0,0,255)} );
            });
            img.save_png(&format!("images/day20-{:05}.png", iteration));
        });
    }

    (
        after0.duration_since(start0).as_nanos(),
        after1.duration_since(start1).as_nanos(),
        after2.duration_since(start2).as_nanos(),
    )
}

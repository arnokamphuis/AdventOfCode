use super::tools;
use std::time::Instant;
use std::collections::HashMap;

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

    let after0 = Instant::now();

    let start1 = Instant::now();

    (0..2).for_each(|iteration| {
        image = update(&image, iteration);
    });

    let res1 = image.values().filter(|&v| *v).count();

    let after1 = Instant::now();
    if print_result {
        println!("Part 1: {}", res1);
    }

    let start2 = Instant::now();

    (2..50).for_each(|iteration| {
        image = update(&image, iteration);
    });

    let res2 = image.values().filter(|&v| *v).count();

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

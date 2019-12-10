use std::time::{Instant};
use super::tools;
use std::collections::HashMap;
use num;

fn check_line(from: (u32,u32), to: (u32,u32), field: &HashMap<(u32,u32),bool>, width: usize, height: usize) -> bool {
    let deltax:i64 = to.0 as i64 - from.0 as i64;
    let deltay:i64 = to.1 as i64 - from.1 as i64;

    let gcd = num::integer::gcd(deltax,deltay) as i64;

    let mut stepx: i64 = 0;
    let mut stepy: i64 = 0;
    if deltax == 0 || deltay == 0 {
        if deltax == 0 { 
            stepx = 0;
            stepy = 1;
         }
        if deltay == 0 { 
            stepx = 1;
            stepy = 0;
        }
    } else {
        stepx = deltax as i64 / gcd;
        stepy = deltay as i64 / gcd;
    }

    println!("checking {:?} -> {:?} gcd:{} of ({},{}) ; ({},{})", from, to, gcd, deltax, deltay, stepx, stepy);


    let mut x: i64 = from.0 as i64;
    let mut y: i64 = from.1 as i64;

    let mut correct = true;
    loop  {
        x += stepx;
        y += stepy;

        print!(" {},{} ",x,y);

        if x < 0 || y < 0 || x >= width as i64 || y >= height as i64 {
            break;
        } 

        if (x as u32,y as u32) == to {
            break;
        }

        if field[&(x as u32,y as u32)] {
            correct = false;
            break;
        }
    }
    println!("");
    correct
}

#[allow(dead_code)]
pub fn run() {
    println!("Day 10 of 2019");

    let start0 = Instant::now();

    let input_file = "./input/day10_19_test.txt";
    // let input_file = "./input/day10_19_real.txt";
    let input = tools::get_input(String::from(input_file));

    let height = input.len();
    let width  = input[0].len();

    let mut field: HashMap<(u32,u32), bool> = HashMap::new();
    input.iter().enumerate().for_each(|(i,r)|{
        r.chars().enumerate().for_each(|(j,a)| {
            field.insert((i as u32,j as u32), a =='#');
        });
    });

    let after0 = Instant::now();
    println!("Init in {:?}", after0.duration_since(start0));

    let start1 = Instant::now();

    let mut max_count = std::u32::MIN;

    field.iter().filter(|p| !*((*p).1)).for_each(|p| {

        let mut count = 0;
        field.iter().filter(|t| *((*t).1)).for_each(|t| {
            if check_line( *p.0, *t.0, &field, width, height) {
                count += 1;
            }
        } );
        if max_count < count { 
            println!("max: {:?} => {}", *p.0, count);
            max_count = count; 
        }
    });


    let after1 = Instant::now();
    println!("Part 1: {}, in {:?}", max_count, after1.duration_since(start1));

    let start2 = Instant::now();

    let after2 = Instant::now();
    println!("Part 2: {}, in {:?}", 0, after2.duration_since(start2));
}

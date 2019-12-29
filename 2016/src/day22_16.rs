use std::time::Instant;
use super::tools;
use std::collections::HashMap;

struct Node {
    // x: u32,
    // y: u32,
    // disksize: u32,
    used: u32,
    available: u32,
    // use_percentage: u32,
}

#[allow(dead_code)]
pub fn run() {
    println!("Day 22 of 2016");

    let start0 = Instant::now();
    let input_file = "./input/day22_16_real.txt";
    let input = tools::get_input(String::from(input_file));

    let mut width = 0;
    let mut height = 0;
    let mut nodes: HashMap<(u32,u32), Node> = HashMap::new();

    input.iter().for_each(|line| {
        let mut iter = line.split_whitespace();
        let name = iter.next().unwrap();
        let disksize_str = iter.next().unwrap().to_string();
        let _disksize = disksize_str[..disksize_str.len()-1].parse::<u32>().unwrap();
        let used_str = iter.next().unwrap();
        let used = used_str[..used_str.len()-1].parse::<u32>().unwrap();
        let available_str = iter.next().unwrap();
        let available = available_str[..available_str.len()-1].parse::<u32>().unwrap();
        let use_percentage_str = iter.next().unwrap();
        let _use_percentage = use_percentage_str[..use_percentage_str.len()-1].parse::<u32>().unwrap();

        let mut nameiter = name.split("-");
        nameiter.next();
        let x = nameiter.next().unwrap()[1..].to_string().parse::<u32>().unwrap();
        let y = nameiter.next().unwrap()[1..].to_string().parse::<u32>().unwrap();

        if x > width { width = x; }
        if y > height { height = y; }
        nodes.insert((x,y), Node { /*x: x, y: y, disksize: disksize, */used: used, available: available/*, use_percentage: use_percentage*/ });
    });

    let after0 = Instant::now();
    println!(
        "Init in {:?}",
        after0.duration_since(start0)
    );

    let start1 = Instant::now();    

    let mut counter = 0;
    nodes.iter().for_each(|((x1,y1),node1)| { 
        nodes.iter().for_each(|((x2,y2),node2)| {
            if !(x1==x2 && y1==y2) {
                if node1.used != 0 && node1.used < node2.available {
                    counter += 1;
                }
            }
        });
    });

    let after1 = Instant::now();
    println!(
        "Part 1: {}, in {:?}",
        counter,
        after1.duration_since(start1)
    );

    let start2 = Instant::now();
    
    let mut empty = (0,0);
    for y in 0..height {
        for x in 0..width {
            if nodes[&(x,y)].used == 0 {
                empty = (x,y);
            }
            
        }
    }

    let res2 = empty.0 + empty.1 + width + (width-1) * 5;
    let after2 = Instant::now();
    println!(
        "Part 2: {}, in {:?}",
        res2,
        after2.duration_since(start2)
    );
}

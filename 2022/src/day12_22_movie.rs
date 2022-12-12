use super::tools;
use std::time::Instant;
use std::collections::HashMap;
use priority_queue::DoublePriorityQueue;
use tools::Image;

fn find_path(map: &HashMap<(i16,i16), i8>, s: &Vec<(i16, i16)>, e: (i16, i16), img: &mut Image, counter: &mut usize) -> i16 {
    let mut q = DoublePriorityQueue::new();
    let mut visited: Vec<(i16,i16)> = vec![];

    let mut prev: HashMap<(i16,i16),(i16,i16)> = HashMap::new();
    let mut dist: HashMap<(i16,i16),i16> = HashMap::new();

    let heuristic = | p: (i16,i16) | -> i16 {
        (p.0-e.0).abs() + (p.1-e.1).abs()
    };

    let reconstruct_path = | prev_map: &HashMap<(i16,i16),(i16,i16)>| -> Vec<(i16,i16)> {
        let mut res = vec![];
        let mut cp = e;
        while let Some(&pp) = prev_map.get(&cp) {
            res.insert(0,pp);
            cp = pp;
        }
        res
    };

    for &start in s {
        println!("start heuristic {}", heuristic(start));
        q.push(start,heuristic(start));
        dist.insert(start,0);
    }

    // *******************************************************
    let part = (s.len()==1) as usize + 1;
    let offset = if part == 1 { 45 } else { 2 };
    map.iter().for_each(|(&(x,y),&h)| {
        img.set_pixel(x as usize,y as usize + offset, (0, (9 * h) as u8,0,255));    
    });
    img.set_pixel(e.0 as usize,e.1 as usize + offset, (255, (9 * *map.get(&e).unwrap())as u8, 255, 255));
    img.save_png(&format!("images/day12_22/search_{:05}.png", counter)); *counter += 1;
    // *******************************************************

    let dir = vec![(-1,0), (1,0), (0,-1), (0,1)];

    while !q.is_empty() {
        let next = q.pop_min().unwrap();
        
        if next.0 == e { 
            // *******************************************************
            let path = reconstruct_path(&prev);
            for p in path {
                img.set_pixel(p.0 as usize, p.1 as usize + offset, (0, 0, 255, 255));
                img.save_png(&format!("images/day12_22/search_{:05}.png", counter)); *counter += 1;
            }
            // *******************************************************
            return dist[&e]; 
        }

        let current_dist = *dist.get(&next.0).unwrap();
        
        visited.push(next.0);
        // *******************************************************
        img.set_pixel(next.0.0 as usize,next.0.1 as usize + offset, (255, (9 * *map.get(&next.0).unwrap()) as u8, 255, 255));
        img.save_png(&format!("images/day12_22/search_{:05}.png", counter)); *counter += 1;
        // *******************************************************

        let current_height = map.get(&next.0).unwrap();

        dir.iter().for_each(|d| {
            let mut p = next.0;
            p.0 += d.0; p.1 += d.1;
            if let Some(height) = map.get(&p) {
                if (height - current_height) <= 1 {
                    if let Some(&p_dist) = dist.get(&p) {
                        if current_dist + 1 < p_dist {
                            q.push(p, current_dist + 1 + heuristic(p));
                            *prev.get_mut(&p).unwrap() = next.0;
                            *dist.get_mut(&p).unwrap() = current_dist + 1;
                        }
                    } else {
                        q.push(p, current_dist + 1 + heuristic(p));
                        prev.insert(p, next.0);
                        dist.insert(p, current_dist + 1);
                    }
                }
            }
        });

    }
    i16::MAX
}

#[allow(dead_code)]
pub fn run(real: bool, print_result: bool) -> (u128, u128, u128) {
    let start0 = Instant::now();

    let input_file: &str = if !real {
        "./input/day12_22_test.txt"
    } else {
        "./input/day12_22_real.txt"
    };
    let input = tools::get_input(String::from(input_file));

    let mut img = Image::new(168,90,8); // offset part 1 -> 2 offset part 2 -> 45
    img.clear((0,0,0,255));
    let mut counter = 0;

    let mut start = (0,0);
    let mut end = (0,0);
    let mut map: HashMap<(i16,i16), i8> = HashMap::new();

    input.iter().enumerate().for_each(|(y, line)| {
        line.chars().enumerate().for_each(|(x, c)| {
            let p = (x as i16, y as i16);
            map.insert(p, match c {
                'S' => { start = p;  0 }
                'E' => { end   = p; 25 },
                _   => { c as i8 - 'a' as i8 }
            });
        });
    });

    let after0 = Instant::now();

    let start1 = Instant::now();

    let res1 = find_path(&map, &vec![start], end, &mut img, &mut counter);

    let after1 = Instant::now();
    if print_result {
        println!("Part 1: {}", res1);
    }

    let start2 = Instant::now();

    let pot_start = map
        .iter()
        .filter(|(_,&h)| h == 0)
        .map(|(&c,_)| c)
        .collect::<Vec<_>>();

    let res2 = find_path(&map, &pot_start, end, &mut img, &mut counter);

    let after2 = Instant::now();
    if print_result {
        println!("Part 2: {}", res2);
    }

    (0..480).for_each(|_| {
        img.save_png(&format!("images/day12_22/search_{:05}.png", counter)); counter += 1;
    });

    (
        after0.duration_since(start0).as_nanos(),
        after1.duration_since(start1).as_nanos(),
        after2.duration_since(start2).as_nanos(),
    )
}

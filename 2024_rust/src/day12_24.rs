use super::tools;
use std::time::Instant;
use std::collections::{HashMap, HashSet};
use std::collections::VecDeque;
#[cfg(feature = "make_movie")]
use crate::tools::Image;

fn find_perimeter(map: &HashSet::<(i32,i32)>) -> HashMap::<(i32,i32), HashSet::<(i32,i32)>> {
    let mut perimeter: HashMap::<(i32,i32), HashSet::<(i32,i32)>> = HashMap::new();

    map.iter().for_each(|(c, r)| {
        [(0,1), (0,-1), (1,0), (-1,0)].iter().for_each(|(dc, dr)| {
            if !map.contains(&(c+dc, r+dr)) {
                perimeter.entry((*dc, *dr)).or_insert(HashSet::<(i32,i32)>::new()).insert((*c, *r));
            }
        });
    });

    perimeter
}


fn remove_connected(map: &HashSet::<(i32,i32)>, (sc, sr): (i32,i32)) -> HashSet::<(i32,i32)> {
    let mut cc: HashSet::<(i32,i32)> = HashSet::new();

    let mut q: VecDeque::<(i32,i32)> = VecDeque::new();
    q.push_back((sc, sr));
    while q.len() > 0 {
        let (c, r) = q.pop_front().unwrap();
        if !map.contains(&(c,r)) ||  cc.contains(&(c,r)) {
            continue;
        }
        cc.insert((c,r));
        [(0,1), (0,-1), (1,0), (-1,0)].iter().for_each(|(dc, dr)| {
            if map.contains(&(c+dc, r+dr)) {
                q.push_back((c+dc, r+dr));
            }
        });
    }
    cc
}

fn find_connected(map: &HashMap<String, HashSet::<(i32,i32)>>) -> HashMap<String, Vec<HashSet::<(i32,i32)>>> {
    let mut connected: HashMap<String, Vec<HashSet::<(i32,i32)>>> = HashMap::new();

    #[cfg(feature = "make_movie")]
    let (C, R) = (140, 140);
    #[cfg(feature = "make_movie")]
    let mut img: Image = Image::new(C as usize, R as usize, 8);
    #[cfg(feature = "make_movie")]
    img.clear((0, 0, 0, 255));
    #[cfg(feature = "make_movie")]
    let plant_count = map.len();
    #[cfg(feature = "make_movie")]
    let colors: Vec<(u8, u8, u8, u8)> = (1..(plant_count + 1))
        .map(|i| (105_usize + 150 * i / plant_count, 105_usize + 150 * i / plant_count, 105_usize + 150 * i / plant_count, 255_usize))
        .map(|(r, g, b, a)| (r as u8, g as u8, b as u8, a as u8))
        .collect();
    #[cfg(feature = "make_movie")]
    let mut img_count = 0;

    map.iter().enumerate().for_each(|(_i, (plant, coords))| {
        connected.insert(plant.to_string(), vec![]);
        let mut remaining = coords.clone();
        while remaining.len() > 0 {
            let (sc, sr) = remaining.iter().next().unwrap();
            let cc = remove_connected(&remaining, (*sc, *sr));
            #[cfg(feature = "make_movie")]
            cc.iter().for_each(|(c, r)| {
                img.set_pixel(*c as usize, *r as usize, colors[i]);
            });
            #[cfg(feature = "make_movie")]
            img.save_png(&format!("images/day12_24_{:06}.png", { let tmp = img_count; img_count += 1; tmp}));
            // #[cfg(feature = "make_movie")]
            // img_count += 1;
            connected.get_mut(plant).unwrap().push(cc.clone());
            cc.iter().for_each(|(c, r)| {
                remaining.remove(&(*c, *r));
            });
        }
    });

    connected
}

fn count_sides(dir: &(i32,i32), perimeter_set: &HashSet::<(i32,i32)>) -> i32 {
    let mut seen_perimeter: HashSet<(i32,i32)> = HashSet::new();
    let mut s = 0;
    perimeter_set.iter().for_each(|(c_pr, r_pr)| {
        if !seen_perimeter.contains(&(*c_pr, *r_pr)) {
            s += 1;
            let mut q: VecDeque::<(i32,i32)> = VecDeque::new();
            q.push_back((*c_pr, *r_pr));
            while q.len() > 0 {
                let (c, r) = q.pop_front().unwrap();
                if !seen_perimeter.contains(&(c,r)) {
                    seen_perimeter.insert((c,r));
                    [(dir.1,-dir.0), (-dir.1, dir.0)].iter().for_each(|(dc, dr)| {
                        if perimeter_set.contains(&(c+dc, r+dr)) {
                            q.push_back((c+dc, r+dr));
                        }
                    });
                }
            }

        }
    });
    s
}

#[allow(dead_code)]
pub fn run(real: bool, print_result: bool) -> (u128, u128, u128) {
    let start0 = Instant::now();

    let input_file: &str = if !real {
        "./input/day12-test.txt"
    } else {
        "./input/day12-real.txt"
    };
    let input = tools::get_input(String::from(input_file));
    let mut maps: HashMap<String, HashSet<(i32,i32)>> = HashMap::new();

    input.iter().enumerate().for_each(|(r, line)| {
        line.chars().enumerate().for_each(|(c, ch)| {
            maps.entry(ch.to_string()).or_insert(HashSet::<(i32,i32)>::new()).insert((r as i32, c as i32));
        });
    });

    let after0 = Instant::now();

    let start1 = Instant::now();

    let connected = find_connected(&maps);
    let res1: i32 = connected.iter().fold(0, |acc, (_, ccs)| {
        acc + ccs
            .iter()
            .map(|cc| {
                cc.iter().fold(0, |acc, (c, r)| {
                    acc + 4 - [(-1,0), (1,0), (0,-1), (0,1)].iter().filter(|(dc, dr)| {
                        cc.contains(&(c+dc, r+dr))
                    }).count() as i32
                }) * cc.len() as i32
            }).sum::<i32>()
    });

    let after1 = Instant::now();
    if print_result {
        println!("Part 1: {}", res1);
    }

    let start2 = Instant::now();

    let res2 = find_connected(&maps).iter().fold(0, |res, (_, ccs)| {
        res + ccs.iter().fold(0, |acc, cc| {
            acc + cc.len() as i32 * 
            find_perimeter(cc).iter().fold(0, |sides, (dir, perimeter_set)| {
                sides + count_sides(dir, perimeter_set)
            })
        })
    });

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

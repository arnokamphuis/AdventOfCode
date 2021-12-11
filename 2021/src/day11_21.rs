use super::tools;
use std::time::Instant;
use std::collections::{BTreeMap, BTreeSet};
// use tools::Image;

#[allow(dead_code)]
pub fn run(real: bool, print_result: bool) -> (u128, u128, u128) {
    let start0 = Instant::now();

    let input_file: &str = if !real {
        "./input/day11_21_test.txt"
    } else {
        "./input/day11_21_real.txt"
    };
    let input = tools::get_input(String::from(input_file));

    let mut octopusses: BTreeMap<(i8,i8),i8> = input.iter().enumerate().fold(BTreeMap::new(), |mut map, (i,line)| {
        line.chars().enumerate().for_each(|(j,c)| { map.insert((i as i8,j as i8), c.to_digit(10).unwrap() as i8); }); map
    });

    // let mut img = Image::new(12,12,10);
    // img.clear((0,0,0,255));
    // let mut img_count = 0;

    let dir = [(-1,-1),(-1,0),(-1,1),(0,-1),(0,1),(1,-1),(1,0),(1,1)];

    let day = |octos: &mut BTreeMap<(i8,i8),i8>| -> usize {
        octos.iter_mut().for_each(|(_,v)| {*v += 1;});

        // img.clear((0,0,0,255));
        // octos.iter().for_each(|((i,j),v)| {
        //     let c = (255 * (*v as u16) / 16) as u8;
        //     img.set_pixel(*i as usize + 1, *j as usize + 1, (c, c, c, 255));
        // });

        let mut flashable: BTreeSet<(i8,i8)> = octos
            .iter()
            .filter(|(_,v)| **v > 9)
            .fold(BTreeSet::new(), |mut s, ((i,j),_)| {
                s.insert((*i,*j)); s
            });

        let mut flashed: BTreeSet<(i8,i8)> = BTreeSet::new();
        while flashed.len() != flashable.len() {
            flashable = flashable
                    .iter()
                    .fold(BTreeSet::new(), |mut map, o| {
                        if !flashed.contains(o) {
                            flashed.insert(*o);
                            for d in dir {
                                let no = (o.0 + d.0, o.1 + d.1);
                                if let Some(oc) = octos.get_mut(&no) { 
                                    *oc += 1; 
                                    if *oc > 9 { map.insert(no); }
                                    // let c = (255 * ( (*oc).min(8) as u16) / 16) as u8;
                                    // img.set_pixel(o.0 as usize + 1, o.1 as usize + 1, (c,c,c,255));
                                };
                            }
                            // img.set_pixel(o.0 as usize + 1, o.1 as usize + 1, (255,255,255,255));
                        }
                        map
                    })
                    .iter()
                    .cloned().collect::<BTreeSet<(i8,i8)>>()
                    .union(&flashable)
                    .cloned().collect::<BTreeSet<(i8,i8)>>();

            // img.save_png(&format!("movie-day11/day11-{:05}.png", img_count));
            // img_count += 1;
        }

        octos
            .iter_mut()
            .filter(|(_,&mut v)| { v > 9 })
            .for_each(|(_,v)| { *v = 0 });

        flashed.len()
    };

    let after0 = Instant::now();

    let start1 = Instant::now();

    let mut res1 = 0;
    for _ in 0..100 {
        res1 += day(&mut octopusses);
    };

    let after1 = Instant::now();
    if print_result {
        println!("Part 1: {}", res1);
    }

    let start2 = Instant::now();

    let mut step = 101;
    while day(&mut octopusses) != 100 {
        step+=1;
    };
    
    // for _ in 0..20 {
    //     img.save_png(&format!("movie-day11/day11-{:05}.png", img_count));
    //     img_count += 1;
    // }

    let after2 = Instant::now();
    if print_result {
        println!("Part 2: {}", step);
    }

    (
        after0.duration_since(start0).as_nanos(),
        after1.duration_since(start1).as_nanos(),
        after2.duration_since(start2).as_nanos(),
    )
}

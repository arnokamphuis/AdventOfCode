use super::tools;
use std::time::Instant;
use std::collections::HashMap;
use tools::Image;

#[derive(Debug)]
enum Inst {
    Walk(i64),
    Rotate(char),
}

#[allow(dead_code)]
pub fn run(real: bool, print_result: bool) -> (u128, u128, u128) {
    let start0 = Instant::now();

    let input_file: &str = if !real {
        "./input/day22_22_test.txt"
    } else {
        "./input/day22_22_real.txt"
    };
    let input = tools::get_input(String::from(input_file));

    let mut start_pos = (0i64,0i64);
    let mut field: HashMap<(i64,i64),bool> = HashMap::new();
    let mut instructions: Vec<Inst> = vec![];
    input.iter().enumerate().for_each(|(y,line)| {
        if y == 0 {
            start_pos = (1 + line.chars().position(|c| c == '.').unwrap() as i64, 1i64);
        }

        if line.contains('.') || line.contains('#') {
            line.chars().enumerate().filter(|&(_,c)| c=='.' || c=='#').for_each(|(x,c)| {
                field.insert((x as i64 + 1,y as i64 + 1), c == '#');
            });
        } else if line.len() > 0 {
            let mut num = 0;
            for c in line.chars() {
                if let Some(v) = c.to_digit(10) {
                    num = num * 10 + v as i64;
                } else {
                    instructions.push(Inst::Walk(num));
                    num = 0;
                    instructions.push(Inst::Rotate(c));
                }
            }
            if num > 0 {
                instructions.push(Inst::Walk(num));
                num = 0;
            }    
        }
    });

    let min_field = field.iter().fold((i64::MAX, i64::MAX), |acc, (v,_)| { (acc.0.min(v.0), acc.1.min(v.1)) });
    let max_field = field.iter().fold((i64::MIN, i64::MIN), |acc, (v,_)| { (acc.0.max(v.0), acc.1.max(v.1)) });

    let dirs = [(1,0), (0,1), (-1,0), (0,-1)];

    let mut img = Image::new( (1+max_field.0 - min_field.0) as usize, (1+max_field.1 - min_field.1) as usize, 4);
    img.clear((155,155,155,255));
    field.iter().for_each(|(pos, w)| {
        img.set_pixel( (pos.0 - min_field.0) as usize, (pos.1 - min_field.1) as usize, if *w {
            (0,0,0,255)
        } else {
            (255,255,255,255)
        });
    });
    let mut img1 = img.clone();
    let mut img2 = img.clone();
    let mut imgs = vec![&mut img1, &mut img2];
    imgs[0].save_png(&format!("images/day22_22/field_part1_{:05}.png", 0));
    imgs[1].save_png(&format!("images/day22_22/field_part2_{:05}.png", 0));

    let mut next_pos = | cp: (i64,i64), cd: i64, field: &HashMap<(i64,i64),bool>, part: usize | -> ((i64,i64), i64) {
        let d = dirs[cd as usize];
        let np = (cp.0 + d.0, cp.1 + d.1);
        if let Some(free_or_not) = field.get(&np) {
            if !free_or_not {
                return (np,cd);
            }
        } else { // need to find free pos in other direction
            if part == 1 {
                let nd = (-d.0, -d.1);
                let mut pos = (cp.0 + nd.0, cp.1 + nd.1);
                while let Some(free_or_not) = field.get(&pos) {
                    pos = (pos.0+nd.0, pos.1+nd.1);
                }
                pos = (pos.0-nd.0, pos.1-nd.1);
                if !field.get(&pos).unwrap() {
                    return (pos,cd);
                }
            } else {
                let norm_cp = (cp.0 - min_field.0, cp.1 - min_field.1);
                let width_cube = 1 + (max_field.0 - min_field.0)/3;
                let height_cube = 1 +(max_field.1 - min_field.1)/4;

                let face_index: HashMap<(i64,i64),i64> = HashMap::from_iter([
                    ((1,0),1), ((2,0), 2), ((1,1), 3), ((0,2), 4), ((1,2),5), ((0,3),6)
                ]);

                let transform = | f_id: i64, d: i64, coord: (i64,i64), w: i64 | -> ((i64,i64),i64) {
                    match (f_id, d) {
                        (1,2) => { (( 0, (w - 1 - coord.1) + 2 * w ), 0) }, // 4 with direction 0
                        (1,3) => { (( 0, (coord.0 - w) + 3 * w), 0) }, // to 6, with direction 0
                        (2,0) => { (( 2 * w - 1, w - 1 - coord.1 + 2 * w ), 2) }, // 5 with direction 2
                        (2,1) => { (( 2 * w - 1, coord.0 - 2 * w + w),2) }, // to 3, with direction 2
                        (2,3) => { (( coord.0 - 2 * w, 4 * w - 1 ), 3) }, // 6 with direction 3
                        (3,0) => { (( coord.1 - w + 2 * w, w - 1), 3) }, // 2 with direction 3
                        (3,2) => { (( coord.1 - w, 2 * w), 1) }, // 4, with direction 1
                        (4,2) => { (( w, (w - 1 - (coord.1 - 2 * w))), 0) }, // 1 with direction 0
                        (4,3) => { (( w, coord.0 + w), 0) }, // 3 with direction 0
                        (5,0) => { (( 3 * w - 1, w - 1 - (coord.1 - 2 * w)), 2) }, // 2 with direction 2
                        (5,1) => { (( w - 1, coord.0 - w + 3 * w), 2) }, // 6 with direction 2
                        (6,0) => { (( coord.1 - 3 * w + w, 3 * w - 1), 3) }, // 5 with direction 3
                        (6,1) => { (( coord.0 + 2 * w, 0), 1) }, // 2, with direction 1
                        (6,2) => { (( (coord.1 - 3 * w) + w, 0), 1) }, // to 1, with direction 1
                        _ => (coord, cd),
                    }
                };

                let face_coord = (norm_cp.0 / width_cube, norm_cp.1 / height_cube);
                let f_id = face_index[&face_coord];

                let (new_cp, new_cd) = transform(f_id, cd, norm_cp, width_cube);
                let ncp = (new_cp.0 + min_field.0, new_cp.1 + min_field.1);
                let ncd = new_cd;
                if let Some(free_or_not) = field.get(&ncp) {
                    if !free_or_not {
                        return (ncp, ncd);
                    }
                } else {
                    panic!("Should be inside field!!!! {:?} ({})", ncp, ncd);
                }
            }
        }
        (cp,cd)     
    };

    let mut run = | initial_pos: (i64,i64), initial_dir: i64, part: usize, counter: &mut usize | -> i64 {
        let mut cur_pos = initial_pos;
        let mut cur_dir = initial_dir;
        instructions.iter().for_each(|inst| {
            match inst {
                Inst::Walk(l) => {
                    (0..*l).for_each(|_| {
                        (cur_pos, cur_dir) = next_pos(cur_pos, cur_dir, &field, part);
                        imgs[part as usize - 1].set_pixel( (cur_pos.0 - min_field.0) as usize, (cur_pos.1 - min_field.1) as usize, (0,0,255,255));
                    });
                },
                Inst::Rotate(t) => {
                    match t {
                        'L' => { cur_dir -= 1; if cur_dir  < 0 { cur_dir = 3; }},
                        'R' => { cur_dir += 1; if cur_dir == 4 { cur_dir = 0; }},
                        _ => panic!(),
                    };
                }
            }
            imgs[part as usize - 1].set_pixel( (cur_pos.0 - min_field.0) as usize, (cur_pos.1 - min_field.1) as usize, (0,0,255,255));
            imgs[part as usize - 1].save_png(&format!("images/day22_22/field_part{}_{:05}.png", part, counter)); *counter += 1;
        });

        cur_pos.1 * 1000 + cur_pos.0 * 4 + cur_dir
    };

    let after0 = Instant::now();

    let start1 = Instant::now();

    let mut counter = 1;
    let res1 = run(start_pos, 0, 1, &mut counter);

    let after1 = Instant::now();
    if print_result {
        println!("Part 1: {}", res1);
    }

    let start2 = Instant::now();

    counter = 1;
    let res2 = run(start_pos, 0, 2, &mut counter);

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

use super::tools;
use std::time::Instant;
use std::collections::{HashSet, VecDeque, HashMap};
use rand::Rng;
use tools::Image;

#[allow(dead_code)]
pub fn run(real: bool, print_result: bool) -> (u128, u128, u128) {
    let start0 = Instant::now();

    let input_file: &str = if !real {
        "./input/day23_22_test.txt"
    } else {
        "./input/day23_22_real.txt"
    };
    let input = tools::get_input(String::from(input_file));

    let mut elves: HashSet<(i64,i64)> = HashSet::new();
    input.iter().enumerate().for_each(|(y,line)| { line.chars().enumerate().for_each(|(x,c)| {
        if c == '#' { elves.insert((x as i64, y as i64)); }
    })});

    let mut elf_pos_id: HashMap<(i64,i64), usize> = HashMap::new();
    let mut elf_id_pos: HashMap<usize, (i64,i64)> = HashMap::new();
    let mut elf_id_color: HashMap<usize, (u8,u8,u8,u8)> = HashMap::new();

    let mut rng = rand::thread_rng();
    elves.iter().enumerate().for_each(|(id,&elf)| {
        elf_pos_id.insert(elf,id);
        elf_id_pos.insert(id,elf);
        let num = rng.gen::<u32>();
        elf_id_color.insert(id, ((num >> 0 & 255) as u8, (num >> 1 & 255) as u8, (num >> 2 & 255) as u8, 255));
    });

    let mut test_actions: VecDeque<([(i64,i64);3] ,(i64,i64))> = VecDeque::from([
        ([(-1,-1), ( 1,-1), ( 0,-1)], ( 0,-1)),
        ([(-1, 1), ( 1, 1), ( 0, 1)], ( 0, 1)),
        ([(-1,-1), (-1, 1), (-1, 0)], (-1, 0)),
        ([( 1,-1), ( 1, 1), ( 1, 0)], ( 1, 0))
    ]);

    let all_eight = [(-1,-1), (-1,0), (-1,1), (0,-1), (0,1), (1,-1), (1,0), (1,1)];

    let rotate_actions = | dq: &mut VecDeque<([(i64,i64);3] ,(i64,i64))> | {
        if let Some(val) = dq.pop_front() {
            dq.push_back(val);
        }
    };

    let _print_elves = | elves: &HashSet<(i64,i64)> | {
        let min_field = elves.iter().fold((i64::MAX, i64::MAX), |acc, elf| (acc.0.min(elf.0), acc.1.min(elf.1)));
        let max_field = elves.iter().fold((i64::MIN, i64::MIN), |acc, elf| (acc.0.max(elf.0), acc.1.max(elf.1)));
        for y in min_field.1..=max_field.1 {
            for x in min_field.0..=max_field.0 {
                print!("{}", if elves.contains(&(x,y)) { '#' } else { '.' });
            }
            println!("");
        }
        println!("");
    };

    let min_coords: (i64,i64) = (-14, -13);
    let max_coords: (i64,i64) = (123, 123);
    let margin: i64 = 10;
    let mut img = Image::new( (max_coords.0 - min_coords.0 + 2 * margin) as usize, (max_coords.1 - min_coords.1 + 2 * margin) as usize, 8 );
    img.clear((0,0,0,255));

    let round = | elves: &HashSet<(i64,i64)>, test_actions: &VecDeque<([(i64,i64);3] ,(i64,i64))>, elf_pos_id: &mut HashMap<(i64,i64), usize>, elf_id_pos: &mut HashMap<usize, (i64,i64)> | -> (bool, HashSet<(i64,i64)>) {
        let mut new_elves: HashMap<(i64,i64), Vec<(i64,i64)>> = HashMap::new();
        let mut no_option: HashSet<(i64,i64)> = HashSet::new();
        for elf in elves {
            let mut found = false;
            if all_eight.iter().any(|delta| elves.contains( &(elf.0 + delta.0, elf.1 + delta.1) )  ) {
                'inner: for ta in test_actions {
                    if ta.0.iter().all(|delta| !elves.contains( &(elf.0 + delta.0, elf.1 + delta.1) )  ) {
                        let new_pos = (elf.0 + ta.1.0, elf.1 + ta.1.1);
                        if let Some(ne) = new_elves.get_mut(&new_pos) {
                            ne.push(*elf);
                        } else {
                            new_elves.insert(new_pos, vec![*elf]);
                        }
                        found = true;
                        break 'inner;
                    }
                }
            } 
            if !found {
                no_option.insert(*elf);
            }
        };

        let mut moved: HashMap<(i64,i64), (i64,i64)> = HashMap::new();
        let mut next_elves: HashSet<(i64,i64)> = new_elves.iter().fold(HashSet::new(), |mut acc, (pos, old_elves_pos)| {
            if old_elves_pos.len() == 1 { 
                moved.insert(old_elves_pos[0],*pos);
                acc.insert(*pos); 
            } else {
                old_elves_pos.iter().for_each(|oep| { acc.insert(*oep); });
            }
            acc
        });

        moved.iter().for_each(|(from,to)| {
            let id = elf_pos_id[from];
            *elf_id_pos.get_mut(&id).unwrap() = *to;

            elf_pos_id.remove(from);
            elf_pos_id.insert(*to, id);
        });


        no_option.iter().for_each(|elf| {
            next_elves.insert(*elf);
        });

        let moved: bool = !next_elves.eq(&elves);

        assert!(elves.len() == next_elves.len(), "ELVES LOST");

        (moved, next_elves)
    };

    let after0 = Instant::now();

    let start1 = Instant::now();

    let mut counter = 0;

    (1..=10).for_each(|_| {
        (_, elves) = round(&elves, &mut test_actions, &mut elf_pos_id, &mut elf_id_pos);

        img.clear((0,0,0,255));
        elves.iter().for_each(|elf| {
            let p = ( (elf.0 - min_coords.0 + margin) as usize, (elf.1 - min_coords.1 + margin) as usize );
            let id = elf_pos_id[elf];
            let color = elf_id_color[&id];
            img.set_pixel(p.0, p.1, color);
        });
        img.save_png(&format!("images/day23_22/elfs_{:05}.png", counter)); counter += 1;

        rotate_actions(&mut test_actions);
    });


    let min_field = elves.iter().fold((i64::MAX, i64::MAX), |acc, elf| (acc.0.min(elf.0), acc.1.min(elf.1)));
    let max_field = elves.iter().fold((i64::MIN, i64::MIN), |acc, elf| (acc.0.max(elf.0), acc.1.max(elf.1)));

    let res1 = (max_field.0 - min_field.0 + 1) * (max_field.1 - min_field.1 + 1) - elves.len() as i64;
    let after1 = Instant::now();
    if print_result {
        println!("Part 1: {}", res1);
    }

    let start2 = Instant::now();

    let mut res2 = 0;
    let mut moved;
    for r in 11.. {
        (moved, elves) = round(&elves, &mut test_actions, &mut elf_pos_id, &mut elf_id_pos);

        img.clear((0,0,0,255));
        elves.iter().for_each(|elf| {
            let p = ( (elf.0 - min_coords.0 + margin) as usize, (elf.1 - min_coords.1 + margin) as usize );
            let id = elf_pos_id[elf];
            let color = elf_id_color[&id];
            img.set_pixel(p.0, p.1, color);
        });
        img.save_png(&format!("images/day23_22/elfs_{:05}.png", counter)); counter += 1;

        rotate_actions(&mut test_actions);
        if !moved {
            res2 = r;
            break;
        }
    }

    let after2 = Instant::now();
    if print_result {
        println!("Part 2: {}", res2);
    }

    println!("field dimensions - min: {:?} \t max: {:?}", min_field, max_field);

    (
        after0.duration_since(start0).as_nanos(),
        after1.duration_since(start1).as_nanos(),
        after2.duration_since(start2).as_nanos(),
    )
}

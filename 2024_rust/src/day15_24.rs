use super::tools;
use std::time::Instant;
use std::collections::{HashSet, HashMap};
use itertools::Itertools;
#[cfg(feature = "make_movie")]
use crate::tools::Image;

fn get_object_ids_at_position(objects: &Vec<Vec<(i32, i32)>>, positions: &HashSet<(i32, i32)>) -> HashSet<usize> {
    positions.iter().fold(HashSet::<usize>::new(), |mut set, pos| {
        objects.iter().enumerate().for_each(|(i, obj)| {
            if obj.contains(pos) {
                set.insert(i);
            }
        });
        set
    })
}

fn can_move(map: &HashSet<(i32, i32)>, objects: &Vec<Vec<(i32, i32)>>, pos_set: &HashSet<(i32, i32)>, dir: &(i32, i32)) -> (bool, HashSet<usize>) {
    let next_pos_set = pos_set.iter().fold(HashSet::<(i32,i32)>::new(), |mut set, pos| {
        set.insert((pos.0 + dir.0, pos.1 + dir.1)); set
    });

    let obj_ids_in_front = get_object_ids_at_position(objects, &next_pos_set);

    let object_pos_in_front = obj_ids_in_front.iter().fold(HashSet::<(i32,i32)>::new(), |mut set, id| {
        objects[*id].iter().for_each(|&pos| {set.insert(pos);}); set
    });

    let next_object_pos_in_front = obj_ids_in_front.iter().fold(HashSet::<(i32,i32)>::new(), |mut set, id| {
        objects[*id].iter().for_each(|&pos| {set.insert((pos.0 + dir.0, pos.1 + dir.1 ) );}); set
    });

    let next_front: HashSet<(i32,i32)> = next_object_pos_in_front.difference(&object_pos_in_front)
        .into_iter()
        .map(|pos| (pos.0 - dir.0, pos.1 - dir.1))
        .fold(HashSet::<(i32,i32)>::new(), |mut set, pos| {set.insert(pos); set});

    if next_pos_set.difference(&next_front).all(|pos| map.contains(pos)) { // no walls in the way
        if obj_ids_in_front.is_empty() { // no objects in the way
            return (true, HashSet::new());
        }

        let result = can_move(map, objects, &next_front, dir);
        if result.0 {
            return (true, obj_ids_in_front.union(&result.1).cloned().collect());
        }
    }

    (false, HashSet::new())
}

fn do_operations(map: HashSet<(i32, i32)>, mut objects: Vec<Vec<(i32, i32)>>, start: (i32, i32), operations: &Vec<char>) -> usize {
    let dirs = vec![('^', (0, -1)), ('v', (0, 1)), ('<', (-1, 0)), ('>', (1, 0))]
        .into_iter()
        .collect::<HashMap<char, (i32, i32)>>();

    let mut frame_count = 0;
    let mut robot = start;
    operations.iter().for_each(|op| {
        let dir = dirs.get(op).unwrap_or(&(0, 0));
        if dir == &(0, 0) {
            panic!("Invalid operation");
        }
        let (can, objs) = can_move(&map, &objects, &HashSet::from([robot;1]), dir);
        if can {
            objs.iter().for_each(|&i| {
                objects[i] = objects[i].iter().map(|pos| (pos.0 + dir.0, pos.1 + dir.1)).collect();
            });
            robot = (robot.0 + dir.0, robot.1 + dir.1);
        }
        #[cfg(feature = "make_movie")]
        make_frame(&map, &objects, robot, frame_count);
        frame_count += 1;
    });

    objects.iter().fold(0, |acc, obj| acc + obj[0].0 as usize + obj[0].1 as usize * 100)
}

#[cfg(feature = "make_movie")]
fn make_frame(map: &HashSet<(i32, i32)>, objects: &Vec<Vec<(i32, i32)>>, robot: (i32, i32), frame_count: usize) {

    let (min_x, max_x, min_y, max_y) = map.iter().fold((0, 0, 0, 0), |(min_x, max_x, min_y, max_y), pos| {
        (min_x.min(pos.0), max_x.max(pos.0), min_y.min(pos.1), max_y.max(pos.1))
    });

    let size = (max_x + min_x + 2, max_y + min_y + 2);

    let colors: HashMap<usize, (u8, u8, u8, u8)> = objects
        .iter()
        .enumerate()
        .fold(HashMap::new(),  |mut hm, (i, _)| { 
            hm.insert( i, (100 + ((155.0 * i as f32) / (objects.len() as f32)) as u8, 0_u8, 0_u8, 255_u8)); hm  
        });

    let mut img: Image = Image::new(size.0 as usize, size.1 as usize, 8);
    img.clear((255_u8, 255_u8, 255_u8, 255_u8));

    map.iter().for_each(|(x, y)| {
        img.set_pixel(*x as usize, *y as usize, (0_u8, 0_u8, 0_u8, 255_u8));
    });

    objects.iter().enumerate().for_each(|(i, obj)| {
        obj.iter().for_each(|(x, y)| {
            img.set_pixel(*x as usize, *y as usize, colors[&i]);
        });
    });

    let part = if objects[0].len() > 1 {2} else {1};

    img.set_pixel(robot.0 as usize, robot.1 as usize, (0_u8, 0_u8, 255_u8, 255_u8));
    img.save_png(&format!("images/day15_24_part_{}_{:06}.png", part, frame_count));
}

#[allow(dead_code)]
pub fn run(real: bool, print_result: bool) -> (u128, u128, u128) {
    let start0 = Instant::now();

    let input_file: &str = if !real {
        "./input/day15-test.txt"
    } else {
        "./input/day15-real.txt"
    };
    let input = tools::get_input(String::from(input_file));
    let empty_line = input.iter().position(|r| r == "").unwrap();

    let mut map: HashSet<(i32, i32)> = HashSet::new();
    let mut objects: Vec<Vec<(i32, i32)>> = vec![];
    let mut start: (i32, i32) = (0, 0);
    input[..empty_line].iter().enumerate().for_each(|(y, line)| {
        line.chars().enumerate().for_each(|(x, c)| {
            let pos = (x as i32, y as i32);
            if c != '#' {
                map.insert(pos);
            }
            if c == 'O' {
                objects.push(vec![pos]);
            }
            if c == '@' {
                start = pos;
            }
        });
    });

    let wide_map: HashSet<(i32, i32)> = map.iter().fold(HashSet::<(i32,i32)>::new(), |mut set, pos| {
        set.insert((2 * pos.0 + 0, pos.1));
        set.insert((2 * pos.0 + 1, pos.1));
        set
    });

    let wide_objects: Vec<Vec<(i32, i32)>> = objects.iter().map(|obj| {
        vec![(2*obj[0].0+0, obj[0].1), (2*obj[0].0+1, obj[0].1)]
    }).collect(); 

    let wide_start = (2 * start.0, start.1);

    let operations = input[empty_line + 1..].iter().join("").chars().collect_vec();
    
    let after0 = Instant::now();

    let start1 = Instant::now();

    let res1 = do_operations(map.clone(), objects.clone(), start, &operations);

    let after1 = Instant::now();
    if print_result {
        println!("Part 1: {}", res1);
    }

    let start2 = Instant::now();

    let res2 = do_operations(wide_map.clone(), wide_objects.clone(), wide_start, &operations);

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

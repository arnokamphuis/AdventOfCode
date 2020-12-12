use super::tools;
use std::time::Instant;

#[allow(dead_code)]
pub fn run(real: bool, print_result: bool) -> (u128, u128, u128) {
    let start0 = Instant::now();

    let input_file: &str = if !real {
        "./input/day12_20_test.txt"
    } else {
        "./input/day12_20_real.txt"
    };
    let input = tools::get_input(String::from(input_file));

    let mut commands: Vec<(&str, i32)> = vec![];
    for line in &input {
        let cmd = &line[0..1];
        let len = line[1..].parse::<i32>().unwrap();
        commands.push((cmd,len));
    }

    let after0 = Instant::now();

    let start1 = Instant::now();

    let directions = vec![(1,0), (0,1), (-1,0), (0,-1)]; // Left is +1, Right is -1

    let res1 = commands
        .iter()
        .fold((0, (0,0)), |res, &step| {
            let dir = directions[res.0];
            let cur_step = match step.0 {
                "F" => ( dir.0 * step.1, dir.1 * step.1 ),
                "E" => ( step.1, 0),
                "N" => ( 0, step.1),
                "W" => ( -step.1, 0),
                "S" => ( 0, -step.1),
                _ => (0,0)
            };

            let new_dir: usize = match step.0 {
                "L" => {
                    let v = (res.0 as i32 + step.1 / 90) % 4;
                    v as usize
                }
                "R" => {
                    let mut v = (res.0 as i32 - step.1 / 90) % 4;
                    if v < 0 { v += 4 } 
                    v as usize
                }
                _ => res.0
            };
            ( new_dir, (res.1.0 + cur_step.0, res.1.1 + cur_step.1))
        });

    let after1 = Instant::now();
    if print_result {
        println!("Part 1: {}",  res1.1.0.abs() + res1.1.1.abs());
    }

    let start2 = Instant::now();

    let res2 = commands
        .iter()
        .fold(((0,0),(10,1)), |res, &step| {
            // ship is .0 , waypoint is .1
            let cur_step = match step.0 {
                "F" => (( step.1 * res.1.0, step.1 * res.1.1 ), (0,0)),
                "E" => ((0,0),( step.1, 0)),
                "N" => ((0,0),( 0, step.1)),
                "W" => ((0,0),( -step.1, 0)),
                "S" => ((0,0),( 0, -step.1)),
                _ => ((0,0),(0,0))
            };
            let mut new_pos = 
                (
                    (res.0.0+cur_step.0.0,res.0.1+cur_step.0.1),
                    (res.1.0+cur_step.1.0,res.1.1+cur_step.1.1)
                );

            if step.0 == "L" || step.0 == "R" {
                let mut rot_time = step.1/90 * if step.0 == "L" {1} else {-1};
                if rot_time < 0 { rot_time += 4;}
                let new_rel_pos: (i32,i32) = match rot_time as usize {
                    0 => ( res.1.0,  res.1.1),
                    1 => (-res.1.1,  res.1.0),
                    2 => (-res.1.0, -res.1.1),
                    3 => ( res.1.1, -res.1.0),
                    _ => (0,0)
                };
                new_pos = ((res.0.0, res.0.1), (new_rel_pos.0, new_rel_pos.1));
            }
            new_pos
        });

    let after2 = Instant::now();
    if print_result {
        println!("Part 2: {}",  res2.0.0.abs() + res2.0.1.abs());
    }

    (
        after0.duration_since(start0).as_nanos(),
        after1.duration_since(start1).as_nanos(),
        after2.duration_since(start2).as_nanos(),
    )
}

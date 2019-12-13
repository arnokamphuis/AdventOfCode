use super::intcode::IntCodeComputer;
use super::tools;
use super::tools::Image;
use std::collections::BTreeMap;
use std::time::Instant;

fn process_score(game: &mut IntCodeComputer) -> ((i64, i64), (i64, i64), i64) {
    let mut count = 0;
    let mut out_triple = vec![0; 3];

    let mut ball_pos: (i64, i64) = (0, 0);
    let mut paddle_pos: (i64, i64) = (0, 0);
    let mut score = 0;

    while let Some(o) = game.get_output() {
        out_triple[count] = o;
        count += 1;
        count = count % 3;

        if count == 0 {
            if out_triple[0] == -1 && out_triple[1] == 0 {
                // score
                score = out_triple[2];
            } else if out_triple[2] == 3 {
                // paddle
                paddle_pos.0 = out_triple[0];
                paddle_pos.1 = out_triple[1];
            } else if out_triple[2] == 4 {
                // ball
                ball_pos.0 = out_triple[0];
                ball_pos.1 = out_triple[1];
            }
        }
    }
    (ball_pos, paddle_pos, score)
}

#[allow(dead_code)]
pub fn run() {
    println!("Day 13 of 2019");

    let start0 = Instant::now();

    let input_file = "./input/day13_19_real.txt";
    let input = tools::get_input(String::from(input_file));

    let line = &input[0];
    let command_strings: Vec<&str> = line.split(",").collect();
    let mut commands: BTreeMap<i64, i64> = BTreeMap::new();
    command_strings
        .iter()
        .filter_map(|s| s.parse::<i64>().ok())
        .enumerate()
        .for_each(|(i, c)| {
            commands.insert(i as i64, c);
        });

    let after0 = Instant::now();
    println!("Init in {:?}", after0.duration_since(start0));

    let start1 = Instant::now();

    let mut img = Image::new(35, 25, 10);

    let mut game1 = IntCodeComputer::new(&commands);
    game1.run();

    let mut output: Vec<i64> = vec![];
    let mut c = 0;
    let mut c_blocks = 0;
    while let Some(o) = game1.get_output() {
        output.push(o);
        if (c % 3) == 2 && c > 0 {
            let pos = (output[c - 2], output[c - 1]);
            img.set_pixel(pos.0 as usize, pos.1 as usize, ((50 * o) as u8, 0, 0, 255));
            if o == 2 {
                c_blocks += 1;
            }
        }
        c += 1;
    }
    img.save_png(&String::from("game.png"));

    let after1 = Instant::now();
    println!(
        "Part 1: {}, in {:?}",
        c_blocks,
        after1.duration_since(start1)
    );

    let start2 = Instant::now();

    let mut game2 = IntCodeComputer::new(&commands);
    game2.set_mem(0, super::intcode::Mode::IMM, 2);

    game2.run();

    let mut score = 0;

    let mut finished = false;
    while !finished {
        finished = game2.run();

        let process_result = process_score(&mut game2);
        let ball_pos = process_result.0;
        let paddle_pos = process_result.1;
        score = process_result.2;

        let move_dir = if paddle_pos.0 < ball_pos.0 {
            1
        } else if paddle_pos.0 > ball_pos.0 {
            -1
        } else {
            0
        };

        game2.add_input(move_dir);
    }

    let after2 = Instant::now();
    println!("Part 2: {}, in {:?}", score, after2.duration_since(start2));
}

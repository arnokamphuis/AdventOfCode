use super::intcode::IntCodeComputer;
use super::tools;
use super::tools::Image;
use std::collections::BTreeMap;
use std::time::Instant;

struct Game {
    computer: IntCodeComputer,
    ball_pos: (i64, i64),
    paddle_pos: (i64, i64),
    score: i64,
    frame_counter: usize,
    img: Image,
}

impl Game {
    fn new(commands: &BTreeMap<i64, i64>) -> Game {
        Game {
            computer: IntCodeComputer::new(commands),
            ball_pos: (0, 0),
            paddle_pos: (0, 0),
            score: 0,
            frame_counter: 0,
            img: Image::new(35, 25, 40),
        }
    }

    fn process_score(&mut self, initial: bool, make_movie: bool) {
        let mut count = 0;
        let mut out_triple = vec![0; 3];

        while let Some(o) = self.computer.get_output() {
            out_triple[count] = o;
            count += 1;
            count = count % 3;

            if initial && make_movie {
                let pos = (out_triple[0], out_triple[1]);
                if pos.0 >= 0 {
                    let tile_type = out_triple[2];
                    match tile_type {
                        0 => {
                            self.img
                                .set_pixel(pos.0 as usize, pos.1 as usize, (0, 0, 0, 255));
                        }
                        1 => {
                            self.img
                                .set_pixel(pos.0 as usize, pos.1 as usize, (255, 0, 0, 255));
                        }
                        2 => {
                            self.img
                                .set_pixel(pos.0 as usize, pos.1 as usize, (255, 255, 0, 255));
                        }
                        3 => {
                            self.img
                                .set_pixel(pos.0 as usize, pos.1 as usize, (0, 0, 255, 255));
                        }
                        4 => {
                            self.img
                                .set_pixel(pos.0 as usize, pos.1 as usize, (255, 0, 255, 255));
                        }
                        _ => {}
                    }
                }
            }

            if count == 0 {
                if out_triple[0] == -1 && out_triple[1] == 0 {
                    // score
                    self.score = out_triple[2];
                } else if out_triple[2] == 0 {
                    self.img.set_pixel(
                        out_triple[0] as usize,
                        out_triple[1] as usize,
                        (0, 0, 0, 255),
                    );
                } else if out_triple[2] == 3 {
                    if make_movie {
                        // paddle
                        self.img.set_pixel(
                            self.paddle_pos.0 as usize,
                            self.paddle_pos.1 as usize,
                            (0, 0, 0, 255),
                        );
                    }

                    self.paddle_pos.0 = out_triple[0];
                    self.paddle_pos.1 = out_triple[1];

                    if make_movie {
                        self.img.set_pixel(
                            self.paddle_pos.0 as usize,
                            self.paddle_pos.1 as usize,
                            (0, 0, 255, 255),
                        );
                    }
                } else if out_triple[2] == 4 {
                    // ball
                    if make_movie {
                        self.img.set_pixel(
                            self.ball_pos.0 as usize,
                            self.ball_pos.1 as usize,
                            (0, 0, 0, 255),
                        );
                    }

                    self.ball_pos.0 = out_triple[0];
                    self.ball_pos.1 = out_triple[1];

                    if make_movie {
                        self.img.set_pixel(
                            self.ball_pos.0 as usize,
                            self.ball_pos.1 as usize,
                            (255, 0, 255, 255),
                        );
                    }
                }
            }
        }
        if make_movie {
            self.img.save_png(&String::from(format!(
                "movie/frame{:05}.png",
                self.frame_counter
            )));
        }
        self.frame_counter += 1;

        if self.frame_counter % 100 == 0 {
            println!("frame counter: {} ", self.frame_counter);
        }
    }

    fn play(&mut self) -> i64 {
        let mut score = 0;

        self.computer.set_mem(0, super::intcode::Mode::IMM, 2);

        let mut finished = false;
        let mut initial = true;
        while !finished {
            finished = self.computer.run();
            let process_result = self.process_score(initial, false);
            initial = false;
            let move_dir = if self.paddle_pos.0 < self.ball_pos.0 {
                1
            } else if self.paddle_pos.0 > self.ball_pos.0 {
                -1
            } else {
                0
            };
            self.computer.add_input(move_dir);
        }
        self.score
    }
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

    let mut game1 = IntCodeComputer::new(&commands);
    game1.run();

    let mut output: Vec<i64> = vec![];
    let mut c = 0;
    let mut c_blocks = 0;
    while let Some(o) = game1.get_output() {
        if (c % 3) == 2 && o == 2 {
            c_blocks += 1;
        }
        c += 1;
    }

    let after1 = Instant::now();
    println!(
        "Part 1: {}, in {:?}",
        c_blocks,
        after1.duration_since(start1)
    );

    let start2 = Instant::now();

    // let mut img = Image::new(35, 25, 10);
    //    img.set_pixel(pos.0 as usize, pos.1 as usize, ((50 * o) as u8, 0, 0, 255));
    //    img.save_png(&String::from("game.png"));
    let mut game = Game::new(&commands);
    let res2 = game.play();

    let after2 = Instant::now();
    println!("Part 2: {}, in {:?}", res2, after2.duration_since(start2));
}

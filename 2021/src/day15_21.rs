use super::tools;
use super::maze;
use std::time::Instant;
use maze::Maze;

#[allow(dead_code)]
pub fn run(real: bool, print_result: bool) -> (u128, u128, u128) {
    let start0 = Instant::now();

    let input_file: &str = if !real {
        "./input/day15_21_test.txt"
    } else {
        "./input/day15_21_real.txt"
    };
    let input = tools::get_input(String::from(input_file));

    let mut maze = input.iter().fold(Maze::new(), |mut maze, line| {maze.add_line(line); maze });
    let (maxx,maxy) = maze.get_size();

    let mut big_maze = maze.clone();
    big_maze.grow(5);
    let (bigx,bigy) = big_maze.get_size();

    let after0 = Instant::now();

    let start1 = Instant::now();

    let res1 = maze.length_shortest((0,0), (maxx-1, maxy-1));

    let after1 = Instant::now();
    if print_result {
        println!("Part 1: {}", res1);
    }

    let start2 = Instant::now();

    let res2 = big_maze.length_shortest((0,0), (bigx-1, bigy-1));

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

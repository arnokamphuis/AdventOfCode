use super::tools;
use std::time::Instant;

#[derive(Debug, Copy, Clone)]
struct Board {
    numbers: [[i16; 5]; 5],
    marked: [[bool; 5]; 5],
    finished: bool,
}

impl Board {
    fn new() -> Board {
        Board {
            numbers: [[0; 5]; 5],
            marked: [[false; 5]; 5],
            finished: false,
        }
    }

    fn call(&mut self, num: i16) -> Option<i16> {
        if !self.finished {
            for i in 0..5usize {
                for j in 0..5usize {
                    if self.numbers[i][j] == num {
                        self.marked[i][j] = true;
                    }
                }
            }
        }

        if self.check() {
            self.finished = true;
            let mut summation = 0;
            for i in 0..5usize {
                for j in 0..5usize {
                    if !self.marked[i][j] {
                        summation += self.numbers[i][j];
                    }
                }
            }
            return Some(summation);
        }
        None
    }

    fn check(&self) -> bool {
        for i in 0..5usize {
            if (self.marked[i][0]
                && self.marked[i][1]
                && self.marked[i][2]
                && self.marked[i][3]
                && self.marked[i][4])
                || (self.marked[0][i]
                    && self.marked[1][i]
                    && self.marked[2][i]
                    && self.marked[3][i]
                    && self.marked[4][i])
            {
                return true;
            }
        }

        false
    }
}

#[allow(dead_code)]
pub fn run(real: bool, print_result: bool) -> (u128, u128, u128) {
    let start0 = Instant::now();

    let input_file: &str = if !real {
        "./input/day04_21_test.txt"
    } else {
        "./input/day04_21_real.txt"
    };
    let input = tools::get_input(String::from(input_file));

    let mut called: Vec<i16> = vec![];
    let mut tokens = input[0].split(',');
    while let Some(numtoken) = tokens.next() {
        called.push(numtoken.parse().unwrap());
    }

    let mut boards: Vec<Board> = vec![];

    let mut rowcount: usize = 0;
    let mut current: Board = Board::new();
    input.iter().skip(2).for_each(|line| {
        if line.len() > 0 {
            let mut tokens = line.split_whitespace();
            for j in 0..5usize {
                current.numbers[rowcount][j] = tokens.next().unwrap().parse().unwrap();
            }
            rowcount += 1;
            if rowcount == 5 {
                boards.push(current);
                current = Board::new();
                rowcount = 0;
            }
        }
    });

    let boardcount = boards.len();

    let mut finishedcount = 0;

    let mut run = |targetcount: usize| -> i16 {
        loop {
            let number = called[0];
            for board in boards.iter_mut() {
                if !board.finished {
                    if let Some(value) = board.call(number) {
                        finishedcount += 1;
                        if finishedcount == targetcount {
                            return value * number;
                        }
                    }
                }
            }
            called.remove(0);
        }
    };

    let after0 = Instant::now();

    let start1 = Instant::now();

    let res1 = run(1);

    let after1 = Instant::now();
    if print_result {
        println!("Part 1: {}", res1);
    }

    let start2 = Instant::now();

    let res2 = run(boardcount);

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

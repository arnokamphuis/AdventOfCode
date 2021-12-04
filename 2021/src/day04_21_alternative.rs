use super::tools;
use std::collections::BTreeMap;
use std::time::Instant;

#[derive(Debug, Clone)]
struct Board {
    numbers: BTreeMap<i16, (usize, usize)>,
    marked: [[bool; 5]; 5],
    finished: bool,
}

impl Board {
    fn new() -> Board {
        Board {
            numbers: BTreeMap::new(),
            marked: [[false; 5]; 5],
            finished: false,
        }
    }

    fn call(&mut self, num: i16) -> Option<i16> {
        if !self.finished {
            if self.numbers.contains_key(&num) {
                let coor = self.numbers[&num];
                self.numbers.remove(&num);
                self.marked[coor.0][coor.1] = true;
                if self.check(coor) {
                    self.finished = true;
                    return Some(self.numbers.keys().sum());
                }
            }
        }
        None
    }

    fn check(&self, coor: (usize, usize)) -> bool {
        if (self.marked[coor.0][0]
            && self.marked[coor.0][1]
            && self.marked[coor.0][2]
            && self.marked[coor.0][3]
            && self.marked[coor.0][4])
            || (self.marked[0][coor.1]
                && self.marked[1][coor.1]
                && self.marked[2][coor.1]
                && self.marked[3][coor.1]
                && self.marked[4][coor.1])
        {
            return true;
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
                current
                    .numbers
                    .insert(tokens.next().unwrap().parse().unwrap(), (rowcount, j));
            }
            rowcount += 1;
            if rowcount == 5 {
                boards.push(current.clone());
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

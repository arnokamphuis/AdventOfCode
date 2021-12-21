use super::tools;
use std::time::Instant;
use std::collections::HashMap;

struct Dirac {
    solutions: HashMap<(u64, u64, u64, u64), (u64, u64)>,
}

impl Dirac {
    fn new() -> Dirac { Dirac{ solutions: HashMap::new(), } }

    fn count_wins(&mut self, pos1: u64, pos2: u64, score1: u64, score2:u64 ) -> (u64, u64) {
        if score1 >= 21 { return (1,0) }
        if score2 >= 21 { return (0,1) }
        if self.solutions.contains_key(&(pos1,pos2,score1,score2)) {
            return self.solutions[&(pos1,pos2,score1,score2)];
        }

        let mut wins = (0,0);
        (1..=3).for_each(|roll1| {
            (1..=3).for_each(|roll2| {
                (1..=3).for_each(|roll3| {
                    let mut new_pos = pos1 + roll1 + roll2 + roll3;
                    if new_pos > 10 { new_pos -= 10; }
                    let new_score = score1 + new_pos;

                    let wins_once = self.count_wins(pos2, new_pos, score2, new_score);
                    wins = (wins.0 + wins_once.1, wins.1 + wins_once.0);
                });    
            });
        });
        self.solutions.insert((pos1,pos2,score1,score2),wins);
        return wins;
    }
}

#[allow(dead_code)]
pub fn run(real: bool, print_result: bool) -> (u128, u128, u128) {
    let start0 = Instant::now();

    let input_file: &str = if !real {
        "./input/day21_21_test.txt"
    } else {
        "./input/day21_21_real.txt"
    };
    let input = tools::get_input(String::from(input_file));

    let mut pos:[u64;2] = [input[0][28..].parse::<u64>().unwrap(), input[1][28..].parse::<u64>().unwrap()];
    let mut score:[u64;2] = [0;2];
    let mut turn = 0;
    let mut die_pos = 0;

    let after0 = Instant::now();

    let start1 = Instant::now();

    let mut rolls = 0;
    while score[0] < 1000 && score[1] < 1000 {
        (0..3).for_each(|_| {
            die_pos += 1;
            if die_pos > 100 { die_pos -= 100;}
            pos[turn] += die_pos;
        });
        while pos[turn] > 10 { pos[turn] -= 10; }
        score[turn] += pos[turn];

        rolls += 3;
        turn = (turn + 1) % 2;
    }
    let res1:u64 = score[turn] * rolls;

    let after1 = Instant::now();
    if print_result {
        println!("Part 1: {}", res1);
    }

    let start2 = Instant::now();

    pos = [input[0][28..].parse::<u64>().unwrap(), input[1][28..].parse::<u64>().unwrap()];
    let res = Dirac::new().count_wins(pos[0],pos[1],0,0);
    let res2 = res.0.max(res.1);

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

use super::tools;
use std::time::Instant;
use std::collections::HashMap;

#[derive(Clone, Debug)]
enum Operation {
    Plus,
    Times,
    Divide,
    Minus,
}

#[derive(Clone, Debug)]
struct MonkeyBusiness {
    to_be_done: HashMap<(Vec<char>,Vec<char>), (Vec<char>,Operation)>,
    yelled: HashMap<Vec<char>, i64>,
}

impl MonkeyBusiness {
    fn run(&mut self) -> i64 {

        let mut last_yelled = 0;

        while self.to_be_done.len() > 0 {
            let tbd = self.to_be_done.clone();
            let doable = tbd.iter().filter(|((n1,n2),(n,op))| {
                    self.yelled.contains_key(n1) && self.yelled.contains_key(n2)
                })
                // .map(|((n1,n2),(n,op))| ((n1.clone(),n2.clone()),(n.clone(),op)))
                .collect::<Vec<_>>();
        
            doable.iter().for_each(|((n1,n2),(n,op))| {
                let v1 = self.yelled[n1];
                let v2 = self.yelled[n2];
                let res = match op {
                    Operation::Minus => v1 - v2,
                    Operation::Plus => v1 + v2,
                    Operation::Times => v1 * v2,
                    Operation::Divide => v1 / v2,
                };
                last_yelled = res;
                self.yelled.insert(n.clone(), res);
                self.to_be_done.remove(&(n1.clone(),n2.clone()));
            });
        }
        last_yelled
    }
}

#[allow(dead_code)]
pub fn run(real: bool, print_result: bool) -> (u128, u128, u128) {
    let start0 = Instant::now();

    let input_file: &str = if !real {
        "./input/day21_22_test.txt"
    } else {
        "./input/day21_22_real.txt"
    };
    let input = tools::get_input(String::from(input_file));

    let mut mb = MonkeyBusiness {
        to_be_done: HashMap::new(),
        yelled: HashMap::new(),
    };

    input.iter().for_each(|line| {
        let words = line.split_whitespace().collect::<Vec<&str>>();
        let name = &words[0][0..4].chars().collect::<Vec<char>>();
        if words.len() > 2 {
            if let Some(c) = words[2].chars().nth(0) {
                let op = match c {
                    '*' => Operation::Times,
                    '+' => Operation::Plus,
                    '/' => Operation::Divide,
                    '-' => Operation::Minus,
                    _ => panic!()
                };
                let name1 = &words[1][0..4].chars().collect::<Vec<char>>();
                let name2 = &words[3][0..4].chars().collect::<Vec<char>>();
                mb.to_be_done.insert((name1.clone(), name2.clone()), (name.clone(), op));
            }
        } else {
            mb.yelled.insert(name.clone(), words[1].parse::<i64>().unwrap());
        }
    });
    // println!("{:?}", mb);

    let after0 = Instant::now();

    let start1 = Instant::now();

    let res1 = mb.run();

    let after1 = Instant::now();
    if print_result {
        println!("Part 1: {}", res1);
    }

    let start2 = Instant::now();

    let after2 = Instant::now();
    if print_result {
        println!("Part 2: {}", 0);
    }

    (
        after0.duration_since(start0).as_nanos(),
        after1.duration_since(start1).as_nanos(),
        after2.duration_since(start2).as_nanos(),
    )
}

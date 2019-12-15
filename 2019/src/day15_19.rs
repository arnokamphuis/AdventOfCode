use super::intcode::get_commands_from_line;
use super::intcode::IntCodeComputer;
use super::tools;
use queues::*;
use std::collections::BTreeMap;
use std::time::Instant;

struct Cleaner {
    commands: BTreeMap<i64, i64>,
    env: BTreeMap<(i64, i64), bool>,
    pos: (i64, i64),
    q: Queue<(i64, i64)>,
    paths: BTreeMap<(i64, i64), Vec<usize>>,
    oxygen: (i64, i64),
}

impl Cleaner {
    fn new(m: &BTreeMap<i64, i64>) -> Cleaner {
        Cleaner {
            commands: m.clone(),
            env: BTreeMap::new(),
            pos: (0, 0),
            q: Queue::new(),
            paths: BTreeMap::new(),
            oxygen: (0, 0),
        }
    }

    #[allow(dead_code)]
    fn print_env(&self) {
        let mut minx = std::i64::MAX;
        let mut maxx = std::i64::MIN;
        let mut miny = std::i64::MAX;
        let mut maxy = std::i64::MIN;
        for (p, _) in &self.env {
            minx = std::cmp::min(minx, p.0);
            maxx = std::cmp::max(maxx, p.0);
            miny = std::cmp::min(miny, p.1);
            maxy = std::cmp::max(maxy, p.1);
        }

        println!("----------------------------------------------------------------------");
        for y in miny..=maxy {
            for x in minx..=maxx {
                if !self.env.contains_key(&(x, y)) {
                    print!(" ");
                } else if self.env[&(x, y)] {
                    print!("#");
                } else {
                    if (x, y) == self.oxygen {
                        print!("O");
                    } else {
                        print!(".");
                    }
                }
            }
            print!("\n");
        }
        println!("----------------------------------------------------------------------");
    }

    fn search(&mut self) -> u64 {
        self.env.insert(self.pos, false);
        self.q.add(self.pos).unwrap();
        self.paths.insert(self.pos, vec![]);

        // north (1), south (2), west (3), and east (4)
        let directions = [(0, 1), (0, -1), (-1, 0), (1, 0)];
        while self.q.size() > 0 {
            if let Ok(c_pos) = self.q.remove() {
                let path = self.paths[&c_pos].clone();

                for dir in 1i64..=4 {
                    let dir_vec = directions[dir as usize - 1];
                    let mut new_pos = c_pos;
                    new_pos.0 += dir_vec.0;
                    new_pos.1 += dir_vec.1;
                    let new_dist = path.len() + 1;

                    if !(self.paths.contains_key(&new_pos) && self.paths[&new_pos].len() < new_dist)
                    {
                        if !(self.env.contains_key(&new_pos) && self.env[&new_pos]) {
                            let mut computer = IntCodeComputer::new(&self.commands);
                            for index in 0..path.len() {
                                computer.add_input(path[index] as i64);
                                computer.run();
                                let tempres = computer.get_output().unwrap();
                                assert!(tempres == 1, format!("PATH NOT FREE {}", tempres));
                            }

                            computer.add_input(dir);
                            computer.run();
                            let status = computer.get_output().unwrap();
                            match status {
                                2 => {
                                    self.oxygen = new_pos;
                                    let mut new_path = path.clone();
                                    new_path.push(dir as usize);
                                    self.paths.insert(new_pos, new_path);
                                    self.env.insert(new_pos, false);
                                }
                                1 => {
                                    let mut new_path = path.clone();
                                    new_path.push(dir as usize);
                                    self.paths.insert(new_pos, new_path);
                                    self.q.add(new_pos).unwrap();
                                    self.env.insert(new_pos, false);
                                }
                                0 => {
                                    self.env.insert(new_pos, true);
                                }
                                _ => {
                                    panic!("THIS SHOULD NOT HAPPEN");
                                }
                            }
                        }
                    }
                }
            }
        }

        self.paths[&self.oxygen].len() as u64
    }

    fn calculate_fill_time(&mut self) -> u64 {
        let mut res: u64 = 0;

        // north (1), south (2), west (3), and east (4)
        let directions = [(0, 1), (0, -1), (-1, 0), (1, 0)];

        let mut ox_q: Queue<(i64, i64)> = Queue::new();
        let mut dist: BTreeMap<(i64, i64), u64> = BTreeMap::new();
        let mut visited: Vec<(i64, i64)> = vec![];

        ox_q.add(self.oxygen).unwrap();
        dist.insert(self.oxygen, 0);

        while let Ok(cur_pos) = ox_q.remove() {
            visited.push(cur_pos);
            let cur_dist = dist[&cur_pos];

            for dir in 0..4 {
                let new_pos = (cur_pos.0 + directions[dir].0, cur_pos.1 + directions[dir].1);
                if self.env.contains_key(&new_pos) && !self.env[&new_pos] {
                    if !visited.contains(&new_pos) {
                        ox_q.add(new_pos).unwrap();
                        dist.insert(new_pos, cur_dist + 1);
                    } else {
                        if (cur_dist + 1) < dist[&new_pos] {
                            ox_q.add(new_pos).unwrap();
                            dist.entry(new_pos).and_modify(|d| *d = cur_dist + 1);
                        }
                    }
                }
            }
        }

        for (_, d) in dist {
            res = std::cmp::max(res, d);
        }

        res
    }
}

#[allow(dead_code)]
pub fn run() {
    println!("Day 15 of 2019");

    let start0 = Instant::now();

    let input_file = "./input/day15_19_real.txt";
    let input = tools::get_input(String::from(input_file));
    let commands = get_commands_from_line(&input[0]);

    let after0 = Instant::now();
    println!("Init in {:?}", after0.duration_since(start0));

    let start1 = Instant::now();

    let mut cleaner1 = Cleaner::new(&commands);
    let res1 = cleaner1.search();

    let after1 = Instant::now();
    println!("Part 1: {}, in {:?}", res1, after1.duration_since(start1));

    let start2 = Instant::now();

    let res2 = cleaner1.calculate_fill_time();

    let after2 = Instant::now();
    println!("Part 2: {}, in {:?}", res2, after2.duration_since(start2));
}

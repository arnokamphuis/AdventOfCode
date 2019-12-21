use super::tools;
use std::cmp::Ordering;
use std::collections::BTreeMap;
use std::time::Instant;
use std::collections::BinaryHeap;
use queues::*;

#[derive(Clone, Eq, PartialEq)]
struct Robots {
    loc: Vec<char>,
}

impl Robots {
    fn new(v: &Vec<char>) -> Robots {
        Robots { 
            loc: v.clone(),
        }
    }
}

#[derive(Clone, Eq, PartialEq)]
struct State {
    distance: i64,
    robots: Robots,
    has_keys: Vec<char>,
}

impl State {
    #[allow(dead_code)]
    fn print(&self) {
        print!("State ( distance: {}, robots: ", self.distance);
        for r in &self.robots.loc {
            print!(" {}", r);
        }
        print!(", keys: ");
        for k in &self.has_keys {
            print!(" {:?} ", k);
        }
        println!(")");
    }
}

impl Ord for State {
    fn cmp(&self, other: &State) -> Ordering {
        other.distance.cmp(&self.distance).then(
            self.has_keys.len().cmp(&other.has_keys.len())
        )
        // self.has_keys.len().cmp(&other.has_keys.len()).then(
        //     other.distance.cmp(&self.distance)
        // )
    }
}

impl PartialOrd for State {
    fn partial_cmp(&self, other: &State) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

struct Cave {
    map: BTreeMap<(i64, i64), u8>,
    keys: BTreeMap<char, (i64, i64)>,
    doors: BTreeMap<char, (i64, i64)>,
    pos_keys: BTreeMap<(i64, i64), char>,
    pos_doors: BTreeMap<(i64, i64), char>,
    paths: BTreeMap<char, Vec<(char, i64, Vec<char>, Vec<char>)>>,
    paths_pair: BTreeMap<(char,char), (i64, Vec<char>, Vec<char>)>,
    start: Vec<(i64, i64)>,
}

impl Cave {
    fn new() -> Cave {
        Cave {
            map: BTreeMap::new(),
            keys: BTreeMap::new(),
            doors: BTreeMap::new(),
            pos_keys: BTreeMap::new(),
            pos_doors: BTreeMap::new(),
            paths: BTreeMap::new(),
            paths_pair: BTreeMap::new(),
            start: vec![],
        }
    }

    fn scan(&mut self, input: &Vec<String>) {
        let mut pos = (0, 0);
        for line in input {
            for c in line.chars() {
                let value;
                match c {
                    '#' => value = 1,
                    '.' => value = 0,
                    _ => {
                        let cu = c as u8;
                        if cu >= 65 && cu <= 90 {
                            // door
                            self.doors.insert((cu + 32) as char, pos);
                            self.pos_doors.insert(pos, (cu + 32) as char);
                        } else if cu >= 97 && cu <= 122 {
                            self.keys.insert(c, pos);
                            self.pos_keys.insert(pos, c);
                        } else if cu == 64 {
                            self.start.push(pos);
                        }
                        value = 0;
                    }
                }
                self.map.insert(pos, value);
                pos.0 += 1;
            }
            pos.0 = 0;
            pos.1 += 1;
        }
    }

    fn find_reachable(
        &mut self,
        from: (i64, i64),
        available: &Vec<char>,
    ) -> Vec<(char, i64, Vec<char>, Vec<char>)> {
        let mut res: Vec<(char, i64, Vec<char>, Vec<char>)> = vec![];

        // close all the doors for which the key is unavailable
        for (d, pos) in &self.doors {
            if !available.contains(&d) {
                self.map.entry(*pos).and_modify(|m| *m = 1);
            }
        }

        let mut q: Queue<(i64,i64)> = queue![];
        let mut visited: Vec<(i64, i64)> = vec![];
        let mut distance: BTreeMap<(i64, i64), i64> = BTreeMap::new();
        let mut prev: BTreeMap<(i64, i64), (i64, i64)> = BTreeMap::new();
        q.add(from).unwrap();
        distance.insert(from, 0);

        let directions = vec![(-1, 0), (0, -1), (1, 0), (0, 1)];
        while let Ok(pos) = q.remove() {
            visited.push(pos);
            let dist = distance[&pos];

            if self.pos_keys.contains_key(&pos) {
                res.push((self.pos_keys[&pos], dist, vec![], vec![]));
            }

            for i in 0..4 {
                let dir = directions[i];
                let new_pos = (pos.0 + dir.0, pos.1 + dir.1);
                let vtd = visited.contains(&new_pos);
                if vtd && (dist + 1) < distance[&new_pos] {
                    println!("SHORTER FOUND");
                } else if !vtd {
                    if self.map.contains_key(&new_pos) && self.map[&new_pos] == 0 {
                        prev.insert(new_pos, pos);
                        q.add(new_pos).unwrap();
                        distance.insert(new_pos, dist + 1);
                    }
                }
            }
        }

        for r in &mut res {
            let mut keys_also = vec![];
            let mut doors_also = vec![];
            let mut path_pos = self.keys[&r.0];
            while path_pos != from {
                path_pos = prev[&path_pos];

                if self.pos_keys.contains_key(&path_pos) {
                    let key = self.pos_keys[&path_pos];
                    if !keys_also.contains(&key) {
                        keys_also.push(key);
                    }
                }

                if self.pos_doors.contains_key(&path_pos) {
                    let key = self.pos_doors[&path_pos];
                    if !doors_also.contains(&key) {
                        doors_also.push(key);
                    }
                }

            }
            r.2 = keys_also;
            r.3 = doors_also;
        }

        // reset all the doors
        for (d, pos) in &self.doors {
            if !available.contains(&d) {
                self.map.entry(*pos).and_modify(|m| *m = 0);
            }
        }

        res
    }


    fn find_all_pairs(&mut self) {
        let available: Vec<char> = self.keys.keys().map(|c| *c).collect();
        for (k,from) in &self.keys.clone() {
            self.paths.insert(*k, vec![]);
            let targets = self.find_reachable(*from, &available);
            for target in targets {
                if *k != target.0 {
                    self.paths.entry(*k).and_modify(|v| v.push( (target.0, target.1, target.2.clone(), target.3.clone()) ));

                    if !self.paths_pair.contains_key(&(*k,target.0)) {
                        self.paths_pair.insert( (*k, target.0), (target.1, target.2.clone(), target.3.clone()) );
                    }
                }
            }
        }

        let mut robot_counter = 1;
        for r in &self.start.clone() {
            let robot_key = std::char::from_digit(robot_counter,10).unwrap();
            let targets = self.find_reachable(*r, &available);
            for target in targets {
                if !self.paths.contains_key(&robot_key) {
                    self.paths.insert(robot_key, vec![]);
                }
                self.paths.entry(robot_key).and_modify(|v| v.push( (target.0, target.1, target.2.clone(), target.3.clone()) ));

                if !self.paths_pair.contains_key(&(robot_key,target.0)) {
                    self.paths_pair.insert( (robot_key, target.0), (target.1, target.2.clone(), target.3.clone()) );
                }
            }
            robot_counter+=1;
        }

        // for (p,d) in &self.paths_pair {
        //     println!("{:?}, {}", p, d.0);
        // }
    }

    fn get_keys(&mut self) -> i64 {
        let mut heap: BinaryHeap<State> = BinaryHeap::new();

        let start_state = State { distance: 0, robots: Robots::new(&vec!['1','2','3','4']), has_keys: vec![]};
        heap.push(start_state);

        let mut visited: Vec<(Robots,Vec<char>)> = vec![];
        let mut already_in_heap: Vec<(Robots,Vec<char>)> = vec![];
        
        let mut counter = 0;
        while let Some(s) = heap.pop() {

            let mut current_keys: Vec<char> = s.robots.loc.clone();
            for k in &s.has_keys {
                if !current_keys.contains(k) {
                    current_keys.push(*k);
                }
            }
            current_keys.sort();

            if current_keys.len() == (self.keys.len()+self.start.len()) {
                return s.distance;
            } else {
                // s.print();
                visited.push( (s.robots.clone(), s.has_keys.clone()) );

                let mut robot_index = 0;
                for r in &s.robots.loc.clone() {
                    let candidates = self.paths[r].clone();

                    for candidate in candidates {
                        if !current_keys.contains(&candidate.0) {

                            // check if for this path the correct keys are already in possesion
                            let mut valid = true;
                            for door in &candidate.3 {
                                if !current_keys.contains(door) {
                                    valid = false;
                                }
                            }
                            let mut keys = current_keys.clone();
                            // if valid consider this candidate
                            if valid {
                                // add the found keys to the list
                                for k in &candidate.2 {
                                    if !keys.contains(k) {
                                        keys.push(*k);
                                    }
                                }
                                keys.sort();

                                let mut rs: Robots = s.robots.clone();
                                rs.loc[robot_index] = candidate.0;

                                if !already_in_heap.contains( &(rs.clone(), keys.clone()) ) {
                                    // if this candidate has not been visited with the correct keys consider this
                                    if !visited.contains(&(rs.clone(), keys.clone())) {

                                        let new_state = State { distance: s.distance + candidate.1, robots: rs.clone(), has_keys: keys.clone() };
                                        // print!("Adding the following state ");
                                        // new_state.print();
                                        heap.push(new_state);
                                        already_in_heap.push( (rs.clone(), keys.clone()) );
                                        // println!("Current heap size = {}", heap.len());
                                    }
                                }
                            }

                        }
                    }
                    robot_index += 1;
                }
            }
        }
        -1
    }

}

#[allow(dead_code)]
pub fn run() {
    println!("Day 18 of 2019");

    let start0 = Instant::now();

    // let input_file = "./input/day18_19_test2.txt";
    let input_file = "./input/day18_19_real2.txt";
    let input = tools::get_input(String::from(input_file));

    let mut cave = Cave::new();
    cave.scan(&input);

    let after0 = Instant::now();
    println!("Init in {:?}", after0.duration_since(start0));

    let start1 = Instant::now();

    let after1 = Instant::now();
    println!("Part 1: {}, in {:?}", 0, after1.duration_since(start1));

    let start2 = Instant::now();

    cave.find_all_pairs();
    let res2 = cave.get_keys();

    let after2 = Instant::now();
    println!("Part 2: {}, in {:?}", res2, after2.duration_since(start2));
}

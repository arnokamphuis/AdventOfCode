use super::tools;
use std::cmp::Ordering;
use std::collections::BTreeMap;
use std::time::Instant;
use std::collections::BinaryHeap;
use queues::*;

#[derive(Clone, Eq, PartialEq)]
struct State {
    distance: i64,
    key: char,
    has_keys: Vec<char>,
}

impl State {
    #[allow(dead_code)]
    fn print(&self) {
        print!("State ( distance: {}, key: {}, has_keys:", self.distance, self.key);
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
    start: (i64, i64),
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
            start: (0, 0),
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
                            self.start = pos;
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
            let targets = self.find_reachable(*from, &available);
            for target in targets {
                if *k != target.0 {
                    if !self.paths.contains_key(k) {
                        self.paths.insert(*k, vec![]);
                    }
                    self.paths.entry(*k).and_modify(|v| v.push( (target.0, target.1, target.2.clone(), target.3.clone()) ));

                    if !self.paths_pair.contains_key(&(*k,target.0)) {
                        self.paths_pair.insert( (*k, target.0), (target.1, target.2.clone(), target.3.clone()) );
                    }
                }
            }
        }

        let targets = self.find_reachable(self.start, &vec![]);
        for target in targets {
            if !self.paths.contains_key(&' ') {
                self.paths.insert(' ', vec![]);
            }
            self.paths.entry(' ').and_modify(|v| v.push( (target.0, target.1, target.2.clone(), target.3.clone()) ));

            if !self.paths_pair.contains_key(&(' ',target.0)) {
                self.paths_pair.insert( (' ', target.0), (target.1, target.2.clone(), target.3.clone()) );
            }

        }

        // for (p,d) in &self.paths_pair {
        //     println!("{:?}, {}", p, d.0);
        // }
    }

    fn get_keys(&mut self) -> i64 {
        let mut heap: BinaryHeap<State> = BinaryHeap::new();

        let start_state = State { distance: 0, key: ' ', has_keys: vec![]};
        heap.push(start_state);

        let mut visited: Vec<(char,Vec<char>)> = vec![];
        let mut already_in_heap: Vec<(char,Vec<char>)> = vec![];
        
        while let Some(s) = heap.pop() {
            let mut current_keys: Vec<char> = vec![s.key];
            for k in &s.has_keys {
                current_keys.push(*k);
            }
            current_keys.sort();

            if current_keys.len() == (self.keys.len()+1) {
                // println!("Found shortest path to all keys => {} ", s.distance);
                return s.distance;
            } else {
                // s.print();
                visited.push( (s.key, s.has_keys.clone()) );
                let candidates = self.paths[&s.key].clone();

                for candidate in candidates {
                    let mut keys = s.has_keys.clone();
                    keys.push(s.key);
                    keys.sort();
                    // only consider this candidate if the target is not in de keylist already
                    if !keys.contains(&candidate.0) {

                        // check if for this path the correct keys are already in possesion
                        let mut valid = true;
                        for door in &candidate.3 {
                            if !keys.contains(door) {
                                valid = false;
                            }
                        }
                        // if valid consider this candidate
                        if valid {
                            // add the found keys to the list
                            for k in &candidate.2 {
                                if !keys.contains(k) {
                                    keys.push(*k);
                                }
                            }
                            
                            // sort the key list
                            keys.sort();

                            if !already_in_heap.contains( &(candidate.0, keys.clone()) ) {
                                // if this candidate has not been visited with the correct keys consider this
                                if !visited.contains(&(candidate.0, keys.clone())) {

                                    let new_state = State { distance: s.distance + candidate.1, key: candidate.0, has_keys: keys.clone() };
                                    // print!("Adding the following state ");
                                    // new_state.print();
                                    heap.push(new_state);
                                    already_in_heap.push( (candidate.0, keys.clone()) );
                                }
                            }
                        }
                    }
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

    // let input_file = "./input/day18_19_test.txt";
    let input_file = "./input/day18_19_real.txt";
    let input = tools::get_input(String::from(input_file));

    let mut cave = Cave::new();
    cave.scan(&input);

    let after0 = Instant::now();
    println!("Init in {:?}", after0.duration_since(start0));

    let start1 = Instant::now();

    cave.find_all_pairs();
    let res1 = cave.get_keys();

    let after1 = Instant::now();
    println!("Part 1: {}, in {:?}", res1, after1.duration_since(start1));

    let start2 = Instant::now();

    let after2 = Instant::now();
    println!("Part 2: {}, in {:?}", 0, after2.duration_since(start2));
}

use std::time::{Instant};
use std::cmp::Ordering;
use super::tools;
use std::collections::BTreeMap;
use std::collections::BinaryHeap;

#[derive(Clone, Eq, PartialEq)]
struct State {
    distance: i64,
    pos: (i64,i64),
}

impl State {
    #[allow(dead_code)]
    fn print(&self) {
        println!("State ( distance: {}, pos: {:?}", self.distance, self.pos);
    }
}

impl Ord for State {
    fn cmp(&self, other: &State) -> Ordering {
        other.distance.cmp(&self.distance).then(
            other.pos.0.cmp(&self.pos.0).then(
                other.pos.1.cmp(&self.pos.1)
            )
        )
    }
}

impl PartialOrd for State {
    fn partial_cmp(&self, other: &State) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

struct Maze {
    size: (i64,i64),
    min_bound: (i64,i64),
    max_bound: (i64,i64),
    field: BTreeMap<(i64,i64), i64>,
    portal: BTreeMap<i64,(i64,i64)>,
    start_portal: i64,
    target_portal: i64,
    found_character: BTreeMap<(i64,i64), char>,
}

impl Maze {
    fn new() -> Maze {
        Maze {
            size: (0,0),
            min_bound: (0,0),
            max_bound: (0,0),
            field: BTreeMap::new(),
            portal: BTreeMap::new(),
            start_portal: -1,
            target_portal: -1,
            found_character: BTreeMap::new(),
        }
    }

    fn add_row(&mut self, line: &String) {
        let mut pos = (0, self.size.1);
        line.chars().for_each(|c| {
            match c {
                '#' => {
                    self.field.insert(pos, 1);
                }
                '.' => {
                    self.field.insert(pos, 0);
                }
                _ => {
                    if c != ' ' {
                        self.found_character.insert(pos, c);
                    }
                }
            }
            pos.0 += 1;
        });
        self.size.0 = std::cmp::max(self.size.0, pos.0);
        self.size.1 += 1;
    }

    fn find_map_position(&self, pos1: (i64,i64), pos2: (i64,i64)) -> (i64,i64) {
        let delta = (pos2.0 - pos1.0, pos2.1 - pos1.1);
        let t1 = (pos1.0 - delta.0, pos1.1 - delta.1);
        let t2 = (pos2.0 + delta.0, pos2.1 + delta.1);
        if self.field.contains_key(&t1) {
            t1
        } else {
            t2
        }
    }

    // fn insert_portal_into_map(&mut self, id: i64) {

    // }

    fn print(&self) {
        println!("------------------------------------------------------------------");
        for y in self.min_bound.1..=self.max_bound.1 {
            for x in self.min_bound.0..=self.max_bound.0 {
                let c = ' ';
                print!("{}",
                    if self.field.contains_key(&(x,y)) && self.field[&(x,y)] == 1 {
                        '#'
                    } else if self.field.contains_key(&(x,y)) && self.field[&(x,y)] == 0 {
                        ' '
                    } else if self.field.contains_key(&(x,y)) {
                        c
                    } else {
                        ' '
                    }
                )
            }
            println!("");
        }
        println!("------------------------------------------------------------------");
        
        for (k,p) in &self.portal {
            println!("{} -> {:?}", k, p);
        }
        println!("------------------------------------------------------------------");
        println!("start portal {}", self.start_portal);
        println!("target portal {}", self.target_portal);
        println!("------------------------------------------------------------------");
    }

    fn connect_portals(&mut self) {
        let mut id_gen = 1;
        let mut processed: Vec<(i64,i64)> = vec![];
        let mut found_portals: BTreeMap<String, i64> = BTreeMap::new();
        let mut pos_portals: BTreeMap<String, (i64,i64)> = BTreeMap::new();

        for (k, v) in &self.found_character {
            if !processed.contains(&k) {
                let directions = vec![(-1,0), (1,0), (0,-1), (0,1)];
                for d in 0..directions.len() {
                    let next_pos = (k.0 + directions[d].0, k.1 + directions[d].1);
                    if self.found_character.contains_key(&next_pos) {
                        let o = self.found_character[&next_pos];
                        let portal_id: String = vec![v,&o].into_iter().collect();

                        processed.push(*k);
                        processed.push(next_pos);


                        if found_portals.contains_key(&portal_id) {
                            let position = self.find_map_position(*k,next_pos);
                            let portal_id_num = id_gen;
                            let other_position = pos_portals[&portal_id];
                            let other_portal_id_num = found_portals[&portal_id];

                            self.field.insert(position,-other_portal_id_num);
                            self.field.insert(other_position,-portal_id_num);
                            
                            self.portal.insert(portal_id_num, position);
                            self.portal.insert(other_portal_id_num, other_position);

                        } else {
                            let position = self.find_map_position(*k,next_pos);

                            if portal_id == "AA" {
                                self.start_portal = id_gen;
                                self.field.insert(position,-id_gen);
                                self.portal.insert(id_gen, position);
                            } else if portal_id == "ZZ" {
                                self.target_portal = id_gen;
                                self.field.insert(position,-id_gen);
                                self.portal.insert(id_gen, position);
                            } else {
                                pos_portals.insert(portal_id.clone(), position);
                                found_portals.insert(portal_id.clone(), id_gen);
                            }
                        }
                        id_gen += 1;
                    }
                }
            }
        }

        let mut minx = std::i64::MAX;
        let mut maxx = std::i64::MIN;
        let mut miny = std::i64::MAX;
        let mut maxy = std::i64::MIN;
        for (pos,v) in &self.field {
            minx = std::cmp::min(minx, pos.0);
            maxx = std::cmp::max(maxx, pos.0);
            miny = std::cmp::min(miny, pos.1);
            maxy = std::cmp::max(maxy, pos.1);
        }

        self.min_bound = (minx, miny);
        self.max_bound = (maxx, maxy);

        // for (k,fp) in found_portals {
        //     println!("{} -> {}", k, fp);
        // }
    }

    fn check_bounds(&self, p: &(i64,i64)) -> bool {
        (p.0 <= self.max_bound.0) &&
        (p.1 <= self.max_bound.1) &&
        (p.0 >= self.min_bound.0) &&
        (p.1 >= self.min_bound.1) &&
        self.field.contains_key(p) 
    }

    fn search(&mut self) -> i64 {
        let mut heap = BinaryHeap::new();
        let mut visited: Vec<(i64,i64)> = vec![];

        let start_pos = self.portal[&self.start_portal];
        let s = State { distance: 0, pos: start_pos };

        heap.push(s);

        while let Some(s) = heap.pop() {
            if s.pos == self.portal[&self.target_portal] {
                return s.distance-1;
            }

            if !visited.contains(&s.pos) {
                visited.push(s.pos);

                let directions = vec![(-1,0), (0,1), (1,0), (0,-1)];
                for d in 0..directions.len() {
                    let mut new_pos = (s.pos.0 + directions[d].0, s.pos.1 + directions[d].1);

                    if !visited.contains(&new_pos) {

                        if self.check_bounds(&new_pos) {

                            let field_value = self.field[&new_pos];

                            let mut step = 1;
                            if field_value != 1 {
                                if field_value < 0 {
                                    new_pos = self.portal[&field_value.abs()];
                                    step += 1;
                                }
                                let new_s = State { distance: s.distance+step, pos: new_pos };
                                heap.push(new_s);
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
    println!("Day 20 of 2019");

    let start0 = Instant::now();

    // let input_file = "./input/day20_19_test.txt";
    let input_file = "./input/day20_19_real.txt";
    let input = tools::get_input(String::from(input_file));

    let mut maze1 = Maze::new();
    input.iter().for_each(|line| {
        maze1.add_row(line);
    });

    maze1.connect_portals();
    // maze1.print();

    let after0 = Instant::now();
    println!("Init in {:?}", after0.duration_since(start0));

    let start1 = Instant::now();

    let res1 = maze1.search();

    let after1 = Instant::now();
    println!("Part 1: {}, in {:?}", res1, after1.duration_since(start1));

    let start2 = Instant::now();

    let after2 = Instant::now();
    println!("Part 2: {}, in {:?}", 0, after2.duration_since(start2));
}

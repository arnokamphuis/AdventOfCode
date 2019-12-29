use std::time::Instant;
use super::tools;
use std::collections::BTreeMap;
use std::collections::BinaryHeap;
use std::cmp::Ordering;

#[derive(Clone)]
struct Maze {
    size: (i64,i64),
    map: BTreeMap<(i64,i64),bool>,
    locations: BTreeMap<u8, (i64,i64)>,
    distances: BTreeMap<(u8,u8), u32>,
}

impl Maze {
    fn new() -> Maze {
        Maze {
            size: (0,0),
            map: BTreeMap::new(),
            locations: BTreeMap::new(),
            distances: BTreeMap::new(),
        }
    }

    fn add_line(&mut self, line: &String) {
        let y = self.size.1;
        let mut x = 0;
        line.chars().for_each(|c| {
            match c {
                '#' => { self.map.insert((x,y),true); }
                '.' => { self.map.insert((x,y),false); }
                _ => { self.map.insert((x,y),false); 
                    self.locations.insert(c.to_digit(10).unwrap() as u8, (x,y));}
            }
            x += 1;
        });
        
        self.size.0 = std::cmp::max(self.size.0, x);
        self.size.1 += 1;
    }

    fn print(&self) {
        for y in 0..self.size.1 {
            for x in 0..self.size.0 {
                print!("{}", 
                    if self.map[&(x,y)] { '#' } else { '.' }
                );
            }
            println!("");
        }
    }

    fn shortest_visit_all(&self, from: u8, part: usize) -> u32 {
        let mut heap: BinaryHeap<Node> = BinaryHeap::new();
        let mut visited: Vec<(u8, Vec<u8>)> = vec![];

        let node = Node{ distance: 0, item: from, done: vec![] };
        heap.push(node);

        loop {
            if let Some(n) = heap.pop() {
                // if part == 2 {
                //     print!("node: {}, distance: {} ", n.item, n.distance);
                //     for d in &n.done { print!("{}, ", d); }
                //     println!("");
                // }

                if part == 1 {
                    if n.done.len() == self.locations.len()-1 {
                        return n.distance;
                    }
                } else {
                    if n.item == from && n.done.len() == self.locations.len() {
                        return n.distance;
                    }
                }

                if !visited.contains(&(n.item, n.done.clone())) {
                    visited.push((n.item,n.done.clone()));

                    for (k,_) in &self.locations {
                        // test code
                        // if n.done.contains(k) && part==2 {
                        //     println!("part: {}, done.len: {}, loc.len: {}, k: {}", part, n.done.len(), self.locations.len(), *k);
                        // }

                        // real code
                        if (!n.done.contains(k) || ( part == 2 && n.done.len()+1==self.locations.len() && *k==from ) ) 
                            && n.item != *k {
                                let mut new_done = n.done.clone();
                                new_done.push(n.item);
                                new_done.sort();

                                let new_node = Node {
                                    distance: n.distance + self.distances[&(n.item, *k)],
                                    item: *k,
                                    done: new_done.clone(),
                                };

                                heap.push(new_node);
                        }
                    }

                }
            } else {
                return 0;
            }
        }
    }

    fn find_all_shortest_paths(&mut self) {
        for (k1,l1) in &self.locations {
            for (k2,l2) in &self.locations {
                if l1 != l2 {
                    let d = self.length_shortest(*l1,*l2);
                    self.distances.insert((*k1,*k2), d);
                    self.distances.insert((*k2,*k1), d);
                }
            }
        }
        // for d in &self.distances {
        //     println!("{} -> {} = {}", (d.0).0, (d.0).1, d.1);
        // }
    }

    fn length_shortest(&self, from: (i64,i64), to: (i64,i64)) -> u32 {
        let mut heap: BinaryHeap<State> = BinaryHeap::new();
        let mut visited: Vec<(i64,i64)> = vec![];
        let state = State{ distance: 0, position: from };
        heap.push(state);

        loop {
            if let Some(s) = heap.pop() {
                if s.position == to {
                    return s.distance;
                }
                if !visited.contains(&s.position) {
                    visited.push(s.position);

                    let directions = [(-1,0), (0,-1), (1,0), (0,1)];
                    for d in 0..4 {
                        let new_pos = (s.position.0 + directions[d].0, s.position.1 + directions[d].1);
                        if !self.map[&new_pos] {
                            heap.push(State{
                                distance: s.distance+1,
                                position: new_pos,
                            });
                        }
                    }
                }
            } else {
                return 0;
            }
        }
    }
}


#[derive(Clone, Eq, PartialEq)]
struct State {
    distance: u32,
    position: (i64,i64),
}

impl Ord for State {
    fn cmp(&self, other: &Self) -> Ordering {
        other.distance.cmp(&self.distance)
    }
}

impl PartialOrd for State {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

#[derive(Clone, Eq, PartialEq)]
struct Node {
    distance: u32,
    item: u8,
    done: Vec<u8>,
}

impl Ord for Node {
    fn cmp(&self, other: &Self) -> Ordering {
        other.distance.cmp(&self.distance).then(
            self.done.len().cmp(&other.done.len())
        )
    }
}

impl PartialOrd for Node {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}


#[allow(dead_code)]
pub fn run() {
    println!("Day 24 of 2016");

    let start0 = Instant::now();
    // let input_file = "./input/day24_16_test.txt";
    let input_file = "./input/day24_16_real.txt";
    let input = tools::get_input(String::from(input_file));

    let mut maze1 = Maze::new();
    input.iter().for_each(|l| {maze1.add_line(l);});
    // maze1.print();

    let after0 = Instant::now();
    println!(
        "Init in {:?}",
        after0.duration_since(start0)
    );

    let start1 = Instant::now();    

    maze1.find_all_shortest_paths();
    let res1 = maze1.shortest_visit_all(0,1);

    let after1 = Instant::now();
    println!(
        "Part 1: {}, in {:?}",
        res1,
        after1.duration_since(start1)
    );

    let start2 = Instant::now();

    let res2 = maze1.shortest_visit_all(0,2);

    let after2 = Instant::now();
    println!(
        "Part 2: {}, in {:?}",
        res2,
        after2.duration_since(start2)
    );
}

use std::collections::BTreeMap;
use std::collections::BinaryHeap;
use std::cmp::Ordering;

#[derive(Clone)]
pub struct Maze {
    size: (i64,i64),
    map: BTreeMap<(i64,i64),i64>,
    prev: BTreeMap<(i64,i64), ((i64,i64), i64)>,
}

impl Maze {
    pub fn new() -> Maze {
        Maze {
            size: (0,0),
            map: BTreeMap::new(),
            prev: BTreeMap::new(),
        }
    }

    pub fn add_line(&mut self, line: &String) {
        let y = self.size.1;
        let mut x = 0;
        line.chars().for_each(|c| { 
            self.map.insert((x,y),c.to_digit(10).unwrap() as i64); 
            x += 1;
        });
        
        self.size.0 = self.size.0.max(x);
        self.size.1 += 1;
    }

    #[allow(dead_code)]
    pub fn print(&self) {
        for y in 0..self.size.1 {
            for x in 0..self.size.0 {
                print!("{}",self.map[&(x,y)] ); 
            }
            println!("");
        }
    }
    #[allow(dead_code)]
    pub fn get_size(&self) -> (i64,i64) {
        self.size
    }

    #[allow(dead_code)]
    pub fn grow(&mut self, factor: i64) {
        let original_map = self.map.clone();
        let mut new_map: BTreeMap<(i64,i64),i64> = BTreeMap::new(); 

        for x_factor in 0..factor {
            for y_factor in 0..factor {
                for mp in &original_map {
                    let pos = mp.0;
                    let value = mp.1;
                    let mut new_value = value + x_factor + y_factor;
                    while new_value > 9 {
                        new_value -= 9;
                    }
                    let new_pos = (pos.0 + x_factor * self.size.0, pos.1 + y_factor * self.size.1 );
                    new_map.insert(new_pos, new_value);
                }
            }
        }
        self.size = (self.size.0 * factor, self.size.1 * factor);
        self.map = new_map;
    }

    pub fn length_shortest(&mut self, from: (i64,i64), to: (i64,i64)) -> i64 {
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
                        if let Some(new_value) = self.map.get(&new_pos) {
                            let new_distance = s.distance + new_value;
                            if !self.prev.contains_key(&new_pos) || new_distance < self.prev.get(&new_pos).unwrap().1 {
                                *(self.prev.entry(new_pos).or_insert(((0,0),0))) = (s.position, new_distance);
                                heap.push(State{
                                    distance: new_distance,
                                    position: new_pos,
                                });
                            }
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
    distance: i64,
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

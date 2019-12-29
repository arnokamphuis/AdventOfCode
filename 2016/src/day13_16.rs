use std::time::Instant;
use std::collections::HashMap;
extern crate queues;
use queues::*;


#[derive(Clone, Hash, Eq, PartialEq, Debug)]
struct Position {
    x: u64, y: u64
}

struct Search {
    fav_number: u64,
    found_distance: u64,
    visited: HashMap<Position, u64>,
    q: Queue<Position>,
}

impl Search {
    fn new() -> Search {
        Search {
            fav_number: 1352,
            found_distance: 0,
            visited: HashMap::new(),
            q: queue![]
        }
    }

    fn open(&self, x: u64, y: u64, fav: u64) -> bool {
        let ones : Vec<bool> = format!("{:b}", x*x + 3*x + 2*x*y + y + y*y + fav).chars().map(|s| s == '1').filter(|b| *b).collect();
        (ones.len() % 2) == 0
    }
    
    fn add_node(&mut self, newpos: &Position, newdist: u64) {
        if self.open(newpos.x, newpos.y, self.fav_number) {
            if self.visited.contains_key(&newpos) {
                if newdist < self.visited[&newpos]  {
                    if let Some(x) = self.visited.get_mut(&newpos) {
                        *x = newdist;
                    }
                }  
            } else {
                self.visited.insert(newpos.clone(), newdist);
                self.q.add(newpos.clone()).ok();
            }
        }
    }

    fn run_part(&mut self, part: usize) -> u64 {
        let startpos = Position{ x:1, y:1 };
        self.q.add( startpos.clone() ).ok();
        self.visited.insert( startpos.clone(), 0);
        while self.q.size() != 0 {
            let top = self.q.remove().unwrap();
            let dist = self.visited[&top];
            
            if part==1 || (part==2 && dist < 50) {
                if part == 1 {
                    if top == (Position{x:31,y:39}) {
                        self.found_distance = dist;
                        break;
                    }
                }
    
                let mut newpos: Position;
                let newdist = dist+1;
                // above
                if top.x >= 1 {
                    newpos = Position{ x: top.x-1, y: top.y };
                    self.add_node(&newpos, newdist);
                }

                // below
                {
                    newpos = Position{ x: top.x+1, y: top.y };
                    self.add_node(&newpos, newdist);
                }
                
                // left
                if top.y >= 1 {
                    newpos = Position{ x: top.x, y: top.y-1 };
                    self.add_node(&newpos, newdist);
                }
                
                // right
                {
                    newpos = Position{ x: top.x, y: top.y+1 };
                    self.add_node(&newpos, newdist);
                }
            }
        }
        match part {
            1 => { self.found_distance as u64 }
            2 => { self.visited.len() as u64 }
            _ => { 0 }
        }
    }    
}


#[allow(dead_code)]
pub fn run() {
    println!("Day 13 of 2016");

    let start1 = Instant::now();

    let mut part1 = Search::new();
    let res_part1 = part1.run_part(1);

    let after1 = Instant::now();
    println!(
        "Part 1: {}, in {:?}",
        res_part1,
        after1.duration_since(start1)
    );

    let start2 = Instant::now();

    let mut part2 = Search::new();
    let res_part2 = part2.run_part(2);

    let after2 = Instant::now();
    println!(
        "Part 2: {}, in {:?}",
        res_part2,
        after2.duration_since(start2)
    );
}

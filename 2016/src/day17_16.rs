use std::time::Instant;
use md5;
use queues::*;


#[derive(Eq, PartialEq, Clone, Hash)]
struct Position {
    x: usize,
    y: usize,
    path: String
}

struct Search {
    start: String,
    found_path: String,
    q: Queue<Position>,
}

impl Search {

    fn new(s: &String) -> Search {
        Search {
            start: s.clone(),
            found_path: String::from(""),
            q: queue![],
        }
    }

    fn is_open(&self, path: &String) -> Vec<bool> {
        let mut res : Vec<bool> = vec![false; 4];

        let hash = format!("{:x}", md5::compute(format!("{}{}", self.start, path).to_string()));
        hash[0..4].chars().enumerate().for_each(|(i, c)| {
            match c {
                'b' | 'c' | 'd' | 'e' | 'f' => { res[i] = true; } 
                _ => { res[i] = false; }
            }
        });

        res
    }

    fn add_node(&mut self, newpos: &Position) {
        self.q.add(newpos.clone()).ok();
    }

    fn search(&mut self, part: usize) -> String {
        let mut paths: Vec<String> = vec![];
        let startpos = Position{ x:0, y:0, path: String::from("") };

        self.q.add( startpos.clone() ).ok();

        while let Ok(top) = self.q.remove() {

            if top.x == 3 && top.y == 3 {
                self.found_path = top.path.clone();
                if part == 1 {
                    break;
                }
            } else {
    
                let mut newpos: Position;
                let mut newpath = top.path.clone();

                let open = self.is_open(&newpath);

                // above
                if top.y >= 1 && open[0] {
                    newpath.push('U');
                    newpos = Position{ x: top.x, y: top.y-1, path: newpath.clone() };
                    self.add_node(&newpos);
                }

                newpath = top.path.clone();
                // below
                if top.y <= 2 && open[1] {
                    newpath.push('D');
                    newpos = Position{ x: top.x, y: top.y+1, path: newpath.clone() };
                    self.add_node(&newpos);
                }
                
                newpath = top.path.clone();
                // left
                if top.x >= 1 && open[2] {
                    newpath.push('L');
                    newpos = Position{ x: top.x-1, y: top.y, path: newpath.clone() };
                    self.add_node(&newpos);
                }
                
                newpath = top.path.clone();
                // right
                if top.x <= 2 && open[3] {
                    newpath.push('R');
                    newpos = Position{ x: top.x+1, y: top.y, path: newpath.clone() };
                    self.add_node(&newpos);
                }
            }
        }
        match part {
            1 | 2=> { self.found_path.clone() }
            _ => {
                println!("done {}", paths.len());
                (&mut paths).sort_by(|a, b| a.len().cmp(&b.len()));
                (&paths[0]).clone()
            }
        }
    }
}

#[allow(dead_code)]
pub fn run() {
    println!("Day 17 of 2016");

    let start0 = Instant::now();

    let start = String::from("edjrjqaa"); // real input
    // let start = String::from("kglvqrro");

    let after0 = Instant::now();
    println!(
        "Init in {:?}",
        after0.duration_since(start0)
    );

    let start1 = Instant::now();

    let mut search1 = Search::new(&start);
    let res1 = search1.search(1);

    let after1 = Instant::now();
    println!(
        "Part 1: {}, in {:?}",
        res1,
        after1.duration_since(start1)
    );

    let start2 = Instant::now();

    let mut search2 = Search::new(&start);
    let res2 = search2.search(2);

    let after2 = Instant::now();
    println!(
        "Part 2: {}, in {:?}",
        res2.len(),
        after2.duration_since(start2)
    );
}

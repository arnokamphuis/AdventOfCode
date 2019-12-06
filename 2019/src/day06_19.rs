use std::time::{Instant};
use super::tools;
use std::collections::HashMap;

struct Node {
  name: String,
  children: Vec<String>,
  depth: u64,
}

impl Node {
    fn new(n: &String) -> Node {
        Node { name : n.clone(), children: vec![], depth: 0 }
    }

    fn add_child(&mut self, n: &String) {
        self.children.push(n.clone());
    }

    fn get_chidren(&self) -> Vec<String> {
        let mut res: Vec<String> = vec![];
        for child in &self.children {
            res.push(child.to_string());
        }
        res.clone()
    }

    // fn calculate_orbits(&self, d: u64) -> u64 {
    //     self.depth = d;
    //     for c in self.children {
    //         d += c.calculate_orbits(d+1);
    //     }
    //     d
    // }
}

struct Tree {
    nodes: HashMap<String, Node>,
}

impl Tree {
    fn new() -> Tree {
        Tree{ nodes: HashMap::new() }
    }

    fn add_orbit(&mut self, a: &String, b: &String) {
        if !self.nodes.contains_key(b) {
            self.nodes.insert(b.to_string(),Node::new(b));            
        }
        if !self.nodes.contains_key(a) {
            self.nodes.insert(a.to_string(),Node::new(a));
        }
        if let Some(ma) = self.nodes.get_mut(a) {
            (*ma).add_child(b);
            // println!("adding {} to parent {}", b, a);
        }
}

    fn calculate_orbits(&mut self, n: String, d: u64) -> u64 {
        let mut total = d;
        for c in self.nodes[&n].get_chidren() {
            total += self.calculate_orbits(c, d+1);
        }
        total
    }
}

pub fn run() {
    println!("Day 6 of 2019");

    // let input_file = "./input/day06_19_test.txt";
    let input_file = "./input/day06_19_real.txt";
    let input = tools::get_input(String::from(input_file));

    let mut orbits = Tree::new();

    for line in input {
        let mut iter = line.split(")");
        let a: String = iter.next().unwrap().to_string();
        let b: String = iter.next().unwrap().to_string();
        orbits.add_orbit(&a,&b);
    }

    let o = orbits.calculate_orbits("COM".to_string(), 0);

    let start1 = Instant::now();

    let after1 = Instant::now();
    println!("Part 1: {}, in {:?}", o, after1.duration_since(start1));

    let start2 = Instant::now();

    let after2 = Instant::now();
    println!("Part 2: {}, in {:?}", 0, after2.duration_since(start2));
}

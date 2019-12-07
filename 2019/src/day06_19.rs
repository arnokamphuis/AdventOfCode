use std::time::{Instant};
use super::tools;
use std::collections::HashMap;
use std::collections::HashSet;

struct Node {
  parent: String,
  children: Vec<String>,
}

impl Node {
    fn new() -> Node {
        Node { parent: String::from(""), children: vec![] }
    }

    fn add_child(&mut self, n: &String) {
        self.children.push(n.clone());
    }

    fn set_parent(&mut self, p: &String) {
        self.parent = p.clone();
    }

    fn get_parent(&self) -> String {
        self.parent.clone()
    }

    fn get_chidren(&self) -> Vec<String> {
        let mut res: Vec<String> = vec![];
        for child in &self.children {
            res.push(child.to_string());
        }
        res.clone()
    }
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
            self.nodes.insert(b.to_string(),Node::new());            
        }
        if !self.nodes.contains_key(a) {
            self.nodes.insert(a.to_string(),Node::new());
        }
        if let Some(ma) = self.nodes.get_mut(a) {
            (*ma).add_child(b);
        }
        if let Some(mb) = self.nodes.get_mut(b) {
            (*mb).set_parent(a);
        }    
    }

    fn calculate_orbits(&mut self, n: String, d: u64) -> u64 {
        let mut total = d;
        for c in self.nodes[&n].get_chidren() {
            total += self.calculate_orbits(c, d+1);
        }
        total
    }

    fn get_parent_set(&self, n: String) -> HashSet<String> {
        let mut set: HashSet<String> = HashSet::new();
        set.insert(n.clone());
        let parent = self.nodes[&n].get_parent();
        if parent != "" {
            let parent_set = self.get_parent_set(parent.clone());
            parent_set.iter().for_each(|p| {
                set.insert(p.to_string());});
        }
        set.clone()
    }
}

#[allow(dead_code)]
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

    let set_san = orbits.get_parent_set("SAN".to_string());
    let set_you = orbits.get_parent_set("YOU".to_string());

    let mut path: Vec<String> = vec![];
    let diff1 = set_san.difference(&set_you);
    for d in diff1 {
        path.push(d.clone());
    }
    let diff2 = set_you.difference(&set_san);
    for d in diff2 {
        path.push(d.clone());
    }

    let after2 = Instant::now();
    println!("Part 2: {}, in {:?}", path.len()-2, after2.duration_since(start2));
}

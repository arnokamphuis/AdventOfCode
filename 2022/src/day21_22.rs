use super::tools;
use std::time::Instant;
use std::collections::HashMap;

type ChildNode<T> = Option<Box<BTNode<T>>>;

enum Op<T> {
    Add, Sub, Div, Mul, Id(T)
}

struct BTNode<T> {
    name: Vec<char>,
    left: ChildNode<T>,
    right: ChildNode<T>,
    op: Op<T>
}

impl BTNode<i64> {
    pub fn new(name: Vec<char>, op: Op<i64>, l: BTNode<i64>, r: BTNode<i64>) -> Self {
        BTNode::<i64> {  
            name: name.clone(),
            op: op, 
            left: Some(Box::new(l)), 
            right: Some(Box::new(r))
        }  
    }
}

fn AddNode(name: Vec<char>, l: BTNode<i64>, r: BTNode<i64>) -> BTNode<i64> {
    BTNode::new(name.clone(), Op::Add, l, r)
}

fn SubNode(name: Vec<char>, l: BTNode<i64>, r: BTNode<i64>) -> BTNode<i64> {
    BTNode::new(name.clone(), Op::Sub, l, r)
}

fn DivNode(name: Vec<char>, l: BTNode<i64>, r: BTNode<i64>) -> BTNode<i64> {
    BTNode::new(name.clone(), Op::Div, l, r)
}

fn MulNode(name: Vec<char>, l: BTNode<i64>, r: BTNode<i64>) -> BTNode<i64> {
    BTNode::new(name.clone(), Op::Mul, l, r)
}

fn IdNode(name: Vec<char>, value: i64) -> BTNode<i64> {
    BTNode {
        name: name.clone(),
        op: Op::Id(value), 
        left: None, 
        right: None
    }
}

struct BinaryTree<T> {
    head: Option<BTNode<T>>
}

impl BinaryTree<i64> {
    pub fn new(head: BTNode<i64>) -> Self {
        BinaryTree::<i64> { head: Some(head) }
    }
    
    pub fn collapse(node: &Box<BTNode<i64>>) -> i64 {
        let mut r: Option<i64> = None;
        let mut l: Option<i64> = None;
        
        if let Some(left) = &node.left {
            l = Some(BinaryTree::collapse(left));
        }
        
        if let Some(right) = &node.right {
            r = Some(BinaryTree::collapse(right));
        }
        
        let r = if let Some(x) = r { x } else { 0 };
        let l = if let Some(x) = l { x } else { 0 };  
        
        match node.op {
            Op::Add => { l + r }
            Op::Sub => { l - r }
            Op::Mul => { l * r }
            Op::Div => {
                if r == 0 { 
                    panic!("attempted divide-by-zero operation.") 
                    
                }
                l / r
            }
            Op::Id(x) => x 
        }
    }

    pub fn find_needed_value<'a>(node: &'a Box<BTNode<i64>>, value: i64) -> i64 {
        if node.name.eq(&vec!['h','u','m','n']) { return value; }

        let left = &Box::new(node.left.as_ref().expect("No left"));
        let right = &Box::new(node.right.as_ref().expect("No right"));
    
        if let Some(human) = BinaryTree::find_child(left, &vec!['h','u','m','n']) {
            let v = BinaryTree::collapse(right);
            let next_value = match node.op {
                Op::Add => { value - v }
                Op::Sub => { value + v }
                Op::Mul => { value / v }
                Op::Div => { value * v }
                Op::Id(x) => panic!() 
            };
            return BinaryTree::find_needed_value(left, next_value);
        }

        if let Some(human_right) = BinaryTree::find_child(right, &vec!['h','u','m','n']) {
            let v = BinaryTree::collapse(left);
            let next_value = match node.op {
                Op::Add => { value - v }
                Op::Sub => { v - value }
                Op::Mul => { value / v }
                Op::Div => { v / value }
                Op::Id(x) => panic!() 
            };
            return BinaryTree::find_needed_value(right, next_value);
        }
        panic!("Should not be here");
        return 0;
    }

    pub fn find_child<'a>(node: &'a Box<BTNode<i64>>, n: &Vec<char>) -> Option<&'a Box<BTNode<i64>>> {
        if node.name.eq(n) { 
            return Some(node);
        }
        if let Some(left) = &node.left {
            if let Some(child) = BinaryTree::find_child(left, n) {
                return Some(child);
            }
        }
        
        if let Some(right) = &node.right {
            if let Some(child) = BinaryTree::find_child(right, n) {
                return Some(child);
            }
        }

        None
    }

    pub fn is_child(root: &Box<BTNode<i64>>, p: &Vec<char>, c: &Vec<char>) -> bool {
        if let Some(parent) = BinaryTree::find_child(root, p) {
            if let Some(child) = BinaryTree::find_child(parent, c) {
                return true;
            }
        }
        false
    }
}





#[derive(Clone, Debug)]
enum Operation {
    Plus,
    Times,
    Divide,
    Minus,
}

#[derive(Clone, Debug)]
struct MonkeyBusiness {
    to_be_done: HashMap<(Vec<char>,Vec<char>), (Vec<char>,Operation)>,
    yelled: HashMap<Vec<char>, i64>,
}

impl MonkeyBusiness {
    fn run(&mut self) -> i64 {

        let mut last_yelled = 0;

        while self.to_be_done.len() > 0 {
            let tbd = self.to_be_done.clone();
            let doable = tbd.iter().filter(|((n1,n2),(n,op))| {
                    self.yelled.contains_key(n1) && self.yelled.contains_key(n2)                })
                .collect::<Vec<_>>();
        
            doable.iter().for_each(|((n1,n2),(n,op))| {
                let v1 = self.yelled[n1];
                let v2 = self.yelled[n2];
                let res = match op {
                    Operation::Minus => v1 - v2,
                    Operation::Plus => v1 + v2,
                    Operation::Times => v1 * v2,
                    Operation::Divide => v1 / v2,
                };
                last_yelled = res;
                self.yelled.insert(n.clone(), res);
                self.to_be_done.remove(&(n1.clone(),n2.clone()));
            });
        }
        last_yelled
    }

    fn create_node(&self, name: &Vec<char>) -> BTNode<i64> {
        let item = self.to_be_done.iter().filter(|(_,(n,_))| { n.eq(name)}).collect::<Vec<_>>();
        if item.len() == 1 {
            match item[0].1.1 {
                Operation::Minus  => SubNode( item[0].1.0.clone(), self.create_node(&item[0].0.0), self.create_node(&item[0].0.1) ),
                Operation::Divide => DivNode( item[0].1.0.clone(), self.create_node(&item[0].0.0), self.create_node(&item[0].0.1) ),
                Operation::Plus   => AddNode( item[0].1.0.clone(), self.create_node(&item[0].0.0), self.create_node(&item[0].0.1) ),
                Operation::Times  => MulNode( item[0].1.0.clone(), self.create_node(&item[0].0.0), self.create_node(&item[0].0.1) ),
            }
        } else {
            IdNode(name.clone(), self.yelled[name])
        }
    }

    fn create_binary_tree(&self) -> BinaryTree<i64> {
        BinaryTree::new(self.create_node(&vec!['r','o','o','t']))
    }
}

#[allow(dead_code)]
pub fn run(real: bool, print_result: bool) -> (u128, u128, u128) {
    let start0 = Instant::now();

    let input_file: &str = if !real {
        "./input/day21_22_test.txt"
    } else {
        "./input/day21_22_real.txt"
    };
    let input = tools::get_input(String::from(input_file));

    let mut mb = MonkeyBusiness {
        to_be_done: HashMap::new(),
        yelled: HashMap::new(),
    };

    input.iter().for_each(|line| {
        let words = line.split_whitespace().collect::<Vec<&str>>();
        let name = &words[0][0..4].chars().collect::<Vec<char>>();
        if words.len() > 2 {
            if let Some(c) = words[2].chars().nth(0) {
                let op = match c {
                    '*' => Operation::Times,
                    '+' => Operation::Plus,
                    '/' => Operation::Divide,
                    '-' => Operation::Minus,
                    _ => panic!()
                };
                let name1 = &words[1][0..4].chars().collect::<Vec<char>>();
                let name2 = &words[3][0..4].chars().collect::<Vec<char>>();
                mb.to_be_done.insert((name1.clone(), name2.clone()), (name.clone(), op));
            }
        } else {
            mb.yelled.insert(name.clone(), words[1].parse::<i64>().unwrap());
        }
    });

    let bt = mb.create_binary_tree();

    let after0 = Instant::now();

    let start1 = Instant::now();

    let res1 = mb.run();

    let after1 = Instant::now();
    if print_result {
        println!("Part 1: {}", res1);
    }

    let start2 = Instant::now();

    let root = &Box::new(bt.head.expect("No head initialized."));
    let left = &Box::new(root.left.as_ref().expect("No left"));
    let right = &Box::new(root.right.as_ref().expect("No right"));

    let mut res2 = 0;

    if let Some(human) = BinaryTree::find_child(left, &vec!['h','u','m','n']) {
        res2 = BinaryTree::find_needed_value(left, BinaryTree::collapse(right));
    }

    if let Some(human_right) = BinaryTree::find_child(right, &vec!['h','u','m','n']) {
        res2 = BinaryTree::find_needed_value(right, BinaryTree::collapse(left));
    }
    
    let after2 = Instant::now();
    if print_result {
        println!("Part 2: {}", res2);
    }

    (
        after0.duration_since(start0).as_nanos(),
        after1.duration_since(start1).as_nanos(),
        after2.duration_since(start2).as_nanos(),
    )
}

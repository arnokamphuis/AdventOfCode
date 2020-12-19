use super::tools;
use std::time::Instant;

fn operand(l: u128, r: u128, o: char) -> u128 {
    match o {
        '+' => l+r, '*' => l*r, _ => 0
    }
}
#[derive(Clone, Debug)]
struct Node {
    value: u128,
    operands: Vec<char>,
    children: Vec<Node>,
}

impl Node {
    pub fn new(line: &String) -> Node {
        let mut node = Node { value: 0, operands: vec![], children: vec![] };

        let mut cv: u128 = 0;
        let mut pc = 0;
        let mut middle = String::from("");
        let mut leftnode: Option<Node> = None;
        for c in line.chars() {
            if c == '(' { pc += 1; }
            if c == ')' { pc -= 1; }
            if pc == 0 {
                if let Some(v) = c.to_digit(10) {
                    cv = cv*10 + v as u128;
                } else if c == '+' || c == '*' {
                    if cv > 0 {
                        node.children.push( Node { value: cv, operands: vec![], children: vec![] });
                        cv = 0;
                    } else {
                        if let Some(n) = &leftnode {
                            node.children.push(n.clone());
                            leftnode = None;
                        }
                    }
                    node.operands.push(c);
                } else {
                    if c == ')' {
                        middle.remove(0);
                        leftnode = Some(Node::new(&middle));
                        middle = String::from("");
                    }
                }
            } else {
                middle.push(c);
            }
        }
        if cv > 0 {
            node.children.push( Node { value: cv, operands: vec![], children: vec![] });
        }

        if let Some(n) = &leftnode {
            node.children.push(n.clone());
        }
        node
    }

    pub fn eval(&self, part: usize) -> u128 {
        let res;
        if self.children.len() == 0 {
            res = self.value;
        } else {
            let mut ops = self.operands.clone();
            let mut vals = self.children.iter().map(|n| n.eval(part) ).collect::<Vec<u128>>();

            if part == 1 {
                while ops.len() > 0 {
                    vals[0] = operand(vals[0], vals[1], ops[0]);
                    ops.remove(0);
                    vals.remove(1);
                }    
            } else {
                for &op in ['+','*'].iter() {
                    while let Some(i) = ops.iter().position(|&c| c==op) {
                        vals[i] = operand(vals[i], vals[i+1], op);
                        ops.remove(i);
                        vals.remove(i+1);
                    }
                }    
            }
            res = vals[0];
        }
        res
    }
}

#[allow(dead_code)]
pub fn run(real: bool, print_result: bool) -> (u128, u128, u128) {
    let start0 = Instant::now();

    let input_file: &str = if !real {
        "./input/day18_20_test.txt"
    } else {
        "./input/day18_20_real.txt"
    };
    let input = tools::get_input(String::from(input_file));
    let nodes = input.iter().map(|line| Node::new(line) ).collect::<Vec<Node>>();

    let after0 = Instant::now();

    let start1 = Instant::now();

    let res1 = nodes.iter().map(|node| node.eval(1)).sum::<u128>();

    let after1 = Instant::now();
    if print_result {
        println!("Part 1: {}", res1);
    }

    let start2 = Instant::now();

    let res2 = nodes.iter().map(|node| node.eval(2)).sum::<u128>();

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

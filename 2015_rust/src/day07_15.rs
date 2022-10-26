use super::tools;
use std::time::Instant;

#[derive(Debug, Clone, PartialEq)]
enum Wire {
    Name(String),
    Value(u16),
}

#[derive(Debug, Clone, PartialEq)]
enum OpType {
    NONE, AND, OR, LSHIFT, RSHIFT, NOT,
}

#[derive(Debug, Clone)]
struct Operation {
    op: OpType,
    input1: Wire,
    input2: Option<Wire>,
    output_name: String,
    output_value: Option<u16>,    
}

impl Operation {
    fn done(&self) -> bool {
        self.output_value != None
    }

    fn execute(&mut self) -> Option<(String,u16)> {
        match self.op {
            OpType::NONE => {
                if let Wire::Value(i1) = self.input1 {
                    self.output_value = Some(i1);
                    return Some((self.output_name.clone(), self.output_value.unwrap()));
                }
            },
            OpType::AND => {
                if let Wire::Value(i1) = self.input1 {
                    if let Some(Wire::Value(i2)) = self.input2 {
                        self.output_value = Some(i1 & i2);
                        return Some((self.output_name.clone(), self.output_value.unwrap()));
                    }
                }
            },
            OpType::OR => {
                if let Wire::Value(i1) = self.input1 {
                    if let Some(Wire::Value(i2)) = self.input2 {
                        self.output_value = Some(i1 | i2);
                        return Some((self.output_name.clone(), self.output_value.unwrap()));
                    }
                }
            },
            OpType::LSHIFT => {
                if let Wire::Value(i1) = self.input1 {
                    if let Some(Wire::Value(i2)) = self.input2 {
                        self.output_value = Some(i1 << i2);
                        return Some((self.output_name.clone(), self.output_value.unwrap()));
                    }
                }
            },
            OpType::RSHIFT => {
                if let Wire::Value(i1) = self.input1 {
                    if let Some(Wire::Value(i2)) = self.input2 {
                        self.output_value = Some(i1 >> i2);
                        return Some((self.output_name.clone(), self.output_value.unwrap()));
                    }
                }
            },
            OpType::NOT => {
                if let Wire::Value(i1) = self.input1 {
                    self.output_value = Some(!i1);
                    return Some((self.output_name.clone(), self.output_value.unwrap()));
                }
            },
        }
        None
    }
}

#[allow(dead_code)]
pub fn run(real: bool, print_result: bool) -> (u128, u128, u128) {
    let start0 = Instant::now();

    let input_file: &str = if !real {
        "./input/day07_15_test.txt"
    } else {
        "./input/day07_15_real.txt"
    };
    let input = tools::get_input(String::from(input_file));

    let operations: Vec<Operation> = input.iter().map(|line| {
        let mut parts = line.split(" -> ");
        let left_part = parts.next().unwrap();
        let output_name = parts.next().unwrap().to_string();


        let left_parts = left_part.split_whitespace().map(|s| s.to_string()).collect::<Vec<String>>();

        if left_parts.len() == 1 { // single value assignment
            let l = left_parts[0].parse::<u16>();
            return Operation {
                op: OpType::NONE,
                input1: match l { Ok(v) => Wire::Value(v), Err(_) => Wire::Name(left_parts[0].clone()) },
                input2: None,
                output_name: output_name,
                output_value: None
            }
        } else if left_parts.len() == 2 {
            let l = left_parts[1].parse::<u16>();
            return Operation {
                op: OpType::NOT,
                input1: match l { Ok(v) => Wire::Value(v), Err(_) => Wire::Name(left_parts[1].clone()) },
                input2: None,
                output_name: output_name,
                output_value: None
            }
        } else {
            let l = left_parts[0].parse::<u16>();
            let r = left_parts[2].parse::<u16>();
            let op = match left_parts[1].as_str() {
                "AND" => OpType::AND,
                "OR" => OpType::OR,
                "LSHIFT" => OpType::LSHIFT,
                "RSHIFT" => OpType::RSHIFT,
                _ => OpType::NONE,
            };

            return Operation {
                op: op,
                input1: match l { Ok(v) => Wire::Value(v), Err(_) => Wire::Name(left_parts[0].clone()) },
                input2: Some(match r { Ok(v) => Wire::Value(v), Err(_) => Wire::Name(left_parts[2].clone()) }),
                output_name: output_name,
                output_value: None
            }
        }
    }).collect();

    let step = | orig_ops: Vec<Operation>, results: &mut Vec<(String,u16)> | -> Vec<Operation> {
        let mut ops = orig_ops.clone();
        for op in &mut ops {
            if let Some(res) = op.execute() {
                results.push(res.clone());
            }
        }
        ops.iter().filter(|op| !op.done()).map(|op| op.clone()).collect()
    };

    let send_signals = | orig_ops: Vec<Operation>, results: &Vec<(String,u16)> | -> Vec<Operation> {
        let mut ops = orig_ops.clone();
        for op in &mut ops {
            for res in results {
                if let Wire::Name(n1) = &op.input1 {
                    if n1.eq(&res.0) {
                        op.input1 = Wire::Value(res.1);
                    }
                }
                if let Some(Wire::Name(n2)) = &op.input2 {
                    if n2.eq(&res.0) {
                        op.input2 = Some(Wire::Value(res.1));
                    }
                }
            }
        }
        ops
    };

    let after0 = Instant::now();

    let start1 = Instant::now();

    let mut results: Vec<(String,u16)> = vec![];
    let mut ops = operations.clone();

    ops = step(ops, &mut results);

    let mut finished = false;
    while !finished {
        ops = send_signals(ops, &results);
        ops = step(ops, &mut results);
        finished = ops.len() == 0;
    }

    let res1 = results.iter().filter(|res| res.0.eq("a")).map(|w| w.1).collect::<Vec<u16>>()[0];

    let after1 = Instant::now();
    if print_result {
        println!("Part 1: {}", res1);
    }

    let start2 = Instant::now();

    let mut results: Vec<(String,u16)> = vec![];
    let mut ops = operations.clone();
    for op in &mut ops {
        if op.output_name.eq("b") {
            op.input1 = Wire::Value(res1);
        }
    }

    ops = step(ops, &mut results);
    let mut finished = false;
    while !finished {
        ops = send_signals(ops, &results);
        ops = step(ops, &mut results);
        finished = ops.len() == 0;
    }

    let res2 = results.iter().filter(|res| res.0.eq("a")).map(|w| w.1).collect::<Vec<u16>>()[0];

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

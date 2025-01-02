use super::tools;
use std::time::Instant;
use std::collections::{HashMap, HashSet};
use itertools::Itertools;

fn process(wires: &HashMap<String, i64>, gates: &HashMap<String, (String, String, String)>) -> HashMap<String, i64> {
    let mut output: HashMap<String, i64> = HashMap::new();
    
    let active = gates.iter().filter(|(_, (in1, _, in2))| {
        wires.contains_key(in1) && wires.contains_key(in2)
    }).collect::<HashMap<&String, &(String, String, String)>>();
    
    for (out, (in1, op, in2)) in active {
        let in1 = wires.get(in1).unwrap();
        let in2 = wires.get(in2).unwrap();
        let result = match op.as_str() {
            "AND" => in1 & in2,
            "OR " => in1 | in2,
            "XOR" => in1 ^ in2,
            _ => panic!("Unknown operator"),
        };
        output.insert(out.to_string(), result);
    }
    output
}

fn get_decimal_value(wires: HashMap<String, i64>) -> i64 {
    let mut v = 0;
    let mut bit = wires.len() as i64 - 1;
    while bit >= 0 {
        let z_str = format!("z{:02}", bit);
        let value = wires[&z_str];
        v |= value << bit;
        bit -= 1;
    }
    v
}

fn process_until_target_are_set(wires: HashMap<String, i64>, gates: &HashMap<String, (String, String, String)>, target: Vec<String>) -> HashMap<String, i64> {
    let mut target_values = HashMap::new();

    let mut updated_wires = wires.clone();

    while target_values.len() < target.len(){
        let mut output = process(&updated_wires, gates);
        for (k, v) in output.drain() {
            *updated_wires.entry(k.clone()).or_insert(0) = v;
            if target.contains(&k) {
                target_values.insert(k, v);
            }
        }
    }
    return target_values;
}

fn check_back(bit: usize, gates: &HashMap<String, (String, String, String)>) -> Result<String, String> {
    let gate_name = format!("z{:02}", bit);
    if !gates.contains_key(&gate_name) {
        return Err(gate_name);
    }
    let xor_gate = gates.get(&gate_name).unwrap();
    if xor_gate.1 != "XOR" {
        return Err(gate_name);
    }

    let in_wires: HashSet<String> = HashSet::from_iter(vec![&xor_gate.0, &xor_gate.2].iter().map(|x| x.to_string()));
    let in_gates = gates.iter().filter(|&(out, _)| in_wires.contains(&(out.clone()))).collect::<HashMap<&String, &(String, String, String)>>();

    let xor_gate_names = in_gates
        .iter()
        .filter(|(_, (_, op, _))| *op == "XOR")
        .map(|(out, _)| out.to_string())
        .collect::<Vec<String>>();

    let not_and_gate_names = in_gates
        .iter()
        .filter(|(_, (_, op, _))| *op != "AND")
        .map(|(out, _)| out.to_string())
        .collect::<Vec<String>>();

    let not_or_gate_names = in_gates
        .iter()
        .filter(|(_, (_, op, _))| *op != "OR ")
        .map(|(out, _)| out.to_string())
        .collect::<Vec<String>>();

    if xor_gate_names.len() == 0 {
        if bit == 1 {
            return Err(not_and_gate_names[0].clone());
        } else {
            return Err(not_or_gate_names[0].clone());
        }
    }

    let x_wire = format!("x{:02}", bit);
    let y_wire = format!("y{:02}", bit);

    let xor_gate_inputs = in_gates
        .iter()
        .filter(|(_, (_, op, _))| *op == "XOR")
        .map(|(_, (in1, _, in2))| (in1.to_string(), in2.to_string()))
        .collect::<HashSet<(String, String)>>();

    if xor_gate_inputs.len() != 1 {
        let incorrect_in_wires = xor_gate_inputs
            .iter()
            .filter(|(in1, in2)| !(in1 == &x_wire && in2 == &y_wire || in1 == &y_wire && in2 == &x_wire ))
            .fold(HashSet::new(), |mut acc, (in1, in2)| {
                acc.insert(in1.clone());
                acc.insert(in2.clone());
                acc
            });
        let incorrect_gate = &gates
            .iter()
            .filter(|(_, (in1, _, in2))| incorrect_in_wires.contains(in1) && incorrect_in_wires.contains(in2))
            .map(|(out, _)| out.to_string())
            .collect::<Vec<String>>()[0].clone();
        return Err(incorrect_gate.to_string());
    }

    if bit == 1 {
        let and_gate: HashSet<String> = in_gates
            .iter()
            .filter(|(_,(_,op,_))| *op == "AND")
            .fold(HashSet::new(), |mut acc, (_, gate)| {
                acc.insert(gate.0.to_string());
                acc.insert(gate.2.to_string());
                acc
            });
        
        let initial_gate = HashSet::from_iter(
            vec!["x00", "y00"]
            .iter()
            .map(|x| x.to_string())
        );

        if initial_gate != and_gate {
            return Err(not_and_gate_names[0].clone());
        }

    } else {
        let or_gate = &in_gates
            .iter()
            .filter(|(_,(_,op,_))| *op == "OR ")
            .map(|(_, gate)| *gate)
            .collect::<Vec<&(String, String, String)>>()[0];

        let mut in_or_gate: HashSet<(String, (String, String, String))> = HashSet::new();
        in_or_gate.insert((or_gate.0.clone(), gates.get(&or_gate.0).unwrap().clone()));
        in_or_gate.insert((or_gate.2.clone(), gates.get(&or_gate.2).unwrap().clone()));
        let in_or_gate_not_and = in_or_gate
            .iter()
            .filter(|(_,(_,op,_))| *op != "AND")
            .map(|(out, _)| out.to_string())
            .collect::<Vec<String>>();
        if in_or_gate_not_and.len() != 0 {
            return Err(in_or_gate_not_and[0].clone());
        }
    }

    return Ok("".to_string());
}

#[allow(dead_code)]
pub fn run(real: bool, print_result: bool) -> (u128, u128, u128) {
    let start0 = Instant::now();

    let input_file: &str = if !real {
        "./input/day24-test.txt"
    } else {
        "./input/day24-real.txt"
    };

    let input = tools::get_input(String::from(input_file));
    let empty = input.iter().enumerate().filter(|(_, x)| *x == "").map(|(i, _)| i).collect::<Vec<usize>>()[0];
    let inputs = input[..empty].iter().fold(HashMap::new(), |mut acc, x| {
        acc.insert(x[0..3].to_string(), x[5..].to_string().parse::<i64>().unwrap());
        acc
    });

    let gates = input[empty+1..]
        .iter()
        .fold(HashMap::new(), |mut acc, line| {
            let inps = line.split(" -> ").collect::<Vec<&str>>();
            let out = inps[1].to_string();
            let inps = inps[0].split(" ").collect::<Vec<&str>>();
            let mut op = inps[1].to_string();
            if op == "OR" { op.push(' '); }
            let in1 = inps[0].to_string();
            let in2 = inps[2].to_string();
            acc.insert(out, (in1, op, in2));
            acc
        });

    let after0 = Instant::now();

    let start1 = Instant::now();

    let z_gates = gates
        .iter()
        .filter(|(out, _)| out.starts_with("z"))
        .map(|(out, _)| out.to_string())
        .collect::<Vec<String>>();
    let z_values = process_until_target_are_set(inputs, &gates, z_gates);
    let res1 = get_decimal_value(z_values);

    let after1 = Instant::now();
    if print_result {
        println!("Part 1: {}", res1);
    }

    let start2 = Instant::now();

    let res2 = (1..45)
        .map(|bit| check_back(bit, &gates) )
        .filter(|x| x.is_err())
        .map(|x| match x { Ok(gate) => {gate}, Err(gate) => gate })
        .collect::<Vec<String>>()
        .iter()
        .sorted()
        .join(",");

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

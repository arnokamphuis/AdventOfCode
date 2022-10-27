use super::tools;
use std::time::Instant;
use serde_json::{Value};

fn traverse(val: &Value, ignore_red: bool) -> i64 {
    let mut res = 0;
    let mut found_red = false;
    let mut list: Vec<Value> = vec![];
    match val {
        Value::Number(n) => { res += n.as_i64().unwrap() },
        Value::Array(arr) => { for a in arr { list.push(a.clone()); } },
        Value::Object(obj) => { 
            for (_,v) in obj { 
                if ignore_red {
                    if let Value::String(s) = v {
                        if s.eq("red") { 
                            found_red = true;
                        } 
                    } 
                }
                list.push(v.clone()); 
            } },
        _ => { },
    }
    if !found_red || !ignore_red {
        for item in &list {
            res += traverse(item, ignore_red);
        }
        res
    } else {
        0
    }
}

#[allow(dead_code)]
pub fn run(real: bool, print_result: bool) -> (u128, u128, u128) {
    let start0 = Instant::now();

    let input_file: &str = if !real {
        "./input/day12_15_test.txt"
    } else {
        "./input/day12_15_real.txt"
    };
    let input = tools::get_input(String::from(input_file));

    let v: Value = serde_json::from_str(&input[0]).unwrap();

    let after0 = Instant::now();

    let start1 = Instant::now();

    let res1 = traverse(&v, false);

    let after1 = Instant::now();
    if print_result {
        println!("Part 1: {}", res1);
    }

    let start2 = Instant::now();

    let res2 = traverse(&v, true);

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

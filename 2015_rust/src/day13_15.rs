use super::tools;
use std::time::Instant;
use std::collections::HashMap;
use permutohedron::heap_recursive;
use itertools::sorted;

#[allow(dead_code)]
pub fn run(real: bool, print_result: bool) -> (u128, u128, u128) {
    let start0 = Instant::now();

    let input_file: &str = if !real {
        "./input/day13_15_test.txt"
    } else {
        "./input/day13_15_real.txt"
    };
    let mut input = tools::get_input(String::from(input_file));

    let mut happiness: HashMap<(String,String), i64> = HashMap::new();
    for line in &mut input {
        line.pop();
        let words = line.split_whitespace().map(|s| s.to_string()).collect::<Vec<String>>();
        // words[0]
        // words[10]
        let val = words[3].parse::<i64>().unwrap();
        match words[2].as_str() {
            "gain" => { happiness.insert((words[0].clone(), words[10].clone()),  val); },
            "lose" => { happiness.insert((words[0].clone(), words[10].clone()), -val); },
            _ => { panic!("oeps"); },
        }
    }

    let mut names = sorted(happiness.iter().map(|((n,_),_)| n.clone()).collect::<Vec<String>>()).map(|s| s.to_string()).collect::<Vec<String>>();
    names.dedup();

    let determine_max_happiness = | names: &mut Vec<String>, happ: &HashMap<(String,String), i64> | -> i64 {
        let mut tables: HashMap<Vec<String>, i64> = HashMap::new();
        heap_recursive(names, |perm| {
            let mut h = 0i64;
            let p = perm.to_vec();
            for index in 0..p.len() {
                let index2 = (index+1) % p.len();
                h += happ.get(&(p[index].to_string(),p[index2].to_string())).unwrap();
                h += happ.get(&(p[index2].to_string(),p[index].to_string())).unwrap();
            }
            tables.insert(p.clone(), h);
        });
    
        tables.iter().map(|(_,&h)| h).max().unwrap()
    };

    // println!("{:?}", names);
    // println!("{:?}", happiness);

    let after0 = Instant::now();

    let start1 = Instant::now();

    let res1 = determine_max_happiness(&mut names, &happiness);

    let after1 = Instant::now();
    if print_result {
        println!("Part 1: {}", res1);
    }

    let start2 = Instant::now();

    for n in &names {
        happiness.insert((n.clone(), "Me".to_string()), 0);
        happiness.insert(("Me".to_string(), n.clone()), 0);
    }
    names.push("Me".to_string());
    
    let res2 = determine_max_happiness(&mut names, &happiness);

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

use super::tools;
use std::time::Instant;
use std::collections::BTreeSet;

#[allow(dead_code)]
pub fn run(real: bool, print_result: bool) -> (u128, u128, u128) {
    let start0 = Instant::now();

    let input_file: &str = if !real {
        "./input/day19_15_test.txt"
    } else {
        "./input/day19_15_real.txt"
    };
    let input = tools::get_input(String::from(input_file));

    let replacements: Vec<(String, String)> = input
        .iter()
        .filter(|line| line.split(" => ").count() > 1)
        .map(|line| {
            let forms = line.split(" => ").collect::<Vec<&str>>();
            (forms[0].to_string(), forms[1].to_string())
        })
        .collect();

    let target: String = input
        .iter()
        .filter(|line| line.split(" => ").count() == 1)
        .filter(|line| line.len() > 0)
        .collect::<Vec<&String>>()[0].to_string();

    let reducements: Vec<(String,String)> = replacements.iter().map(|(f,t)| (t.to_string(),f.to_string())).collect();

    let produce = | tbr: &String, rules: &Vec<(String, String)> | -> BTreeSet<String> {
        let mut new_mols: BTreeSet<String> = BTreeSet::new();
        for (f,t) in rules {
            let mut offset = 0;
            let mut index = 0;
            while let Some(p) = tbr.chars().skip(index).take(tbr.len()-index).collect::<String>().find(f) {
                let mut new_mol = tbr.clone();
                new_mol.replace_range(index+p..index+p+f.len(),t);
                new_mols.insert(new_mol.clone());

                index = offset+p+1;
                offset += p+1;
            }
        }
        new_mols
    };

    let after0 = Instant::now();

    let start1 = Instant::now();

    let res1 = produce(&target, &replacements).len();

    let after1 = Instant::now();
    if print_result {
        println!("Part 1: {}", res1);
    }

    let start2 = Instant::now();

    let mut reduced: BTreeSet<(String, usize)> = BTreeSet::new();
    reduced.insert((target.clone(),0));

    let mut minimum: usize;
    loop {
        let candidate = reduced.iter().min_by(|a,b| a.0.len().cmp(&b.0.len()) ).unwrap().clone();

        minimum = candidate.1;
        let res = produce(&candidate.0, &reducements);
        if res.contains(&String::from("e")) {
            break;
        }

        reduced.remove(&candidate);
        for r in &res {
            reduced.insert((r.to_string(), minimum+1));
        }
    }
    let res2 = minimum+1;

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

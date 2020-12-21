use super::tools;
use std::time::Instant;
use std::collections::{BTreeSet, BTreeMap};

#[allow(dead_code)]
pub fn run(real: bool, print_result: bool) -> (u128, u128, u128) {
    let start0 = Instant::now();

    let input_file: &str = if !real {
        "./input/day21_20_test.txt"
    } else {
        "./input/day21_20_real.txt"
    };
    let input = tools::get_input(String::from(input_file));

    let mut ingredients_count: BTreeMap<String, usize> = BTreeMap::new();
    let mut possibly_safe: BTreeSet<String> = BTreeSet::new();
    let mut allergen_in: BTreeMap<String, Vec<BTreeSet<String>>> = BTreeMap::new();
    let mut ingredient_contains: BTreeMap<String, Vec<BTreeSet<String>>> = BTreeMap::new();

    for line in &input {
        let to_be_parsed = line[0..line.len()-1].to_string();
        let parts: Vec<String> = to_be_parsed.split(" (contains ").map(|s| String::from(s)).collect();
        let ingredients: Vec<String> = parts[0].split(" ").map(|s| String::from(s)).collect();
        let allergens: Vec<String> = parts[1].split(", ").map(|s| String::from(s)).collect();

        for ing in &ingredients {
            *(ingredients_count.entry(ing.clone()).or_insert(0)) += 1;
            possibly_safe.insert(ing.clone());

            let set: BTreeSet<String> = allergens.iter()
                .fold(BTreeSet::new(), |mut acc, ing| { acc.insert(ing.clone()); return acc });
            ingredient_contains.entry(ing.clone()).or_insert(vec![]).push(set.clone());
        }

        for all in &allergens {
            let set: BTreeSet<String> = ingredients.iter()
                .fold(BTreeSet::new(), |mut acc, ing| { acc.insert(ing.clone()); return acc });
            allergen_in.entry(all.clone()).or_insert(vec![]).push(set.clone());
        }
    }

    let after0 = Instant::now();

    let start1 = Instant::now();

    for (_, all_in) in &allergen_in {
        let in_all = all_in
            .iter()
            .skip(1)
            .fold(all_in.first().unwrap().clone(), |gs, set| {
                gs.intersection(set).cloned().collect()
            });
        for ia in in_all {
            possibly_safe.remove(&ia);
        }
    }

    let res1 = possibly_safe.iter().fold(0, |acc, ps| { acc + ingredients_count[ps] });

    let after1 = Instant::now();
    if print_result {
        println!("Part 1: {}", res1);
    }

    let start2 = Instant::now();

    for all in possibly_safe {
        ingredient_contains.remove(&all);
    }

    let mut mapping: BTreeMap<String,String> = BTreeMap::new();
    let mut possibilities: BTreeMap<String, BTreeSet<String>> = BTreeMap::new();
    for (all_name, ing) in &allergen_in {
        let set = ing
            .iter()
            .skip(1)
            .fold(ing.first().unwrap().clone(), |gs, set| {
                gs.intersection(set).cloned().collect()
            });
        possibilities.insert(all_name.clone(), set.clone());
    }


    while mapping.len() != possibilities.len() {
        for poss in &possibilities {
            if poss.1.len() == 1 {
                mapping.insert(poss.0.clone(), poss.1.iter().next().unwrap().clone());
            }
        }

        for (_, t) in &mapping {
            for poss in possibilities.iter_mut() {
                poss.1.remove(t);
            }
        }

    }

    let res2 = mapping.iter().fold(String::from(""), |mut acc, (_, ing)| {
        acc.push(','); acc.push_str(ing); return acc
    })[1..].to_string();

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

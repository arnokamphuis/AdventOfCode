use super::tools;
use std::collections::BTreeMap;
use std::time::Instant;

#[derive(Clone)]
struct Reaction {
    from: BTreeMap<String, u64>,
    to: String,
    amount: u64,
}

fn split_reactors(s: &String) -> BTreeMap<String, u64> {
    let mut res: BTreeMap<String, u64> = BTreeMap::new();
    let reactors = s.split(", ");
    for reactor in reactors {
        let mut reactor_parts = reactor.split_whitespace();
        if let Ok(p) = reactor_parts.next().unwrap().parse::<u64>() {
            let reactant = reactor_parts.next().unwrap().to_string();
            res.insert(reactant.clone(), p);
        }
    }
    res.clone()
}

impl Reaction {
    fn new(line: &String) -> Reaction {
        let mut reaction_split = line.split(" => ");

        let from_parts = reaction_split.next().unwrap();
        let to_parts = reaction_split.next().unwrap();

        let to_reactants = split_reactors(&to_parts.to_string());
        let mut iter = to_reactants.iter();
        let mut to_r = String::from("");
        let mut amount_r = 0;
        while let Some(r) = iter.next() {
            to_r = r.0.to_string();
            amount_r = *r.1;
        }

        Reaction {
            from: split_reactors(&from_parts.to_string()),
            to: to_r,
            amount: amount_r,
        }
    }
}

#[derive(Clone)]
struct Nanofactory {
    reactions: Vec<Reaction>,
    inputs: BTreeMap<String, u64>,
}

impl Nanofactory {
    fn new() -> Nanofactory {
        Nanofactory {
            reactions: vec![],
            inputs: BTreeMap::new(),
        }
    }

    fn add_reaction(&mut self, r: &Reaction) {
        self.reactions.push(r.clone());
        for (from_r, _) in &r.from {
            *self.inputs.entry(from_r.to_string()).or_insert(0) += 1;
        }
        if !self.inputs.contains_key(&r.to.to_string()) {
            *self.inputs.entry(r.to.to_string()).or_insert(0) = 0;
        }
    }

    fn find_product(&self, reactant: &String) -> Vec<Reaction> {
        let mut res = vec![];
        for r in &self.reactions {
            if r.to == *reactant {
                res.push(r.clone());
            }
        }
        res
    }
}

#[allow(dead_code)]
pub fn run() {
    println!("Day 14 of 2019");

    let start0 = Instant::now();

    // let input_file = "./input/day14_19_test.txt";
    let input_file = "./input/day14_19_real.txt";
    let input = tools::get_input(String::from(input_file));

    let mut factory = Nanofactory::new();
    input.iter().for_each(|line| {
        factory.add_reaction(&Reaction::new(line));
    });

    let after0 = Instant::now();
    println!("Init in {:?}", after0.duration_since(start0));

    let start1 = Instant::now();

    let mut todo: Vec<String> = vec![];
    let mut have: BTreeMap<String, u64> = BTreeMap::new();
    let mut need: BTreeMap<String, u64> = BTreeMap::new();

    need.insert(String::from("FUEL"), 1);
    have.insert(String::from("FUEL"), 0);
    todo.push(String::from("FUEL"));

    let mut input_counters = factory.inputs.clone();

    let res1;

    loop {
        if need.len() == 1 && need.contains_key("ORE") {
            res1 = need["ORE"];
            break;
        }

        let mut target = String::from("");
        for (t, input_count) in &input_counters {
            if *input_count == 0 {
                target = t.clone();
                break;
            }
        }

        let needed = need[&target];
        let already_have = have[&target];

        if needed < already_have {
            have.entry(target.clone()).and_modify(|e| *e -= needed);
            need.remove(&target);
            todo.remove(0);
        } else {
            let reactions = &factory.find_product(&target);
            for reaction in reactions {
                let produces_amount = reaction.amount;

                let delta = needed - already_have;

                let mut factor = (delta) / produces_amount;
                if delta % produces_amount > 0 {
                    factor += 1;
                }

                for (reactant, required_amount) in &reaction.from {
                    let total_required = factor * required_amount;

                    input_counters
                        .entry(reactant.to_string())
                        .and_modify(|r| *r -= 1);

                    if !need.contains_key(reactant) {
                        need.insert(reactant.to_string(), 0);
                    }

                    need.entry(reactant.to_string())
                        .and_modify(|e| *e += total_required);

                    if !have.contains_key(reactant) {
                        have.insert(reactant.to_string(), 0);
                    }
                }

                let redundant = factor * produces_amount - delta;
                if redundant > 0 {
                    *have.entry(target.clone()).or_insert(0) = redundant;
                }
            }

            need.remove(&target);
        }

        input_counters.remove(&target);
    }

    let after1 = Instant::now();
    println!("Part 1: {}, in {:?}", res1, after1.duration_since(start1));

    let start2 = Instant::now();

    let after2 = Instant::now();
    println!("Part 2: {}, in {:?}", 0, after2.duration_since(start2));
}

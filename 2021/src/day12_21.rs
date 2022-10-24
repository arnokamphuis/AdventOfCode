use super::tools;
use std::time::Instant;
use std::collections::HashMap;
use std::collections::BTreeSet;

fn find_paths(current: usize, from: usize, to: usize, blocked: &BTreeSet<usize>, edges: &HashMap<usize,Vec<usize>>, part: u8, visited: Option<usize>, lower_ids: &BTreeSet<usize>) -> u64 {
    let mut pathcount = 0;

    edges[&current]
        .iter()
        .filter(|&t| *t != from)
        .for_each(|&t| {     
            let lower = lower_ids.contains(&t);           
            let already_visited = blocked.contains(&t);

            if t == to {
                pathcount += 1;
            } else {

                if !lower || !already_visited || (part==2 && already_visited && visited == None) {
                    let mut new_blocked = blocked.clone();
                    if lower {
                        new_blocked.insert(t);
                    }
                    
                    let mut new_visited = visited;
                    if part == 2 && visited == None && already_visited && lower {
                        new_visited = Some(t);
                    }
                    pathcount += find_paths(t, from, to, &new_blocked, edges, part, new_visited, lower_ids);
                }
            }
        });
    pathcount
}

#[allow(dead_code)]
pub fn run(real: bool, print_result: bool) -> (u128, u128, u128) {
    let start0 = Instant::now();

    let input_file: &str = if !real {
        "./input/day12_21_test.txt"
    } else {
        "./input/day12_21_real.txt"
    };
    let input = tools::get_input(String::from(input_file));

    let paths = input.iter().map(|line| {
        let mut tokens = line.split('-');
        let from = tokens.next().unwrap();
        let to = tokens.next().unwrap();
        (String::from(from),String::from(to))
    }).collect::<Vec<(String,String)>>();

    let mut lower_ids: BTreeSet<usize> = BTreeSet::new();
    let mut numbering: HashMap<String, usize> = HashMap::new();
    paths.iter().for_each(|(f,t)| {
        if !numbering.contains_key(f) { 
            if f.to_lowercase() == f.to_string() { lower_ids.insert(numbering.len()); }
            numbering.insert(f.to_string(), numbering.len()); 
        }
        if !numbering.contains_key(t) { 
            if t.to_lowercase() == t.to_string() { lower_ids.insert(numbering.len()); }
            numbering.insert(t.to_string(), numbering.len()); 
        }
    });

    let start_id = numbering[&"start".to_string()];
    let end_id = numbering[&"end".to_string()];

    let edges = paths.iter().fold(HashMap::new(), |mut map, (f,t)| {
        let f_id = numbering[f];
        let t_id = numbering[t];

        let edge = map.entry(f_id).or_insert(vec![]);
        edge.push(t_id);

        if !((t.to_string() == "end".to_string()) || (f.to_string() == "start".to_string())) {
            let edge = map.entry(t_id).or_insert(vec![]);
            edge.push(f_id);
        }
        map
    });

    let after0 = Instant::now();

    let start1 = Instant::now();

    let res1 = find_paths(start_id, start_id, end_id, &BTreeSet::new(), &edges, 1, None, &lower_ids);

    let after1 = Instant::now();
    if print_result {
        println!("Part 1: {}", res1);
    }

    let start2 = Instant::now();

    let res2 = find_paths(start_id, start_id, end_id, &BTreeSet::new(), &edges, 2, None, &lower_ids);

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

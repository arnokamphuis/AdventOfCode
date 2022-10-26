use super::tools;
use std::time::Instant;
use std::collections::HashMap;
use itertools::sorted;
use permutohedron::heap_recursive;

#[allow(dead_code)]
pub fn run(real: bool, print_result: bool) -> (u128, u128, u128) {
    let start0 = Instant::now();

    let input_file: &str = if !real {
        "./input/day09_15_test.txt"
    } else {
        "./input/day09_15_real.txt"
    };
    let input = tools::get_input(String::from(input_file));

    let distances: HashMap<(String, String), usize> = input.iter().map(|line| {
        let route_distances = line.split(" = ").map(|s| s.to_string()).collect::<Vec<String>>();
        let route = route_distances[0].split(" to ").map(|s| s.to_string()).collect::<Vec<String>>();
        let distance = route_distances[1].parse::<usize>().unwrap();
        ((route[0].clone(),route[1].clone()), distance)
    }).collect::<HashMap<(String,String), usize>>();

    let mut locations: Vec<String> = sorted(distances.iter().map(|((n1,n2),_)| vec![n1,n2]).flatten().map(|s| s.to_string()).collect::<Vec<String>>()).collect::<Vec<String>>();
    locations.dedup();

    let after0 = Instant::now();

    let start1 = Instant::now();

    let mut candidates = locations.clone();
    let mut permutations: HashMap<Vec<_>, usize> = HashMap::new();
    heap_recursive(&mut candidates, |perm| {
        let perm_vec = perm.to_vec();
        permutations.insert( 
            perm_vec.clone(), 
            perm_vec.windows(2).fold(0usize, |len, places| {
                let mut route = (places[0].clone(),places[1].clone());
                if !distances.contains_key(&route) {
                    route = (route.1,route.0);
                }
                len + distances.get(&route).unwrap()
            })
        );
    });

    let res1: usize = permutations.iter().map(|(_,d)| *d).min().unwrap();

    let after1 = Instant::now();
    if print_result {
        println!("Part 1: {}", res1);
    }

    let start2 = Instant::now();

    let res2: usize = permutations.iter().map(|(_,d)| *d).max().unwrap();

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

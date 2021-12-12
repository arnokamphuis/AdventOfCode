use super::tools;
use std::time::Instant;

fn find_paths(current_path: &Vec<String>, paths: &Vec<(String,String)>, part: u8, visited: Option<String>) -> u64 {
    let mut pathcount = 0;

    if let Some(last) = current_path.last() {
        paths
            .iter()
            .filter(|(f,t)| f == last && *t != "start".to_string())
            .for_each(|(_,t)| {     
                let lower = t.to_string() == t.to_lowercase();           
                let already_visited = current_path.contains(t);

                if *t == String::from("end") {
                    pathcount += 1;
                } else {

                    if !lower || !already_visited || (part==2 && already_visited && visited == None) {
                        let mut new_path = current_path.clone();
                        new_path.push(t.clone());
                        
                        let mut new_visited = visited.clone();
                        if part == 2 && visited == None && already_visited && lower {
                            new_visited = Some(t.to_string());
                        }
                        pathcount += find_paths(&new_path, paths, part, new_visited);
                    }
                }
            });
    }
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

    let mut paths = input.iter().map(|line| {
        let mut tokens = line.split('-');
        let from = tokens.next().unwrap();
        let to = tokens.next().unwrap();
        (String::from(from),String::from(to))
    }).collect::<Vec<(String,String)>>();

    paths.append(&mut paths.iter().map(|(f,t)| (t.clone(),f.clone())).collect::<Vec<(String,String)>>());
    paths.retain(|(f,t)| (*f != String::from("end")) && (*t != String::from("start")));

    let after0 = Instant::now();

    let start1 = Instant::now();

    let res1 = find_paths(&vec![String::from("start")], &paths, 1, None);

    let after1 = Instant::now();
    if print_result {
        println!("Part 1: {}", res1);
    }

    let start2 = Instant::now();

    let res2 = find_paths(&vec![String::from("start")], &paths, 2, None);

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

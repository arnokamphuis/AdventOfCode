use super::tools;
use std::time::Instant;
use std::collections::HashMap;
use priority_queue::DoublePriorityQueue;

fn find_path(map: &HashMap<(i16,i16), i8>, s: &Vec<(i16, i16)>, e: (i16, i16)) -> i16 {
    let mut minimum = i16::MAX;

    for start in s {
        let mut q = DoublePriorityQueue::new();
        let mut visited: Vec<(i16,i16)> = vec![];
        q.push(*start,0);
        visited.push(*start);

        let dir = vec![(-1,0), (1,0), (0,-1), (0,1)];

        'inner: while !q.is_empty() {
            let next = q.pop_min().unwrap();
            
            if next.0 == e { minimum = minimum.min(next.1); break 'inner; }
            
            if next.1 > minimum { break 'inner; }

            visited.push(next.0);

            let current_height = map.get(&next.0).unwrap();

            dir.iter().for_each(|d| {
                let mut p = next.0;
                p.0 += d.0; p.1 += d.1;
                if let Some(height) = map.get(&p) {
                    if (height - current_height) <= 1 && !visited.contains(&p) {
                        q.push(p, next.1 + 1);
                    }
                }
            });

        }
    }
    minimum
}

#[allow(dead_code)]
pub fn run(real: bool, print_result: bool) -> (u128, u128, u128) {
    let start0 = Instant::now();

    let input_file: &str = if !real {
        "./input/day12_22_test.txt"
    } else {
        "./input/day12_22_real.txt"
    };
    let input = tools::get_input(String::from(input_file));

    let mut start = (0,0);
    let mut end = (0,0);
    let mut map: HashMap<(i16,i16), i8> = HashMap::new();

    input.iter().enumerate().for_each(|(y, line)| {
        line.chars().enumerate().for_each(|(x, c)| {
            let p = (x as i16, y as i16);
            map.insert(p, match c {
                'S' => { start = p;  0 }
                'E' => { end   = p; 25 },
                _   => { c as i8 - 'a' as i8 }
            });
        });
    });

    let after0 = Instant::now();

    let start1 = Instant::now();

    let res1 = find_path(&map, &vec![start], end);

    let after1 = Instant::now();
    if print_result {
        println!("Part 1: {}", res1);
    }

    let start2 = Instant::now();

    let pot_start = map
        .iter()
        .filter(|(_,&h)| h == 0)
        .map(|(&c,_)| c)
        .collect::<Vec<_>>();

    let res2 = find_path(&map, &pot_start, end);

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

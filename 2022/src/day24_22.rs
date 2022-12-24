use super::tools;
use std::time::Instant;
use priority_queue::PriorityQueue;
use std::cmp::Reverse;

#[allow(dead_code)]
pub fn run(real: bool, print_result: bool) -> (u128, u128, u128) {
    let start0 = Instant::now();

    let input_file: &str = if !real {
        "./input/day24_22_test.txt"
    } else {
        "./input/day24_22_real.txt"
    };
    let input = tools::get_input(String::from(input_file));

    let start = [ 1i64, 0i64];
    let goal  = [ input[0].len() as i64 - 2, input.len() as i64 - 1 ];
    let max   = [ input[0].len() as i64 - 1, input.len() as i64 - 1 ];

    let blizzards: Vec<([i64;2],[i64;2])> = input
        .iter()
        .enumerate()
        .map(|(y,line)| {
            line
                .chars()
                .enumerate()
                .filter(|&(_,c)| c != '#' && c != '.' )
                .map(|(x, c)| {
                    ([x as i64, y as i64], match c {
                        '>' => [ 1, 0],
                        '<' => [-1, 0],
                        'v' => [ 0, 1],
                        '^' => [ 0,-1],
                        _ => panic!()
                    })})
                .collect::<Vec<([i64;2],[i64;2])>>() })
        .collect::<Vec<Vec<([i64;2],[i64;2])>>>()
        .iter()
        .flatten()
        .map(|&t| t)
        .collect::<Vec<([i64;2],[i64;2])>>();

    let update_blizzards = | t: i64, blzds: &Vec<([i64;2],[i64;2])> | -> Vec<[i64;2]> {
        blzds.iter().map(|blzd| {
            let mut p = blzd.0;
            let v = blzd.1;

            (0..2).for_each(|i| {
                p[i] += v[i] * t;
                p[i] = (p[i]-1).rem_euclid(max[i]-1)+1;
            });

            p
        }).collect()
    };

    let inside = | p: [i64;2] | -> bool { 
        (0..2).all(|i| p[i] > 0 && p[i] < max[i]) || p == start || p == goal
    };

    let directions = [ [-1i64, 0i64], [ 1i64, 0i64], [0i64, -1i64], [0i64, 1i64], [0i64, 0i64] ];

    let find = | start_pos: [i64;2], end_pos: [i64;2], time_offset: i64 | -> i64 {
        let state = (time_offset, start_pos);
        let mut pq = PriorityQueue::new();
        pq.push( state, Reverse(time_offset) );
        while let Some(((t,pos), _prio)) = pq.pop() {
            if pos == end_pos { return t; }
            let positions = update_blizzards(t+1, &blizzards);
            for dir in &directions {
                let next_pos = [pos[0] + dir[0], pos[1] + dir[1]];
                if inside(next_pos) && !positions.contains(&next_pos) {
                    let state = (t+1, next_pos);
                    pq.push(state, Reverse(t+1));
                }
            }
        }
        0
    };

    let after0 = Instant::now();

    let start1 = Instant::now();

    let res1 = find(start,goal,0);

    let after1 = Instant::now();
    if print_result {
        println!("Part 1: {}", res1);
    }

    let start2 = Instant::now();

    let res2 = find(start,goal, find(goal,start,res1));

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

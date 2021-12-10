use super::tools;
use std::time::Instant;
use std::collections::HashMap;

#[allow(dead_code)]
pub fn run(real: bool, print_result: bool) -> (u128, u128, u128) {
    let start0 = Instant::now();

    let input_file: &str = if !real {
        "./input/day10_21_test.txt"
    } else {
        "./input/day10_21_real.txt"
    };
    let input = tools::get_input(String::from(input_file));

    let startchars        = [   '<',  '{', '[', '('];
    let endchars          = [   '>',  '}', ']', ')'];
    let scores_1: [u64;4] = [ 25137, 1197,  57,   3];
    let scores_2: [u64;4] = [     4,    3,   2,   1];

    let belongs:         HashMap<char,char> = startchars.iter().zip(endchars.iter()).fold(HashMap::new(), |mut map, (f,t)| { map.insert(*t,*f); map } );
    let scores_1_lookup: HashMap<char,u64>  =   endchars.iter().zip(scores_1.iter()).fold(HashMap::new(), |mut map, (c,s)| { map.insert(*c,*s); map } );
    let scores_2_lookup: HashMap<char,u64>  = startchars.iter().zip(scores_2.iter()).fold(HashMap::new(), |mut map, (c,s)| { map.insert(*c,*s); map } );
    
    let after0 = Instant::now();

    let start1 = Instant::now();

    let mut scoredlines: Vec<(&String, u64, Vec<char>)> = input.iter().map(|line| {
        line
            .chars()
            .fold( (line, 0, vec![]), |(line, mut score, mut q), c| {
                if score==0 {
                    if startchars.contains(&c) {
                        q.push(c);
                    } else {
                        if let Some(sc)= q.pop() {
                            if belongs[&c] != sc { score += scores_1_lookup[&c]; } 
                        }
                    }
                }
                (line, score, q)
            })
    }).collect();

    let res1: u64 = scoredlines.iter().map(|(_,score,_)| score).filter(|&score| *score > 0).sum();

    let after1 = Instant::now();
    if print_result {
        println!("Part 1: {}", res1);
    }

    let start2 = Instant::now();

    scoredlines.retain(|(_,score,_)| *score == 0);
    let mut scores: Vec<u64> = scoredlines.iter().fold(vec![],|mut scores, (_,_,q)| {
        scores.push(q.iter().rev().fold(0, |s, c| { s * 5 + scores_2_lookup[&c] })); scores
    });
    scores.sort();
    let res2 = scores[ scores.len()/2 ];

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

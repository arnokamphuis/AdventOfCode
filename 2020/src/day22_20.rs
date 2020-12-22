use super::tools;
use std::time::Instant;
use std::collections::HashSet;

fn play_rc(s_p_deck: &Vec<usize>, s_o_deck: &Vec<usize>, part: usize) -> (usize, usize) {
    let score = |deck: &Vec<usize>| {
        deck.iter().enumerate().fold(0, |acc, (index, v)| acc + (deck.len() - index) * v )
    };

    let mut prev_p_deck: HashSet<Vec<usize>> = HashSet::new();
    let mut prev_o_deck: HashSet<Vec<usize>> = HashSet::new();

    let mut p_deck = s_p_deck.clone();
    let mut o_deck = s_o_deck.clone();

    while p_deck.len() != 0 && o_deck.len() != 0 {
        if part == 2 {
            if prev_p_deck.contains(&p_deck) || prev_o_deck.contains(&o_deck) {
                return (1, score(&p_deck));
            } else {
                prev_p_deck.insert(p_deck.clone());
                prev_o_deck.insert(o_deck.clone());
            }
        }

        let p = p_deck.remove(0);
        let o = o_deck.remove(0);

        let win_by;
        
        if part == 2 && p <= p_deck.len() && o <= o_deck.len() {
            win_by = play_rc(&p_deck[0..p].to_vec(), &o_deck[0..o].to_vec(), part).0;
        } else {
            win_by = if p > o { 1 } else { 2 }
        }

        if win_by == 1 { p_deck.push(p); p_deck.push(o) } else { o_deck.push(o); o_deck.push(p); }
    }

    if p_deck.len() > 0 {
        (1, score(&p_deck))
    } else {
        (2, score(&o_deck))
    }
}

#[allow(dead_code)]
pub fn run(real: bool, print_result: bool) -> (u128, u128, u128) {
    let start0 = Instant::now();

    let input_file: &str = if !real {
        "./input/day22_20_test.txt"
    } else {
        "./input/day22_20_real.txt"
    };
    let input = tools::get_input(String::from(input_file));

    let mut p_deck: Vec<usize> = vec![];
    let mut o_deck: Vec<usize> = vec![];


    let mut player_1 = true;
    for line in &input {
        if line == "Player 2:" { player_1 = false }

        if let Ok(v) = line.parse::<usize>() {
            if player_1 { p_deck.push(v); } else { o_deck.push(v); }
        }
    }

    let after0 = Instant::now();

    let start1 = Instant::now();

    let res1 = play_rc(&p_deck, &o_deck, 1).1;

    let after1 = Instant::now();
    if print_result {
        println!("Part 1: {}", res1);
    }

    let start2 = Instant::now();

    let res2 = play_rc(&p_deck, &o_deck, 2).1;

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

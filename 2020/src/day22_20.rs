use super::tools;
use std::time::Instant;
use std::collections::BTreeSet;

fn play_rc(s_p_deck: &Vec<usize>, s_o_deck: &Vec<usize>) -> (usize, Vec<usize>) {
    let mut prev_p_decks: BTreeSet<Vec<usize>> = BTreeSet::new();
    let mut prev_o_decks: BTreeSet<Vec<usize>> = BTreeSet::new();

    let mut p_deck = s_p_deck.clone();
    let mut o_deck = s_o_deck.clone();

    while p_deck.len()!=0 && o_deck.len()!= 0 {
        if prev_p_decks.contains(&p_deck) || prev_p_decks.contains(&o_deck) {
            return (1, p_deck.clone());
        } else {
            prev_p_decks.insert(p_deck.clone());
            prev_o_decks.insert(o_deck.clone());
        }

        let p = p_deck.remove(0);
        let o = o_deck.remove(0);

        let mut win_by = 0;
        if p <= p_deck.len() && o <= o_deck.len() {
            let (winner, _) = play_rc(&p_deck[0..p].to_vec(), &o_deck[0..o].to_vec());
            win_by = winner;
        } else {
            if p > o { win_by = 1 } else { win_by = 2 }
        }

        if win_by == 1 { p_deck.push(p); p_deck.push(o) } else { o_deck.push(o); o_deck.push(p); }
    }

    if p_deck.len() > 0 {
        (1, p_deck.clone())
    } else {
        (2, o_deck.clone())
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


    let mut player = true;
    for line in &input {
        if line == "Player 1:" { player = true }
        if line == "Player 2:" { player = false }

        if let Ok(v) = line.parse::<usize>() {
            if player { p_deck.push(v); } else { o_deck.push(v); }
        }
    }

    let orig_p_deck = p_deck.clone();
    let orig_o_deck = o_deck.clone();

    let after0 = Instant::now();

    let start1 = Instant::now();

    while p_deck.len()!=0 && o_deck.len()!= 0 {
        let p = p_deck.remove(0);
        let o = o_deck.remove(0);

        if p > o { p_deck.push(p); p_deck.push(o) } else { o_deck.push(o); o_deck.push(p); }
    }

    let winning_deck:Vec<usize> = if o_deck.len()==0 { p_deck.clone() } else { o_deck.clone() };
    let res1 = winning_deck.iter().enumerate().fold(0, |acc, v| acc + (winning_deck.len() - v.0) * v.1 );

    let after1 = Instant::now();
    if print_result {
        println!("Part 1: {}", res1);
    }

    let start2 = Instant::now();

    let res2 = play_rc(&orig_p_deck, &orig_o_deck).1.iter().enumerate().fold(0, |acc, v| acc + (winning_deck.len() - v.0) * v.1 );

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

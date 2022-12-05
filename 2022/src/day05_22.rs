use super::tools;
use std::time::Instant;

#[allow(dead_code)]
pub fn run(real: bool, print_result: bool) -> (u128, u128, u128) {
    let start0 = Instant::now();

    let input_file: &str = if !real {
        "./input/day05_22_test.txt"
    } else {
        "./input/day05_22_real.txt"
    };
    let input = tools::get_input(String::from(input_file));

    let emptyline = input
        .iter()
        .enumerate()
        .filter(|(_,line)| line.len() == 0 )
        .map(|(index,_)| index )
        .collect::<Vec<usize>>()[0];

    let n = input[emptyline-1]
        .trim()
        .chars()
        .rev()
        .collect::<String>()[0..1]
        .parse::<usize>()
        .unwrap();

    let mut piles: Vec<Vec<char>> = vec![vec![];n];
    input[..emptyline-1].iter().for_each(|line| { 
        line
            .as_bytes()
            .chunks(4)
            .map(|buf| unsafe { std::str::from_utf8_unchecked(buf) })
            .map(|s| s.to_string().chars().nth(1).unwrap())
            .enumerate()
            .for_each(|(pile, c)| if c != ' ' { piles[pile].insert(0,c); });
    });
    let original_piles = piles.clone();

    let moves = input[emptyline+1..].iter().map(|line| { 
        line
            .splitn(6, [' '])
            .filter(|part| !part.contains("o") )
            .map(|s| s.to_string() )
            .map(|n| n.parse::<usize>().unwrap() )
            .collect::<Vec<usize>>()
        })
        .collect::<Vec<_>>();

    let after0 = Instant::now();

    let start1 = Instant::now();

    moves
        .iter()
        .for_each(|moveit| {
            let from = moveit[1]-1;
            let to   = moveit[2]-1;
            let f_n  = piles[from].len().saturating_sub(moveit[0]);
            
            let pile = piles[from].drain(f_n..).rev().collect::<Vec<_>>();
            piles[to].extend(pile);
        });

    let res1 = piles
        .iter()
        .map(|pile| pile[pile.len()-1] )
        .collect::<String>();

    let after1 = Instant::now();
    if print_result {
        println!("Part 1: {}", res1);
    }

    let start2 = Instant::now();

    piles = original_piles.clone();
    moves
        .iter()
        .for_each(|moveit| {
            let from = moveit[1]-1;
            let to   = moveit[2]-1;
            let f_n  = piles[from].len().saturating_sub(moveit[0]);
            
            let pile = piles[from].drain(f_n..).collect::<Vec<_>>();
            piles[to].extend(pile);
        });

    let res2 = piles
        .iter()
        .map(|pile| pile[pile.len()-1] )
        .collect::<String>();

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

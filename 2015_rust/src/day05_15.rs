use super::tools;
use std::time::Instant;

#[allow(dead_code)]
pub fn run(real: bool, print_result: bool) -> (u128, u128, u128) {
    let start0 = Instant::now();

    let input_file: &str = if !real {
        "./input/day05_15_test.txt"
    } else {
        "./input/day05_15_real.txt"
    };
    let input = tools::get_input(String::from(input_file));

    let contains_three_vowels = | s: &String | -> bool {
        s.chars().fold(0, | count, c | count + (if "aeoui".find(c) != None { 1 } else { 0 })) > 2
    };

    let contains_two_letters_in_a_row = | s: &String | -> bool {
        s.chars().collect::<Vec<char>>().windows(2).fold(0, | count, c | count + (if c[0]==c[1] { 1 } else { 0 })) > 0
    };

    let contains_no_illegal_pairs = | s: &String | -> bool {
        let pairs = vec![('a','b'), ('c','d'), ('p','q'), ('x','y')];
        s.chars().collect::<Vec<char>>().windows(2).fold(0, | count, c | count + (if pairs.contains(&(c[0],c[1])) { 1 } else { 0 })) == 0
    };


    // It contains a pair of any two letters that appears at least twice in the string without overlapping, like xyxy (xy) or aabcdefgaa (aa), but not like aaa (aa, but it overlaps).
    let contains_two_pairs_non_overlapping = | s: &String | -> bool {
        s.chars().collect::<Vec<char>>().iter().enumerate().collect::<Vec<(usize,&char)>>().windows(2).fold(0, | count, c | {
            let rest = s.chars().skip(c[1].0+1).collect::<Vec<char>>();
            let mut it = rest.windows(2);
            let mut pc = 0usize;
            while it.position(|pair| &pair[0]==c[0].1 && &pair[1]==c[1].1) != None { pc+=1; }
            count + pc
        }) > 0
    };


    // It contains at least one letter which repeats with exactly one letter between them, like xyx, abcdefeghi (efe), or even aaa.
    let contains_one_pairs_skipping = | s: &String | -> bool {
        s.chars().collect::<Vec<char>>().windows(3).fold(0, | count, c | {
            count + if c[0] == c[2] { 1 } else { 0 }
        }) > 0
    };

    let after0 = Instant::now();

    let start1 = Instant::now();

    let res1 = input
        .iter()
        .map(|s| { contains_three_vowels(s) && contains_two_letters_in_a_row(s) && contains_no_illegal_pairs(s) } )
        .filter(|&b| b)
        .count();

    let after1 = Instant::now();
    if print_result {
        println!("Part 1: {}", res1);
    }

    let start2 = Instant::now();

    let res2 = input
        .iter()
        .map(|s| { contains_two_pairs_non_overlapping(s) && contains_one_pairs_skipping(s) } )
        .filter(|&b| b)
        .count();

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

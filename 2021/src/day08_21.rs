use super::tools;
use std::collections::BTreeSet;
use std::collections::HashMap;
use std::time::Instant;

#[allow(dead_code)]
pub fn run(real: bool, print_result: bool) -> (u128, u128, u128) {
    let start0 = Instant::now();

    let input_file: &str = if !real {
        "./input/day08_21_test.txt"
    } else {
        "./input/day08_21_real.txt"
    };
    let input = tools::get_input(String::from(input_file));

    let mut digits: HashMap<usize, BTreeSet<char>> = HashMap::new();
    digits.insert(0, BTreeSet::from(['a', 'b', 'c', 'e', 'f', 'g']));
    digits.insert(1, BTreeSet::from(['c', 'f']));
    digits.insert(2, BTreeSet::from(['a', 'c', 'd', 'e', 'g']));
    digits.insert(3, BTreeSet::from(['a', 'c', 'd', 'f', 'g']));
    digits.insert(4, BTreeSet::from(['b', 'c', 'd', 'f']));
    digits.insert(5, BTreeSet::from(['a', 'b', 'd', 'f', 'g']));
    digits.insert(6, BTreeSet::from(['a', 'b', 'd', 'e', 'f', 'g']));
    digits.insert(7, BTreeSet::from(['a', 'c', 'f']));
    digits.insert(8, BTreeSet::from(['a', 'b', 'c', 'd', 'e', 'f', 'g']));
    digits.insert(9, BTreeSet::from(['a', 'b', 'c', 'd', 'f', 'g']));

    let reverse_digits = digits.iter().fold(HashMap::new(), |mut m, (n,s)| {m.insert(s,n); m});

    let mut possibilities: HashMap<char, BTreeSet<char>> = HashMap::new();
    possibilities.insert('a', BTreeSet::from(['a', 'b', 'c', 'd', 'e', 'f', 'g']));
    possibilities.insert('b', BTreeSet::from(['a', 'b', 'c', 'd', 'e', 'f', 'g']));
    possibilities.insert('c', BTreeSet::from(['a', 'b', 'c', 'd', 'e', 'f', 'g']));
    possibilities.insert('d', BTreeSet::from(['a', 'b', 'c', 'd', 'e', 'f', 'g']));
    possibilities.insert('e', BTreeSet::from(['a', 'b', 'c', 'd', 'e', 'f', 'g']));
    possibilities.insert('f', BTreeSet::from(['a', 'b', 'c', 'd', 'e', 'f', 'g']));
    possibilities.insert('g', BTreeSet::from(['a', 'b', 'c', 'd', 'e', 'f', 'g']));

    let mut firsts: Vec<Vec<BTreeSet<char>>> = vec![];
    let mut seconds: Vec<Vec<BTreeSet<char>>> = vec![];
    input.iter().for_each(|line| {
        let twoparts: Vec<String> = line.split(" | ").fold(vec![], |mut list: Vec<String>, s| {
            list.push(String::from(s));
            list
        });
        let first: Vec<BTreeSet<char>> =
            twoparts[0]
                .split_whitespace()
                .fold(vec![], |mut list: Vec<BTreeSet<char>>, s| {
                    list.push(s.chars().collect());
                    list
                });
        firsts.push(first);
        let second: Vec<BTreeSet<char>> =
            twoparts[1]
                .split_whitespace()
                .fold(vec![], |mut list: Vec<BTreeSet<char>>, s| {
                    list.push(s.chars().collect());
                    list
                });
        seconds.push(second);
    });

    let after0 = Instant::now();

    let start1 = Instant::now();

    let mut res1 = 0;
    seconds.iter().for_each(|second| {
        let digits = second
            .iter()
            .filter(|s| s.len() == 2 || s.len() == 3 || s.len() == 4 || s.len() == 7)
            .map(|v| v.clone())
            .collect::<Vec<BTreeSet<char>>>();
        res1 += digits.iter().count();
    });

    let after1 = Instant::now();
    if print_result {
        println!("Part 1: {}", res1);
    }

    let start2 = Instant::now();

    let res2: usize = firsts
        .iter()
        .zip(seconds.iter())
        .map(|(first, second)| {
            let mut poss = possibilities.clone();

            first.iter().filter(|f| f.len()==2).for_each(|f| {
                if let Some(cc) = poss.get_mut(&'c') { cc.retain(|ch| f.contains(ch)); }
                if let Some(cf) = poss.get_mut(&'f') { cf.retain(|ch| f.contains(ch)); }
            });

            first.iter().filter(|f| f.len()==3).for_each(|f| {
                if let Some(ca) = poss.get_mut(&'a') { ca.retain(|ch| f.contains(ch)); }
                let new_ca = poss[&'a'].difference(&poss[&'c']).cloned().collect(); 
                if let Some(ca) = poss.get_mut(&'a') { *ca = new_ca }
            });

            for ch in ['b', 'c', 'd', 'e', 'f', 'g'] {
                let new_set = poss[&ch].difference(&poss[&'a']).cloned().collect();
                if let Some(set) = poss.get_mut(&ch) { *set = new_set }
            }

            first.iter().filter(|f| f.len()==4).for_each(|f| {
                let new_set = poss[&'b'].intersection(f).cloned().collect(); if let Some(s) = poss.get_mut(&'b') { *s = new_set }
                let new_set = poss[&'c'].intersection(f).cloned().collect(); if let Some(s) = poss.get_mut(&'c') { *s = new_set }
                let new_set = poss[&'d'].intersection(f).cloned().collect(); if let Some(s) = poss.get_mut(&'d') { *s = new_set }
                let new_set = poss[&'f'].intersection(f).cloned().collect(); if let Some(s) = poss.get_mut(&'f') { *s = new_set }
            });

            let all = BTreeSet::from(['a', 'b', 'c', 'd', 'e', 'f', 'g']);
            let mut inter: BTreeSet<char> = all.clone();
            first.iter().filter(|f| f.len()==6).for_each(|f| {
                inter = inter.intersection(f).cloned().collect();
            });
            let new_set: BTreeSet<char> = poss[&'c'].difference(&inter).cloned().collect(); if let Some(s) = poss.get_mut(&'c') { *s = new_set.clone() }
            let new_set: BTreeSet<char> = poss[&'d'].difference(&inter).cloned().collect(); if let Some(s) = poss.get_mut(&'d') { *s = new_set.clone() }
            let new_set: BTreeSet<char> = poss[&'e'].difference(&inter).cloned().collect(); if let Some(s) = poss.get_mut(&'e') { *s = new_set.clone() }


            for ch in ['b', 'd', 'e', 'f', 'g'] {
                let new_set = poss[&ch].difference(&poss[&'c']).cloned().collect();
                if let Some(set) = poss.get_mut(&ch) { *set = new_set }
            }

            for ch in ['b', 'd', 'e', 'g'] {
                let new_set = poss[&ch].difference(&poss[&'f']).cloned().collect();
                if let Some(set) = poss.get_mut(&ch) { *set = new_set }
            }

            let mut inter: BTreeSet<char> = all.clone();
            first.iter().filter(|f| f.len()==5).for_each(|f| {
                inter = inter.intersection(f).cloned().collect();
            });
            for ch in ['a', 'c', 'd', 'f'] {
                inter = inter.difference(&poss[&ch]).cloned().collect();
            }
            if let Some(s) = poss.get_mut(&'g') { *s = inter.clone() }

            for ch in ['b', 'd', 'e'] {
                let new_set = poss[&ch].difference(&poss[&'g']).cloned().collect();
                if let Some(set) = poss.get_mut(&ch) { *set = new_set }
            }

            for ch in ['b', 'e'] {
                let new_set = poss[&ch].difference(&poss[&'d']).cloned().collect();
                if let Some(set) = poss.get_mut(&ch) { *set = new_set }
            }

            let translation: HashMap<char,char> = poss.iter().fold(HashMap::new(), |mut hm, (c,set)| {
                hm.insert(set.clone().iter().cloned().collect::<Vec<char>>()[0], *c); hm
            });

            second.iter().fold(0, |value, s| {
                let news = s.iter().map(|d| translation[d]).collect::<BTreeSet<_>>();
                value * 10 + reverse_digits[&news]
            })
        }).sum();

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

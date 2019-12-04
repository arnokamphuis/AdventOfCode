use std::time::{Instant};

fn check_rules(pw: &[u8; 6], strict: bool) -> bool {
    let mut increasing = true;

    pw.iter().enumerate().for_each( |(index, v)| if index<5 && v > &pw[index+1] { increasing=false; } );

    if increasing {
        let mut counter = [0;10];
        pw.iter().for_each(|n| { counter[*n as usize] += 1; } );

        let mut doublefound : bool = false;
        if strict {
            counter.iter().for_each(|c| if *c == 2 { doublefound = true; });
        } else {
            counter.iter().for_each(|c| if *c >= 2 { doublefound = true; });
        }

        doublefound
    } else {
        increasing
    }
}

#[allow(dead_code)]
pub fn run() {
    println!("Day 14 of 2019");

    const RADIX: u32 = 10;

    let start0 = Instant::now();    

    let after0 = Instant::now();
    println!("Init in {:?}", after0.duration_since(start0));

    let start1 = Instant::now();

    let mut code : [u8; 6] = [0; 6];

    let mut count = 0;
    (172851..675870).into_iter().for_each(|i| {
        format!("{}",i).chars().enumerate().for_each(|(i,v)| code[i] = v.to_digit(RADIX).unwrap() as u8);
        if check_rules(&code, false) {
            count += 1;
        }
    });

    let after1 = Instant::now();
    println!("Part 1: {}, in {:?}", count, after1.duration_since(start1));

    let start2 = Instant::now();
    
    count = 0;
    for i in 172851..675870 {
        const RADIX: u32 = 10;
        format!("{}",i).chars().enumerate().for_each(|(i,v)| code[i] = v.to_digit(RADIX).unwrap() as u8);
        if check_rules(&code, true) {
            // println!("{:?}", code);
            count += 1;
        }
    }

    let after2 = Instant::now();
    println!("Part 2: {}, in {:?}", count, after2.duration_since(start2));
}

use std::time::Instant;

#[allow(dead_code)]
pub fn run() {
    println!("Day 19 of 2016");

    let start0 = Instant::now();

    let input = 3014603;
    // let input = 5;
    let mut presents: Vec<bool> = vec![true; input];

    let after0 = Instant::now();
    println!(
        "Init in {:?}",
        after0.duration_since(start0)
    );

    let start1 = Instant::now();    

    let mut offset;
    let mut active = 0;
    loop {
        // println!("{:?}", presents);

        while !presents[active] { active = (active+1) % input; }
        offset = (active+1) % input;
        while !presents[offset] && active != offset {
            offset = (offset+1) % input;
        }
        if active == offset {
            break;
        }
        presents[offset] = false;

        active = (active + 1) % input;
    }

    let mut elf_with_presents = 0;
    presents.iter().enumerate().for_each(|(i,e)| if *e { elf_with_presents = i+1; } );

    let after1 = Instant::now();
    println!(
        "Part 1: {}, in {:?}",
        elf_with_presents,
        after1.duration_since(start1)
    );

    let start2 = Instant::now();


    let input2 = input;
    let mut presents2 = vec![0; input2];
    for i in 0..input2 {
        presents2[i] = i+1;
    }

    let mut cur_index = 0;
    while presents2.len()!=1 {
        let half_size = presents2.len()/2;
        let remove_index = (cur_index + half_size) % presents2.len();
        if remove_index < cur_index {
            cur_index -= 1;
        }
        presents2.remove(remove_index);
        cur_index = (cur_index + 1) % presents2.len();
    }

    elf_with_presents = presents2[0];

    let after2 = Instant::now();
    println!(
        "Part 2: {}, in {:?}",
        elf_with_presents,
        after2.duration_since(start2)
    );
}

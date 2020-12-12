use super::tools;
use std::time::Instant;
use tools::Image;

#[allow(dead_code)]
pub fn seats_map(count: usize, row: i16, col: i16, img: &mut Image, save: bool, target: bool) {
    img.set_pixel(2*col as usize + 1, 2*row as usize + 1, if save { if target {(0,0,255,255)} else {(255,0,0,255)}} else {(200,200,200,255)});
    if save {
        img.save_png(&format!("images/seats-{:05}.png", count));
    }
}

#[allow(dead_code)]
pub fn run(real: bool, print_result: bool) -> (u128, u128, u128) {
    let start0 = Instant::now();

    let input_file: &str = if !real {
        "./input/day05_20_test.txt"
    } else {
        "./input/day05_20_real.txt"
    };
    let input = tools::get_input(String::from(input_file));

    // let mut count = 0;
    // let mut img: Image = Image::new(257, 17, 8);
    // img.clear((255,255,255,255));
    // for i in 0..1024 {
    //     seats_map(0, i&7, i>>3, &mut img, false, false);
    // }

    let mut boardingcards: Vec<i16> = vec![];
    for line in &input {
        let card = line.chars().fold(0, |score, c| match c {
            'B' | 'R' => (score << 1) + 1,
            'F' | 'L' => (score << 1),
            _ => score,
        });
        // seats_map(count, card & 7, card >> 3, &mut img, true, false); count+=1;
        boardingcards.push(card);
    }
    boardingcards.sort();

    let after0 = Instant::now();

    let start1 = Instant::now();

    let res1 = boardingcards.iter().last().unwrap();

    let after1 = Instant::now();
    if print_result {
        println!("Part 1: {}", res1);
    }

    let start2 = Instant::now();

    let res2 = boardingcards
        .iter()
        .enumerate()
        .take(boardingcards.len() - 1)
        .skip(1)
        .filter(|(i, _)| boardingcards[i - 1] + 2 != boardingcards[i + 1])
        .map(|(_, v)| *v)
        .collect::<Vec<i16>>()
        .first()
        .unwrap()
        + 1;

    // for _ in 0..200 {
    //     seats_map(count, res2 & 7, res2 >> 3, &mut img, true, true); count+=1;
    // }

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

use super::tools;
use std::time::Instant;
use std::collections::HashSet;
use font8x8::{BASIC_FONTS, UnicodeFonts};
use tools::Image;

fn play_rc(s_p_deck: &Vec<usize>, s_o_deck: &Vec<usize>, part: usize, depth: usize, img_c: &mut usize) -> (usize, usize) {
    let create_movie = false;
    let score = |deck: &Vec<usize>| {
        deck.iter().enumerate().fold(0, |acc, (index, v)| acc + (deck.len() - index) * v )
    };

    let mut prev_p_deck: HashSet<Vec<usize>> = HashSet::new();
    let mut prev_o_deck: HashSet<Vec<usize>> = HashSet::new();

    let mut p_deck = s_p_deck.clone();
    let mut o_deck = s_o_deck.clone();

    let mut round = 0;
    let mut img_count = *img_c;

    if create_movie {
        hand_image(s_p_deck, s_o_deck, depth, round, &mut img_count);
        round += 1;
    }

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
            win_by = play_rc(&p_deck[0..p].to_vec(), &o_deck[0..o].to_vec(), part, depth+1, &mut img_count).0;
        } else {
            win_by = if p > o { 1 } else { 2 }
        }

        if win_by == 1 { p_deck.push(p); p_deck.push(o) } else { o_deck.push(o); o_deck.push(p); }

        if create_movie {
            hand_image(&p_deck, &o_deck, depth, round, &mut img_count);
            round += 1;
        }
    }

    if create_movie && depth == 1 {
        round -= 1;
        for _ in 0..10 { hand_image(&p_deck, &o_deck, depth, round, &mut img_count); }
    }

    *img_c = img_count;

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

    let mut img_count = 0;

    let after0 = Instant::now();

    let start1 = Instant::now();

    let res1 = play_rc(&p_deck, &o_deck, 1, 1, &mut img_count).1;

    let after1 = Instant::now();
    if print_result {
        println!("Part 1: {}", res1);
    }

    let start2 = Instant::now();

    let res2 = play_rc(&p_deck, &o_deck, 2, 1, &mut img_count).1;

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


// -----------------------------------------------------------------------------------------------------
// -----------------------------------------------------------------------------------------------------
// -----------------------------------------------------------------------------------------------------
// -----------------------------------------------------------------------------------------------------

fn hand_image(p_deck: &Vec<usize>, o_deck: &Vec<usize>, game_depth: usize, round: usize, img_count: &mut usize) {
    let mut img = Image::new(292,128,4);
    img.clear((255,255,255,255));
    let left_offset = (4, 168);
    let top_offset = 4;

    // print round
    let c0_index: u8 = round as u8 / 100;
    if c0_index != 0 {
        if let Some(glyph) = BASIC_FONTS.get( (c0_index + b'0') as char ) {
            for (i,x) in glyph.iter().enumerate() {
                for bit in 0..8 {
                    let coor = (132+bit, top_offset+i);
                    match *x & 1 << bit {
                        0 => img.set_pixel(coor.0, coor.1, (255,255,255,255)),
                        _ => img.set_pixel(coor.0, coor.1, (  0,  0,  0,255)),
                    }
                }
            }
        }
    }

    let c1_index: u8 = (round as u8 / 10) % 10;
    if c1_index != 0 {
        if let Some(glyph) = BASIC_FONTS.get( (c1_index + b'0') as char ) {
            for (i,x) in glyph.iter().enumerate() {
                for bit in 0..8 {
                    let coor = (140+bit, top_offset+i);
                    match *x & 1 << bit {
                        0 => img.set_pixel(coor.0, coor.1, (255,255,255,255)),
                        _ => img.set_pixel(coor.0, coor.1, (  0,  0,  0,255)),
                    }
                }
            }
        }
    }

    let c2_index: u8 = round as u8 % 10;
    if let Some(glyph) = BASIC_FONTS.get( (c2_index + b'0') as char ) {
        for (i,x) in glyph.iter().enumerate() {
            for bit in 0..8 {
                let coor = (148+bit, top_offset+i);
                match *x & 1 << bit {
                    0 => img.set_pixel(coor.0, coor.1, (255,255,255,255)),
                    _ => img.set_pixel(coor.0, coor.1, (  0,  0,  0,255)),
                }
            }
        }
    }

    // print depth
    let c1_index: u8 = game_depth as u8 / 10;
    if c1_index != 0 {
        if let Some(glyph) = BASIC_FONTS.get( (c1_index + b'0') as char ) {
            for (i,x) in glyph.iter().enumerate() {
                for bit in 0..8 {
                    let coor = (140+bit, top_offset+i+24);
                    match *x & 1 << bit {
                        0 => img.set_pixel(coor.0, coor.1, (255,255,255,255)),
                        _ => img.set_pixel(coor.0, coor.1, (  0,  0,  0,255)),
                    }
                }
            }
        }
    }

    let c2_index: u8 = game_depth as u8 % 10;
    if let Some(glyph) = BASIC_FONTS.get( (c2_index + b'0') as char ) {
        for (i,x) in glyph.iter().enumerate() {
            for bit in 0..8 {
                let coor = (148+bit, top_offset+i+24);
                match *x & 1 << bit {
                    0 => img.set_pixel(coor.0, coor.1, (255,255,255,255)),
                    _ => img.set_pixel(coor.0, coor.1, (  0,  0,  0,255)),
                }
            }
        }
    }



    // print cards
    for (index, &card) in p_deck.iter().enumerate() {
        let x = index % 5;
        let y = index / 5;
        let off_x = x * 24;
        let off_y = y * 12;
        let color = if index==0 {(255,0,0,255)} else {(0,255,0,255)};

        let c1_index: u8 = card as u8 / 10;

        if c1_index != 0 {
            if let Some(glyph) = BASIC_FONTS.get( (c1_index + b'0') as char ) {
                for (i,x) in glyph.iter().enumerate() {
                    for bit in 0..8 {
                        let coor = (left_offset.0+off_x+bit, top_offset+off_y+i);
                        match *x & 1 << bit {
                            0 => img.set_pixel(coor.0, coor.1, (255,255,255,255)),
                            _ => img.set_pixel(coor.0, coor.1, color),
                        }
                    }
                }
            }
        }

        let c2_index: u8 = card as u8 % 10;
        if let Some(glyph) = BASIC_FONTS.get( (c2_index + b'0') as char ) {
            for (i,x) in glyph.iter().enumerate() {
                for bit in 0..8 {
                    let coor = (left_offset.0+off_x+8+bit, top_offset+off_y+i);
                    match *x & 1 << bit {
                        0 => img.set_pixel(coor.0, coor.1, (255,255,255,255)),
                        _ => img.set_pixel(coor.0, coor.1, color),
                    }
                }
            }
        }
    }

    for (index, &card) in o_deck.iter().enumerate() {
        let x = index % 5;
        let y = index / 5;
        let off_x = x * 24;
        let off_y = y * 12;
        let color = if index==0 {(255,0,0,255)} else {(0,0,255,255)};

        let c1_index: u8 = card as u8 / 10;

        if c1_index != 0 {
            if let Some(glyph) = BASIC_FONTS.get( (c1_index + b'0') as char ) {
                for (i,x) in glyph.iter().enumerate() {
                    for bit in 0..8 {
                        let coor = (left_offset.1+off_x+bit, top_offset+off_y+i);
                        match *x & 1 << bit {
                            0 => img.set_pixel(coor.0, coor.1, (255,255,255,255)),
                            _ => img.set_pixel(coor.0, coor.1, color),
                        }
                    }
                }
            }
        }

        let c2_index: u8 = card as u8 % 10;
        if let Some(glyph) = BASIC_FONTS.get( (c2_index + b'0') as char ) {
            for (i,x) in glyph.iter().enumerate() {
                for bit in 0..8 {
                    let coor = (left_offset.1+off_x+8+bit, top_offset+off_y+i);
                    match *x & 1 << bit {
                        0 => img.set_pixel(coor.0, coor.1, (255,255,255,255)),
                        _ => img.set_pixel(coor.0, coor.1, color),
                    }
                }
            }
        }
    }

    img.save_png(&format!("images/cards-{:010}.png", img_count));
    *img_count += 1;
}


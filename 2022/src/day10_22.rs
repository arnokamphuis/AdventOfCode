use super::tools;
use std::time::Instant;
// use tools::Image;
use advent_of_code_ocr::parse_string_to_letters;

#[allow(dead_code)]
pub fn run(real: bool, print_result: bool) -> (u128, u128, u128) {
    let start0 = Instant::now();

    let input_file: &str = if !real {
        "./input/day10_22_test.txt"
    } else {
        "./input/day10_22_real.txt"
    };
    let input = tools::get_input(String::from(input_file));

    let program_parsed = input
        .iter()
        .map(|line| {
            let parts = line.split_whitespace().collect::<Vec<&str>>();
            let cycles = match parts[0] {
                "addx" => 2,
                _ => 1,
            };
            if parts.len() > 1 {
                (parts[0], cycles, parts[1].parse::<i64>().unwrap() )
            } else {
                (parts[0], cycles, 0i64 )
            }
        })
        .collect::<Vec<(&str, usize, i64)>>();

    let mut program: Vec<((usize,usize), (&str, i64))> = vec![];
    let mut pc = 0;
    program_parsed.iter().for_each(|cmd| {
        match cmd.0 {
            "addx" => {
                let r = (pc+1, pc+cmd.1);
                program.push((r, (cmd.0, cmd.2)));
            },
            _ => {},
        }
        pc += cmd.1;
    });

    let after0 = Instant::now();

    let start1 = Instant::now();

    let mut register: i64 = 1;
    let mut strengths: Vec<(usize,i64)> = vec![];

    pc = 0;
    program
        .iter()
        .for_each(|cmd| {
            while pc < cmd.0.1 { 
                pc += 1; 
                strengths.push((pc, register));
            }
            match cmd.1.0 {
                "addx" => { register += cmd.1.1; },
                _ => {}
            }
        });
    while pc < 240 { 
        pc += 1; 
        strengths.push((pc, register));
    }

    let res1 = strengths
        .iter()
        .filter(|&(c,_)| c >= &20 && (c - 20) % 40 == 0 )
        .fold(0, |acc, &(c,s)| acc + c as i64 * s);

    let after1 = Instant::now();
    if print_result {
        println!("Part 1: {}", res1);
    }

    let start2 = Instant::now();

    let pixels = strengths.iter().filter(|(c,s)| {
        (*c % 40) as i64 >= *s && (*c % 40) as i64 <= s+2
    }).collect::<Vec<_>>();

    let mut crt = vec![vec![' ';40];6];
    pixels.iter().for_each(|(c,_)| {
        crt[ *c / 40 ][ *c % 40 ] = '█';
    });

    let mut crt_str = vec![];
    crt.iter().for_each(|l| { l.iter().skip(1).for_each(|&c| crt_str.push(if c != ' ' {'#'} else {'.'})); crt_str.push('\n');});
    let res2 = parse_string_to_letters(&crt_str.iter().collect::<String>());

    // let masks = create_character_masks('A', 'Z', 4, 6);
    // for index in 0..8 {
    //     let mut c: Vec<Vec<char>> = vec![vec![' ';4];6];
    //     for y in 0..6 {
    //         for x in 0..4 {
    //             c[y][x] = crt[y][1+index*5+x];
    //         }
    //     }
    //     println!("--> {}", find_character(&c, &masks));
    // }

    /*******************************************************************/
    // let mut img: Image = Image::new(40,6,80);
    // img.clear((0,0,0,255));
    // let mut counter = 0;
    // for i in 0..6 { for j in 0..40 {
    //     img.set_pixel( j, i, (0, if crt[i][j]==' ' { 0 } else { 255 },0,255));
    //     if !(j+1 >= 40 && i==5) {
    //         img.set_pixel( (j+1)%40, i + if j+1 >= 40 {1} else {0}, (0, 100 ,0,255));
    //     }
    //     img.save_png(&format!("images/day10_22/crt_{:05}.png", counter));
    //     counter += 1;
    // }}
    /*******************************************************************/

    // if print_result {
    //     for i in 0..6 { for j in 0..40 { print!("{}", crt[i][j]); } println!(""); }
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

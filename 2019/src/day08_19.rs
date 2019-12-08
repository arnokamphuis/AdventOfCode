use std::time::{Instant};
use super::tools;

pub fn run() {
    println!("Day 8 of 2019");

    let start0 = Instant::now();

    let input_file = "./input/day08_19_real.txt";
    let input = tools::get_input(String::from(input_file));

    const WIDTH: usize = 25;
    const HEIGHT: usize = 6;
    let pixelcount = input[0].len();
    let layercount = pixelcount / (WIDTH*HEIGHT);

    let mut final_image: Vec<Vec<u32>> = vec![vec![0; WIDTH]; HEIGHT];
    let mut layers: Vec<Vec<Vec<u32>>> = vec![vec![vec![0; WIDTH]; HEIGHT]; layercount];

    const RADIX: u32 = 10;
    let pixels: Vec<u32> = input[0].chars().map(|c| c.to_digit(RADIX).unwrap() as u32).collect();

    let after0 = Instant::now();
    println!("Init in {:?}", after0.duration_since(start0));

    let start1 = Instant::now();

    let mut histograms: Vec<Vec<u32>> = vec![vec![0; 3]; layercount];
    pixels.chunks(WIDTH*HEIGHT).enumerate().for_each( |(l,layer)| {
        let mut layerhist: Vec<u32> = vec![0; 3];

        layer.chunks(WIDTH).enumerate().for_each( |(r, row)| {
            row.iter().enumerate().for_each( |(c, p)| { 
                layerhist[*p as usize] += 1;
                layers[l][r][c] = *p;
            });
        });

        layerhist.iter().enumerate().for_each(|(i,h)| { histograms[l][i] = *h; });
    });

    let mut least_zeros: Vec<u32> = vec![std::u32::MAX; 3];

    histograms.iter().for_each(|h| { if h[0] < least_zeros[0] { least_zeros = h.to_vec(); } } );

    let res1 = least_zeros[1] * least_zeros[2] ;

    let after1 = Instant::now();
    println!("Part 1: {}, in {:?}", res1, after1.duration_since(start1));

    let start2 = Instant::now();

    layers.iter().rev().enumerate().for_each(|(l, layer)| {
        match l {
            0 => {
                final_image = layer.clone();
            }
            _ => {
                layer.iter().enumerate().for_each( |(r,row)| {
                    row.iter().enumerate().for_each( |(c, p)| {
                        if *p != 2 { final_image[r][c] = *p; }
                    });
                });
            }
        }
    });

    println!("");
    final_image.iter().for_each( |row| {
        row.iter().for_each( |p| { 
            match *p {
                0 => { print!(" "); }
                1 => { print!("#"); }
                _ => {}
            }
        });
        print!("\n");
    });
    println!("");

    let after2 = Instant::now();
    println!("Part 2: {}, in {:?}", "see pixels above", after2.duration_since(start2));
}

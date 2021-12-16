use super::tools;
use std::time::Instant;

fn parse_packet(packet: &Vec<bool>, start_pos: &mut usize) -> (u64, u64) {
    let packetversion = packet[*start_pos+0..*start_pos+3].iter().fold(0, |n, &b| (n << 1) + b as u64);
    let packettypeid = packet[*start_pos+3..*start_pos+6].iter().fold(0, |n, &b| (n << 1) + b as u64);

    let mut res = (packetversion,0);

    match packettypeid {
        4 => { // literal
            let mut pc = 0;
            let mut val = 0;
            loop {
                val = packet[*start_pos+7+5*pc..*start_pos+11+5*pc].iter().fold(val, |n, &b| (n << 1) + b as u64);
                if !packet[*start_pos+6+5*pc] { break; }
                pc += 1;
            }
            *start_pos += 11+5*pc;
            res = (packetversion, val);
        },
        _ => { // operator
            let mut values = vec![];
            match packet[*start_pos+6] {
                false => { // total length
                    let total_length = packet[*start_pos+7..*start_pos+22].iter().fold(0, |n, &b| (n << 1) + b as usize);
                    *start_pos += 22;
                    let from = *start_pos;
                    while *start_pos - from < total_length {
                        let temp_res = parse_packet(packet, start_pos);
                        values.push(temp_res.1);
                        res = (res.0 + temp_res.0, res.1 + temp_res.1);
                    }
                },
                true => { //  subpackets
                    let packet_count = packet[*start_pos+7..*start_pos+18].iter().fold(0, |n, &b| (n << 1) + b as u64);
                    *start_pos += 18;
                    (0..packet_count).for_each(|_| {
                        let temp_res = parse_packet(packet, start_pos);
                        values.push(temp_res.1);
                        res = (res.0 + temp_res.0, res.1 + temp_res.1);
                    });
                }
            }

            match packettypeid {
                0 => { res.1 = values.iter().sum(); },
                1 => { res.1 = values.iter().fold(1, |p, v| p * v); },
                2 => { res.1 = *values.iter().min().unwrap(); },
                3 => { res.1 = *values.iter().max().unwrap(); },
                5 => { res.1 = (values[0] > values[1]) as u64 },
                6 => { res.1 = (values[0] < values[1]) as u64 },
                7 => { res.1 = (values[0] == values[1]) as u64 },
                _ => panic!()
            }
        }
    }
    res
}

#[allow(dead_code)]
pub fn run(real: bool, print_result: bool) -> (u128, u128, u128) {
    let start0 = Instant::now();

    let input_file: &str = if !real {
        "./input/day16_21_test.txt"
    } else {
        "./input/day16_21_real.txt"
    };
    let input = tools::get_input(String::from(input_file));
    let message = input[0]
        .chars()
        .map(|c| c.to_digit(16).unwrap() )
        .map(|n| {
            (0..4)
                .fold( vec![], |mut v, bit| { 
                    v.push(((n >> bit) & 1) == 1); v 
                })
                .iter()
                .rev()
                .map(|&b|b)
                .collect()
        })
        .collect::<Vec<Vec<bool>>>()
        .iter()
        .flatten()
        .map(|&b|b)
        .collect::<Vec<bool>>();

    let after0 = Instant::now();

    let start1 = Instant::now();

    let mut pos = 0;
    let (res1, res2) = parse_packet(&message.clone(), &mut pos);

    let after1 = Instant::now();
    if print_result {
        println!("Part 1: {}", res1);
    }

    let start2 = Instant::now();

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

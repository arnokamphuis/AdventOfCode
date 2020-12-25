use super::tools;
use std::time::Instant;

fn play_game(numbers: &Vec<usize>, runs: usize, take: usize, part: usize) -> Vec<usize> {
    
    let collect = |lnks: &Vec<usize>, t: usize| -> Vec<usize> {
        let mut result = vec![lnks[1]];
        let mut taken = 1;
        while taken < t {
            result.push(lnks[result[result.len() - 1]]);
            taken += 1;
        }    
        result.clone()
    };


    let get_dest = | picked: &Vec<usize>, start: usize, len: usize | -> usize {
        let mut dest = start-1;
        if dest == 0 { dest = len; }
        while picked.contains(&dest) {
            dest -= 1;
            if dest == 0 { dest = len; }
        }
        dest
    };

    // Build a linked list, links array points gives the number directly to the right of the number given
    let llen = numbers.len()+1;
    let mut links = (0..llen).enumerate().map(|(index,_)| index+1).collect::<Vec<usize>>();
    links[llen-1] = numbers[0];
    numbers.iter().enumerate().for_each(|(index, &num)| {
        links[num] = 
            if index < (numbers.len()-1) { 
                numbers[index + 1]
            } else if numbers.len() < (links.len()-1) {
                numbers.len() + 1
            } else {
                numbers[0]
            };
    });

    let len = llen-1;
    let mut picked = vec![0; 3];

    let mut i = 0;
    let mut current_number = numbers[i];

    while i < runs {
        // take the three number to the right of start
        //   every links lookup find the number to the right
        picked[0] = links[current_number];
        picked[1] = links[picked[0]];
        picked[2] = links[picked[1]];

        // set the link of the current number to the number after the last picked
        links[current_number] = links[picked[2]];

        // update the destination according to the given rules
        let dest = get_dest(&picked, current_number, len);
        // get the number to which the destination if pointing
        let end = links[dest];
        // point the destination to the first picked number
        links[dest] = picked[0];
        // point the last picked number to where the destination was pointing
        links[picked[2]] = end;

        // update the current number to the next number
        current_number = links[current_number];

        i += 1;
    }

    if part == 1 {
        // for part 1 collect all numbers after 1
        collect(&links, take)
    } else {
        // for part to get the two numbers after 1
        let first = links[1];
        let second = links[first];
        vec![first, second]
    }
}

#[allow(dead_code)]
pub fn run(real: bool, print_result: bool) -> (u128, u128, u128) {
    let start0 = Instant::now();

    let input_file: &str = if !real {
        "./input/day23_20_test.txt"
    } else {
        "./input/day23_20_real.txt"
    };
    let input = tools::get_input(String::from(input_file));

    let numbers1 = input[0].chars().map(|c| c.to_digit(10).unwrap() as usize).collect::<Vec<usize>>();

    let mut numbers2 = numbers1.clone();
    (10..1_000_001).for_each(|v| numbers2.push(v));

    let after0 = Instant::now();

    let start1 = Instant::now();

    let result1 = play_game(&numbers1, 100, numbers1.len(), 1);
    let res1 = result1.iter().take(numbers1.len()-1).map(|v| v.to_string()).collect::<String>();

    let after1 = Instant::now();
    if print_result {
        println!("Part 1: {}", res1);
    }

    let start2 = Instant::now();

    let result2 = play_game(&numbers2, 10_000_000, numbers2.len(), 2);
    let res2 = result2[0] * result2[1];

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

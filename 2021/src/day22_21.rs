use super::tools;
use std::time::Instant;
use std::collections::HashMap;

#[allow(dead_code)]
pub fn run(real: bool, print_result: bool) -> (u128, u128, u128) {
    let start0 = Instant::now();

    let input_file: &str = if !real {
        "./input/day22_21_test.txt"
    } else {
        "./input/day22_21_real.txt"
    };
    let input = tools::get_input(String::from(input_file));

    let mut actions: Vec<(Vec<[i64;2]>,bool)> = vec![];
    let mut field: HashMap<[i64;3],bool> = HashMap::new();

    input.iter().for_each(|line| {
        let mut line_tokens = line.split(" ");
        let onoff = line_tokens.next().unwrap() == "on";
        line_tokens = line_tokens.next().unwrap().split(",");
        let mut coor_tokens = line_tokens.next().unwrap()[2..].split("..");
        let x_range = [
            coor_tokens.next().unwrap().parse::<i64>().unwrap(), 
            coor_tokens.next().unwrap().parse::<i64>().unwrap()];
        let mut coor_tokens = line_tokens.next().unwrap()[2..].split("..");
        let y_range = [
            coor_tokens.next().unwrap().parse::<i64>().unwrap(), 
            coor_tokens.next().unwrap().parse::<i64>().unwrap()];
            let mut coor_tokens = line_tokens.next().unwrap()[2..].split("..");
        let z_range = [
            coor_tokens.next().unwrap().parse::<i64>().unwrap(), 
            coor_tokens.next().unwrap().parse::<i64>().unwrap()];
        actions.push((vec![x_range, y_range, z_range], onoff));
    });
    // println!("{:?}",actions);

    let after0 = Instant::now();

    let start1 = Instant::now();

    actions.iter().for_each(|action| {
        (action.0[0][0].max(-50)..=action.0[0][1].min(50)).for_each(|x| {
            (action.0[1][0].max(-50)..=action.0[1][1].min(50)).for_each(|y| {
                (action.0[2][0].max(-50)..=action.0[2][1].min(50)).for_each(|z| {
                    *field.entry([x,y,z]).or_insert(false) = action.1;                                
                });
            });
        });
    });

    let res1 = field.values().fold(0, |count, b| count + *b as i64);

    let after1 = Instant::now();
    if print_result {
        println!("Part 1: {}", res1);
    }

    let start2 = Instant::now();

    let x_coords: Vec<i64> = actions.iter().map(|action| vec![action.0[0][0],action.0[0][1]]).collect::<Vec<Vec<i64>>>().iter().flatten().map(|v|*v).collect();
    let y_coords: Vec<i64> = actions.iter().map(|action| vec![action.0[1][0],action.0[1][1]]).collect::<Vec<Vec<i64>>>().iter().flatten().map(|v|*v).collect();
    let z_coords: Vec<i64> = actions.iter().map(|action| vec![action.0[2][0],action.0[2][1]]).collect::<Vec<Vec<i64>>>().iter().flatten().map(|v|*v).collect();

    x_coords.sort();
    y_coords.sort();
    z_coords.sort();

    

    println!("{:?}", x_coords);

    let after2 = Instant::now();
    if print_result {
        println!("Part 2: {}", 0);
    }

    (
        after0.duration_since(start0).as_nanos(),
        after1.duration_since(start1).as_nanos(),
        after2.duration_since(start2).as_nanos(),
    )
}

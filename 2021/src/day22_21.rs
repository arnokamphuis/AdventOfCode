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

    let mut x_coords: Vec<i64> = actions.iter().map(|action| vec![action.0[0][0],action.0[0][1]+1]).collect::<Vec<Vec<i64>>>().iter().flatten().map(|v|*v).collect();
    let mut y_coords: Vec<i64> = actions.iter().map(|action| vec![action.0[1][0],action.0[1][1]+1]).collect::<Vec<Vec<i64>>>().iter().flatten().map(|v|*v).collect();
    let mut z_coords: Vec<i64> = actions.iter().map(|action| vec![action.0[2][0],action.0[2][1]+1]).collect::<Vec<Vec<i64>>>().iter().flatten().map(|v|*v).collect();

    x_coords.sort();
    y_coords.sort();
    z_coords.sort();

    let mut index = 0;
    loop { 
        if x_coords[index+1] == x_coords[index] { x_coords.remove(index); } else { index += 1; }
        if index+1 >= x_coords.len() { break; }
    }
    index = 0;
    loop { 
        if y_coords[index+1] == y_coords[index] { y_coords.remove(index); } else { index += 1; }
        if index+1 >= y_coords.len() { break; }
    }
    index = 0;
    loop { 
        if z_coords[index+1] == z_coords[index] { z_coords.remove(index); } else { index += 1; }
        if index+1 >= z_coords.len() { break; }
    }

    let mut x_compressed: HashMap<i64,i64> = HashMap::new();
    let mut x_sizes: HashMap<i64,i64> = HashMap::new();
    for i in 0..x_coords.len()-1 {
        if x_coords[i+1] - x_coords[i] > 0 {
            x_compressed.insert(x_coords[i], x_sizes.len() as i64);
            x_sizes.insert(x_sizes.len() as i64, x_coords[i+1] - x_coords[i]);
        }
    }
    x_compressed.insert(x_coords[x_coords.len()-1], x_sizes.len() as i64);
    x_sizes.insert(x_sizes.len() as i64,0);

    let mut y_compressed: HashMap<i64,i64> = HashMap::new();
    let mut y_sizes: HashMap<i64,i64> = HashMap::new();
    for i in 0..y_coords.len()-1 {
        if y_coords[i+1] - y_coords[i] > 0 {
            y_compressed.insert(y_coords[i], y_sizes.len() as i64);
            y_sizes.insert(y_sizes.len() as i64, y_coords[i+1] - y_coords[i]);
        }
    }
    y_compressed.insert(y_coords[y_coords.len()-1], y_sizes.len() as i64);
    y_sizes.insert(y_sizes.len() as i64,0);

    let mut z_compressed: HashMap<i64,i64> = HashMap::new();
    let mut z_sizes: HashMap<i64,i64> = HashMap::new();
    for i in 0..z_coords.len()-1 {
        if z_coords[i+1] - z_coords[i] > 0 {
            z_compressed.insert(z_coords[i], z_sizes.len() as i64);
            z_sizes.insert(z_sizes.len() as i64, z_coords[i+1] - z_coords[i]);
        }
    }
    z_compressed.insert(z_coords[z_coords.len()-1], z_sizes.len() as i64);
    z_sizes.insert(z_sizes.len() as i64,0);

    let mut field2: HashMap<[i64;3],bool> = HashMap::new();
    actions.iter().for_each(|action| {
        (x_compressed[&action.0[0][0]]..x_compressed[&(action.0[0][1]+1)]).for_each(|x| {
            (y_compressed[&action.0[1][0]]..y_compressed[&(action.0[1][1]+1)]).for_each(|y| {
                (z_compressed[&action.0[2][0]]..z_compressed[&(action.0[2][1]+1)]).for_each(|z| {
                    if action.1 {
                        *field2.entry([x,y,z]).or_insert(false) = true;                                
                    } else {
                        field2.remove(&[x,y,z]);
                    }
                });
            });
        });
    });
    let res2 = field2.iter().fold(0, |count, (pos, _)| {
        count + x_sizes[&pos[0]] * y_sizes[&pos[1]] * z_sizes[&pos[2]]
    });    

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

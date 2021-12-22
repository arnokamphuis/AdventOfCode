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

    let mut coords: Vec<Vec<i64>> = (0..3).fold(vec![], |mut sc, i| {
        sc.push(actions.iter().map(|action| vec![action.0[i][0],action.0[i][1]+1]).collect::<Vec<Vec<i64>>>().iter().flatten().map(|v|*v).collect::<Vec<i64>>()); sc
    });

    // let mut x_coords: Vec<i64> = actions.iter().map(|action| vec![action.0[0][0],action.0[0][1]+1]).collect::<Vec<Vec<i64>>>().iter().flatten().map(|v|*v).collect();
    // let mut y_coords: Vec<i64> = actions.iter().map(|action| vec![action.0[1][0],action.0[1][1]+1]).collect::<Vec<Vec<i64>>>().iter().flatten().map(|v|*v).collect();
    // let mut z_coords: Vec<i64> = actions.iter().map(|action| vec![action.0[2][0],action.0[2][1]+1]).collect::<Vec<Vec<i64>>>().iter().flatten().map(|v|*v).collect();

    let mut compressed: Vec<HashMap<i64,i64>> = vec![HashMap::new();3];
    let mut sizes: Vec<HashMap<i64,i64>> = vec![HashMap::new();3];
    (0..3).for_each(|i| { 
        coords[i].sort();
        let mut index = 0;
        loop { 
            if coords[i][index+1] == coords[i][index] { coords[i].remove(index); } else { index += 1; }
            if index+1 >= coords[i].len() { break; }
        }

        for index in 0..coords[i].len()-1 {
            if coords[i][index+1] - coords[i][index] > 0 {
                let ni = sizes[i].len() as i64;
                compressed[i].insert(coords[i][index], ni);
                sizes[i].insert(ni, coords[i][index+1] - coords[i][index]);
            }
        }
        let ni = sizes[i].len() as i64;
        compressed[i].insert(coords[i][coords[i].len()-1], ni);
        sizes[i].insert(ni,0);
    });

    let mut field2: HashMap<[i64;3],bool> = HashMap::new();
    actions.iter().for_each(|action| {
        (compressed[0][&action.0[0][0]]..compressed[0][&(action.0[0][1]+1)]).for_each(|x| {
            (compressed[1][&action.0[1][0]]..compressed[1][&(action.0[1][1]+1)]).for_each(|y| {
                (compressed[2][&action.0[2][0]]..compressed[2][&(action.0[2][1]+1)]).for_each(|z| {
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
        count + sizes[0][&pos[0]] * sizes[1][&pos[1]] * sizes[2][&pos[2]]
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

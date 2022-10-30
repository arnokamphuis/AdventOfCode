// use super::tools;
use std::time::Instant;

#[allow(dead_code)]
pub fn run(_real: bool, print_result: bool) -> (u128, u128, u128) {
    let start0 = Instant::now();

    // let input_file: &str = if !real {
    //     "./input/dayxx_15_test.txt"
    // } else {
    //     "./input/dayxx_15_real.txt"
    // };
    // let input = tools::get_input(String::from(input_file));

    let cst_weapon = vec![8,10,25,40,74];
    let val_weapon = vec![4,5,6,7,8];
    
    let cst_armor  = vec![0,13,31,53,75,102];
    let val_armor  = vec![0,1,2,3,4,5];

    let cst_ring1  = vec![0,25,50,100,20,40,80];
    let val_ring1  = vec![0,1,2,3,1,2,3];
    
    let cst_ring2  = vec![0,25,50,100,20,40,80];
    let val_ring2  = vec![0,1,2,3,1,2,3];
    
    let mut purchases: Vec<Vec<usize>> = vec![];
    for w in 0..val_weapon.len() {
        for a in 0..val_armor.len() {
            for r1 in 0..val_ring1.len() {
                for r2 in 0..val_ring2.len() {
                    purchases.push(vec![w,a,r1,r2]);
                }
            }
        }
    }

    let after0 = Instant::now();

    let start1 = Instant::now();

    let mut stats = vec![0i32, 0];
    let res1 = purchases.iter()
        .filter(|p| p[2] != p[3])
        .filter(|p| {
            stats = vec![ val_weapon[p[0]], val_armor[p[1]] ];
            stats[(p[2]<4) as usize] += val_ring1[p[2]];
            stats[(p[3]<4) as usize] += val_ring2[p[3]];
            (stats[0] + stats[1]) >= 10
        }) // 8+2
        .map(|a| cst_weapon[a[0]] + cst_armor[a[1]] + cst_ring1[a[2]] + cst_ring2[a[3]])
        .min().unwrap();

    let after1 = Instant::now();
    if print_result {
        println!("Part 1: {}", res1);
    }

    let start2 = Instant::now();

    let res2 = purchases.iter()
        .filter(|p| p[2] != p[3])
        .filter(|p| {
            stats = vec![ val_weapon[p[0]], val_armor[p[1]] ];
            stats[(p[2]<4) as usize] += val_ring1[p[2]];
            stats[(p[3]<4) as usize] += val_ring2[p[3]];
            (stats[0] + stats[1]) < 10
        }) // 8+2
        .map(|a| cst_weapon[a[0]] + cst_armor[a[1]] + cst_ring1[a[2]] + cst_ring2[a[3]])
        .max().unwrap();

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

use super::tools;
use std::time::Instant;
use std::collections::BTreeSet;
use std::collections::HashMap;
// use permutohedron::heap_recursive;


fn orient(mat: [[i32;3];3], p: [i32;3]) -> [i32;3] {
    let mut res = [0;3];
    (0..3).for_each(|i| {
        (0..3).for_each(|j| {
            res[i] += mat[i][j] * p[j];
        });
    });
    res
}

fn back_orient(mat: [[i32;3];3], p: [i32;3]) -> [i32;3] {
    let mut res = [0;3];
    (0..3).for_each(|i| {
        (0..3).for_each(|j| {
            res[i] += mat[j][i] * p[j];
        });
    });
    res
}

#[derive(Debug, Clone)]
struct Scanner {
    scans: Vec<[i32;3]>,
    orientation: [[i32;3];3],
    displacement: [i32;3],
    beacons: Vec<[i32;3]>,
}

impl Scanner {
    fn new() -> Scanner {
        Scanner{
            scans: vec![],
            orientation: [[1,0,0],[0,1,0],[0,0,1]],
            displacement: [0,0,0],
            beacons: vec![],

        }
    }

    pub fn add_scan(&mut self, line: &String) {
        self.scans.push( line.split(",").enumerate().map(|(i,v)| (i,v.parse::<i32>().unwrap())).fold([0;3], |mut arr, (i,v)| {arr[i] = v; arr}) );
    }

    pub fn overlapping(&self, other: &mut Scanner, orientation: [[i32;3];3]) -> bool {
        let other_scans: Vec<[i32;3]> = other.scans.iter().fold(vec![], |mut set, &scan| {
            set.push( orient(orientation, scan) ); set
        });

        let mut diff_counter: HashMap<[i32;3],usize> = HashMap::new();

        self.scans.iter().for_each(|scan1| {
            other_scans.iter().for_each(|scan2| {
                let diff: [i32;3] = [ scan1[0]-scan2[0], scan1[1]-scan2[1], scan1[2]-scan2[2] ];
                *diff_counter.entry(diff).or_insert(0) += 1;
            });
        });

        diff_counter.retain(|_,s| *s >= 12);
        if diff_counter.len() > 0 {
            let displacement = diff_counter.iter().map(|(d,_)| *d).collect::<Vec<[i32;3]>>()[0];
            // println!("displacement {:?}", displacement);

            self.scans.iter().for_each(|scan1| {
                other_scans.iter().for_each(|scan2| {
                    let diff: [i32;3] = [ scan1[0]-scan2[0], scan1[1]-scan2[1], scan1[2]-scan2[2] ];
                    if diff[0] == displacement[0] && diff[1] == displacement[1] && diff[2] == displacement[2] {
                        // println!("{} - {:?}  ###### {} - {:?}",self.id, scan1, other.id, scan2);
                        other.beacons.push( back_orient(orientation, *scan2));
                    }
                });
            });
    

            other.orientation = orientation;
            other.displacement = displacement;
        }
        diff_counter.len() > 0
    }
}

#[allow(dead_code)]
pub fn run(real: bool, print_result: bool) -> (u128, u128, u128) {
    let start0 = Instant::now();

    let input_file: &str = if !real {
        "./input/day19_21_test.txt"
    } else {
        "./input/day19_21_real.txt"
    };
    let input = tools::get_input(String::from(input_file));

    let mut id = 0;
    let mut scanners: HashMap<usize, Scanner> = HashMap::new();
    input
        .iter()
        .for_each(|line| {
            if line.starts_with("---") {
                id = line[12..14].chars().filter(|&c| c!=' ').collect::<String>().parse::<usize>().unwrap();
            } else if line.len() > 0 {
                scanners.entry(id).or_insert(Scanner::new()).add_scan(line);
            }
        });

    // println!("scanner count: {:?}", scanners);

    let mult = | m1: &[[i32;3];3], m2: &[[i32;3];3] | -> [[i32;3];3] {
        let mut res = [[0;3];3];

        (0..3).for_each(|i| {
            (0..3).for_each(|j| {
                (0..3).for_each(|k| {
                    res[i][j] += m1[i][k] * m2[k][j];
                });
            });
        });

        res
    };

    // let mut indices = [0, 1, 2];
    // let mut permutations = Vec::new();
    // heap_recursive(&mut indices, |permutation| {
    //     permutations.push(permutation.to_vec())
    // });


    let mut orientations: BTreeSet<[[i32;3];3]> = BTreeSet::new();

    // let identity = [
    //     [1,0,0],
    //     [0,1,0],
    //     [0,0,1]
    // ];
    // (0..48).for_each(|d| {
    //     let perm = &permutations[d/8];
    //     let mut mat: [[i32;3];3] = [
    //         [ identity[0][perm[0]], identity[0][perm[1]], identity[0][perm[2]] ],
    //         [ identity[1][perm[0]], identity[1][perm[1]], identity[1][perm[2]] ],
    //         [ identity[2][perm[0]], identity[2][perm[1]], identity[2][perm[2]] ],
    //     ];

    //     if d     % 2 == 1 { mat[0][0] *= -1; mat[1][0] *= -1; mat[2][0] *= -1; }
    //     if (d/2) % 2 == 1 { mat[0][1] *= -1; mat[1][1] *= -1; mat[2][1] *= -1; }
    //     if (d/4) % 2 == 1 { mat[0][2] *= -1; mat[1][2] *= -1; mat[2][2] *= -1; }
    //     orientations.insert(mat);
    // });

    let basis_a = vec![
        [[1, 0, 0], [0, 1, 0], [0, 0, 1]],
        [[0, 1, 0], [0, 0, 1], [1, 0, 0]],
        [[0, 0, 1], [1, 0, 0], [0, 1, 0]],
        ];    

    let basis_b = vec![
        [[ 1, 0, 0], [ 0, 1, 0], [ 0, 0, 1]],
        [[-1, 0, 0], [ 0,-1, 0], [ 0, 0, 1]],
        [[-1, 0, 0], [ 0, 1, 0], [ 0, 0,-1]],
        [[ 1, 0, 0], [ 0,-1, 0], [ 0, 0,-1]]];
    
    let basis_c = vec![
        [[ 1, 0, 0], [ 0, 1, 0], [ 0, 0, 1]],
        [[ 0, 0,-1], [ 0,-1, 0], [-1, 0, 0]]];


    for a in &basis_a {
        for b in &basis_b {
            for c in &basis_c {
                orientations.insert(mult(&mult(a,b),c));
            }
        }
    }

    // for orient in &orientations {
    //     println!("{:?}", orient);
    // }


    let after0 = Instant::now();

    let start1 = Instant::now();

    let mut known_scanners: BTreeSet<usize> = BTreeSet::new();
    let mut known_overlap: BTreeSet<(usize,usize)> = BTreeSet::new();

    known_scanners.insert(0);

    while known_scanners.len() < scanners.len() {
        // println!("---------------------------------------------------------------");
        for &ks in &known_scanners {
            let known = scanners[&ks].clone();
            for from_scanner in 0..scanners.len() {
                if !(known_overlap.contains(&(ks, from_scanner)) || known_overlap.contains(&(from_scanner, ks))) && ks != from_scanner {
                    for orient in &orientations {
                        if known.overlapping( scanners.get_mut(&from_scanner).unwrap(), *orient ) {
                            known_overlap.insert((ks,from_scanner));
                            // println!("{} {} overlapping", ks, from_scanner);
                        }
                    }
                }
            }
        }
        known_overlap.iter().for_each(|(i,j)| {
            known_scanners.insert(*i); 
            known_scanners.insert(*j); 
        });
    }

    let mut beacons: BTreeSet<[i32;3]> = BTreeSet::new();
    scanners[&0].scans.iter().for_each(|scan| { beacons.insert(*scan); } );

    let mut scanner_positions: Vec<[i32;3]> = vec![];

    (1..scanners.len()).for_each(|scanner_id| {
        // println!("{} ============================================================", scanner_id);
        let mut to = scanner_id;
        let mut from = known_overlap.iter().filter(|(_,t)| *t == to).map(|(f,_)| *f).collect::<Vec<usize>>()[0];

        let mut from_beacons: BTreeSet<[i32;3]> = BTreeSet::new();
        let mat = scanners[&to].orientation;
        let dis = scanners[&to].displacement;

        let mut scanner_pos = dis;

        scanners[&scanner_id].scans.iter().for_each(|beacon| {
            let mut new_pos = orient(mat, *beacon);
            (0..3).for_each(|i| { new_pos[i] += dis[i]; });
            // println!("{:?}     .... converting..... {:?}", beacon, new_pos);
            from_beacons.insert(new_pos);
        });

        while from != 0 {
            to = from;
            from = known_overlap.iter().filter(|(_,t)| *t == to).map(|(f,_)| *f).collect::<Vec<usize>>()[0];
            // println!("from {} to {}", from, to);
            let mut next_beacons: BTreeSet<[i32;3]> = BTreeSet::new();
            let mat = scanners[&to].orientation;
            let dis = scanners[&to].displacement;

            scanner_pos = orient(mat, scanner_pos);
            (0..3).for_each(|i| { scanner_pos[i] += dis[i]; });
            if from == 0 {
                // println!("scanner {} pos {:?} ", scanner_id, scanner_pos);
                scanner_positions.push(scanner_pos);
            }
    
            from_beacons.iter().for_each(|beacon| {
                let mut new_pos = orient(mat, *beacon);
                (0..3).for_each(|i| { new_pos[i] += dis[i]; });
                // println!("{:?}     .... converting..... {:?}", beacon, new_pos);
                next_beacons.insert(new_pos);
            });
            from_beacons = next_beacons;
        }
        // // if scanner_id == 4 {
        //     from_beacons.iter().for_each(|beacon| {
        //         println!("beacon: {:?}", beacon);
        //     });
        // // }
        beacons = beacons.union(&from_beacons).cloned().collect();
        // println!("beacons: {}", beacons.len());
    });

    // beacons.iter().for_each(|beacon| {
    //     println!("beacon: {:?}", beacon);
    // });

    let res1 = beacons.len();
    // println!("beacons: {}", beacons.len());
    // scanners.iter().for_each(|(id,s)| {
    //     println!("{} -> {:?}   + {:?}", s.id, s.orientation, s.displacement);
    // });
    // println!("{:?}", known_overlap);    

    let after1 = Instant::now();
    if print_result {
        println!("Part 1: {}", res1);
    }

    let start2 = Instant::now();

    let mut max_dist: i32 = 0;
    scanner_positions.iter().for_each(|sp1| {
        scanner_positions.iter().for_each(|sp2| {
            let dist = (sp1[0]-sp2[0]).abs() + (sp1[1]-sp2[1]).abs() + (sp1[2]-sp2[2]).abs();
            max_dist = max_dist.max(dist);
        });    
    });

    let after2 = Instant::now();
    if print_result {
        println!("Part 2: {}", max_dist);
    }

    (
        after0.duration_since(start0).as_nanos(),
        after1.duration_since(start1).as_nanos(),
        after2.duration_since(start2).as_nanos(),
    )
}

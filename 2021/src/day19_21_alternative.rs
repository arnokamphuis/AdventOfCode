use super::tools;
use std::time::Instant;
use std::collections::BTreeSet;
use std::collections::HashMap;

fn orient(mat: [[i32;3];3], p: [i32;3]) -> [i32;3] {
    let mut res = [0;3];
    (0..3).for_each(|i| {
        (0..3).for_each(|j| {
            res[i] += mat[i][j] * p[j];
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

    pub fn overlapping(&mut self, other: &mut Scanner, orientation: [[i32;3];3]) -> bool {
        let other_scans: Vec<[i32;3]> = other.scans.iter().fold(vec![], |mut set, &scan| {
            set.push( orient(orientation, scan) ); set
        });

        let mut diff_counter: HashMap<[i32;3],usize> = HashMap::new();

        self.beacons.iter().for_each(|scan1| {
            other_scans.iter().for_each(|scan2| {
                let diff: [i32;3] = [ scan1[0]-scan2[0], scan1[1]-scan2[1], scan1[2]-scan2[2] ];
                *diff_counter.entry(diff).or_insert(0) += 1;
            });
        });

        diff_counter.retain(|_,s| *s >= 12);
        if diff_counter.len() > 0 {
            let displacement = diff_counter.iter().map(|(d,_)| *d).collect::<Vec<[i32;3]>>()[0];
            other_scans.iter().for_each(|scan2| {
                let mut new_pos = *scan2;
                (0..3).for_each(|i| { new_pos[i] += displacement[i]; });
                self.beacons.push( new_pos );
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

    let mut orientations: BTreeSet<[[i32;3];3]> = BTreeSet::new();

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

    let after0 = Instant::now();

    let start1 = Instant::now();

    let mut known_scanners: BTreeSet<usize> = BTreeSet::new();

    known_scanners.insert(0);
    let mut base = scanners.get(&0).unwrap().clone();
    base.beacons = base.scans.clone();
    
    while known_scanners.len() < scanners.len() {
        for from_scanner in 1..scanners.len() {
            if !known_scanners.contains(&from_scanner) {
                for orient in &orientations {
                    if base.overlapping( scanners.get_mut(&from_scanner).unwrap(), *orient ) {
                        known_scanners.insert(from_scanner);
                    }
                }
            }
        }
    }

    let mut beacons: BTreeSet<[i32;3]> = BTreeSet::new();
    base.beacons.iter().for_each(|scan| { beacons.insert(*scan); } );

    let res1 = beacons.len();

    let after1 = Instant::now();
    if print_result {
        println!("Part 1: {}", res1);
    }

    let start2 = Instant::now();

    scanners.entry(0).and_modify(|e| { *e = base; });
    let scanner_positions = scanners.iter().map(|(_, scanner)| scanner.displacement ).collect::<Vec<[i32;3]>>();

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

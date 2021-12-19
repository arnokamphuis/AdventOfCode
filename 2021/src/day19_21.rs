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

#[derive(Debug)]
struct Scanner {
    id: usize,
    scans: Vec<[i32;3]>,
    orientation: [[i32;3];3],
}

impl Scanner {
    fn new(i: usize) -> Scanner {
        Scanner{
            id: i,
            scans: vec![],
            orientation: [[1,0,0],[0,1,0],[0,0,1]],
        }
    }

    pub fn add_scan(&mut self, line: &String) {
        self.scans.push( line.split(",").map(|v| v.parse::<i32>().unwrap()).collect::<Vec<i32>>().to_arr() );
    }

    pub fn overlapping(&self, other: &Scanner, orientation: [[i32;3];3]) -> bool {
        let self_scans: BTreeSet<(i32,i32,i32)> = self.scans.iter().fold(BTreeSet::new(), |mut set, scan| {set.insert(scan); set });
        let other_scans: BTreeSet<(i32,i32,i32)> = other.scans.iter().fold(BTreeSet::new(), |mut set, scan| {
            set.insert( orient(orientation, scan) ); set
        });

        let inter = self_scans.intersection(&other_scans).cloned.collect::<BTreeSet<[i32;3]>>();
        println!("insection size {}", inter.len());
        false
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
                scanners.entry(id).or_insert(Scanner::new(id)).add_scan(line);
            }
        });

    println!("scanner count: {:?}", scanners);

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

    let basis_a = vec![
        [[1, 0, 0], [0, 1, 0], [0, 0, 1]],
        [[0, 1, 0], [0, 0, 1], [1, 0, 0]],
        [[0, 0, 1], [1, 0, 0], [0, 1, 0]]];    

    let basis_b = vec![
        [[ 1, 0, 0], [ 0, 1, 0], [ 0, 0, 1]],
        [[-1, 0, 0], [ 0,-1, 0], [ 0, 0, 1]],
        [[-1, 0, 0], [ 0, 1, 0], [ 0, 0,-1]],
        [[ 1, 0, 0], [ 0,-1, 0], [ 0, 0,-1]]];
    
    let basis_c = vec![
        [[ 1, 0, 0], [ 0, 1, 0], [ 0, 0, 1]],
        [[ 0, 0,-1], [ 0,-1, 0], [-1, 0, 0]]];


    let mut orientations: BTreeSet<[[i32;3];3]> = BTreeSet::new();
    for a in &basis_a {
        for b in &basis_b {
            for c in &basis_c {
                orientations.insert(mult(&mult(a,b),c));
            }
        }
    }


    let mut known_scanners: Vec<usize> = vec![0];
    for &ks in &known_scanners {
        let known = scanners[ks];
        for from_scanner in 0..scanners.len() {
            if ks != from_scanner {
                for orient in &orientations {
                    if known.overlapping( scanners[from_scanner], orient ) {
                        println!("{} {} overlapping", ks, from_scanner);
                    }
                }
            }
        }
    }
    
    println!("{:?}", orientations.len());    
    let after0 = Instant::now();

    let start1 = Instant::now();

    let after1 = Instant::now();
    if print_result {
        println!("Part 1: {}", 0);
    }

    let start2 = Instant::now();

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

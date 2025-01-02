use super::tools;
use std::time::Instant;
use std::collections::{HashSet, HashMap};
use itertools::Itertools;

fn solve(res: i64, nums:Vec<i64>, concat: bool, multi_prods: &HashMap<(usize,usize),Vec<Vec<usize>>>) -> bool {

    let mut ops = vec!["+", "*"];
    if concat {
        ops.push("||");
    }

    let n = nums.len()-1;
    let m = ops.len();

    let multi_prod = multi_prods.get(&(n,m)).unwrap();

    for op in multi_prod.iter() {
        let mut t = nums[0];
        for i in 0..n {
            match ops[op[i]] {
                "+" => t += nums[i+1],
                "*" => t *= nums[i+1],
                "||" => {
                    t = t * 10_i64.pow(nums[i+1].ilog10() + 1) + nums[i+1];
                }
                _ => panic!("Unknown operator"),
            }
        }
        if t == res {
            return true;
        }
    }

    return false;
}

#[allow(dead_code)]
pub fn run(real: bool, print_result: bool) -> (u128, u128, u128) {
    let start0 = Instant::now();

    let input_file: &str = if !real {
        "./input/day07-test.txt"
    } else {
        "./input/day07-real.txt"
    };
    let input = tools::get_input(String::from(input_file));

    let mut max_n = 0;
    let mut min_n = 0;
    let mut equations: HashSet<(Vec<i64>, i64)> = HashSet::new();
    input
        .iter()
        .for_each(|line| {
            let parts = line.split(": ").collect::<Vec<&str>>();
            let result = parts[0].parse::<i64>().unwrap();
            let eqstr = parts[1].split_whitespace().collect::<Vec<&str>>();
            let eq = eqstr.iter().map(|x| x.parse::<i64>().unwrap()).collect::<Vec<i64>>();
            max_n = max_n.max(eq.len());
            min_n = min_n.min(eq.len());
            equations.insert((eq, result));
        });

    let mut multi_prods: HashMap<(usize,usize),Vec<Vec<usize>>> = HashMap::new();
    for m in 2..4 {
        for n in min_n..max_n {
            multi_prods.insert((n,m),(0..n).map(|_| 0..m).multi_cartesian_product().collect::<Vec<_>>());
        }
    }

    let after0 = Instant::now();

    let start1 = Instant::now();

    let mut res1 = 0;
    for (eq, res) in equations.iter() {
        if solve(*res, eq.clone(), false, &multi_prods) {
            res1 += res;
        }
    }

    let after1 = Instant::now();
    if print_result {
        println!("Part 1: {}", res1);
    }

    let start2 = Instant::now();

    let mut res2 = 0;
    for (eq, res) in equations.iter() {
        if solve(*res, eq.clone(), true, &multi_prods) {
            res2 += res;
        }
    }

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

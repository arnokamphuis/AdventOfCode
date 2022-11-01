use std::time::Instant;

#[allow(dead_code)]
pub fn run(_real: bool, print_result: bool) -> (u128, u128, u128) {
    let start0 = Instant::now();

    let column = 3029;
    let row = 2947;

    let after0 = Instant::now();

    let start1 = Instant::now();

    let mut number = 20151125u64;

    let factor = 252533u64;
    let division = 33554393u64;

    let target: u64 = (1..=column).fold(0, | sum, v | sum + v) as u64 + (1..row).fold(0, | sum, v | sum + column-1 + v) as u64;

    (1..target).for_each(|_| {
        number = (number * factor) % division;
    });

    let after1 = Instant::now();
    if print_result {
        println!("Part 1: {}", number);
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

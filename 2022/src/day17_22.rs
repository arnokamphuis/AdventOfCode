use super::tools;
use std::time::Instant;
use std::collections::HashMap;

#[derive(Clone)]
struct Cave {
    rocks: Vec<Vec<Vec<u8>>>,
    field: Vec<Vec<u8>>,
    jets: Vec<i16>,
    top: usize,
    left: usize,
    current_jet_index: usize,
    rock_counter: usize,
    added_height_in_cycles: usize,
}

impl Cave {
    fn new(rs: &Vec<Vec<Vec<u8>>>, f: &Vec<Vec<u8>>, j: &Vec<i16>) -> Cave {
        Cave {
            rocks: rs.clone(),
            field: f.clone(),
            jets: j.clone(),
            top: f.len()-1,
            left: 2,
            current_jet_index: j.len()-1,
            rock_counter: 0,
            added_height_in_cycles: 0,
        }
    }

    fn current_rock(&self) -> &Vec<Vec<u8>> {
        &self.rocks[(self.rock_counter-1) % self.rocks.len()]
    }

    fn rock_width(&self) -> usize {
        self.rocks[(self.rock_counter-1) % self.rocks.len()][0].len()
    }

    fn current_jet(&self) -> i16 {
        self.jets[self.current_jet_index]
    }

    fn next_jet(&mut self) -> i16 {
        self.current_jet_index += 1;
        self.current_jet_index %= self.jets.len();
        self.current_jet()
    }

    fn next_rock(&mut self) -> &Vec<Vec<u8>> {
        self.rock_counter += 1;
        self.current_rock()
    }


    fn get_height(&self, total: bool) -> usize {
        self.field.iter().position(|row| {
            row.iter().all(|&x| x == 0)
        }).unwrap() - 1 + if total { self.added_height_in_cycles } else { 0 }
    }

    fn reset(&mut self) {
        let h = self.get_height(false) + 1;
        while h < self.field.len() {
            self.field.remove(h);
        }
        for _ in self.field.len()..h+3+self.current_rock().len() {
            self.field.push(vec![0,0,0,0,0,0,0]);
        }
        self.left = 2;
        self.top  = self.field.len() - self.current_rock().len(); 
    }

    fn collide(&self, delta_x: i64, delta_y: i64) -> bool {
        if (self.left + self.rock_width()) as i64 + delta_x > 7 {
            return true;
        }
        if (self.left as i64 + delta_x ) < 0 {
            return true;
        }

        let res = self.current_rock().iter().rev().enumerate().map(|(i,rr)| {
            {
                let y_index = ((self.top+i) as i64 + delta_y) as usize;                
                rr.iter().enumerate().all(|(j,&rc)| {
                        let x_index = ((self.left+j) as i64 + delta_x) as usize;
                        rc==0 || self.field[y_index][x_index]==0 
                    }
                )
            }
        }).all(|rr| rr);
        !res
    }

    fn solidify_rock(&mut self) {
        let rock = self.current_rock().clone();
        rock.iter().rev().enumerate().for_each(|(i,rr)| {
            rr.iter().enumerate().for_each(|(j,&rc)| {
                if rc == 1 {
                    self.field[self.top+i][self.left+j] = 1;
                }
            });
        });
    }

    fn step(&mut self, target: usize) -> usize {
        self.next_rock();
        self.reset();

        let mut previous: HashMap<(usize, usize, usize),(usize, usize)> = HashMap::new();

        while self.rock_counter <= target {
            let jet = self.next_jet();
            if !self.collide(jet as i64, 0) {
                self.left = (self.left as i16 + jet) as usize;
            }
            if !self.collide(0,-1) {
                self.top -= 1;
            } else {
                self.solidify_rock();

                if self.rock_counter > 2022 { // part 2
                    let history = self.field
                        .iter()
                        .last()
                        .unwrap()
                        .iter()
                        .enumerate()
                        .fold(0usize, |acc, (i,&v)| acc + v as usize * (1 << i) );

                    let height = self.get_height(false);
                    let key: (usize, usize, usize) = (self.current_jet_index as usize, self.rock_counter as usize % self.rocks.len(), history);

                    if let Some((prev_rock_counter, old_height)) = previous.get(&key) {
                        let delta_rocks = self.rock_counter - prev_rock_counter;
                        let delta_height = height - old_height;

                        let cycles = ((target - self.rock_counter) as f64 / delta_rocks as f64).floor() as usize;
                        self.added_height_in_cycles += cycles * delta_height;
                        self.rock_counter += cycles * delta_rocks;
                    }
                    previous.insert(key, (self.rock_counter, height));
                }
                self.next_rock();
                self.reset();
            }

        }
        self.get_height(true)
    }

    #[allow(dead_code)]
    fn print(&self) {
        let rock = self.current_rock().clone();
        let mut field = self.field.clone();
        rock.iter().rev().enumerate().for_each(|(i,rr)| {
            rr.iter().enumerate().for_each(|(j,&rc)| {
                if rc == 1 && self.top+i<self.field.len() && self.left+j < 7 {
                    field[self.top+i][self.left+j] = 2;
                }
            });
        });

        let skip = field.len() - self.get_height(false)-1;
        for (i,row) in field.iter().enumerate().rev().skip(skip) {
            print!("{} \t| ", i);
            row.iter().for_each(|v| print!("{}", match v {
                0 => ".",
                1 => "#",
                2 => "#",
                _ => " "
            } ));
            println!("");
        }
    }

}

#[allow(dead_code)]
pub fn run(real: bool, print_result: bool) -> (u128, u128, u128) {
    let start0 = Instant::now();

    let input_file: &str = if !real {
        "./input/day17_22_test.txt"
    } else {
        "./input/day17_22_real.txt"
    };
    let input = tools::get_input(String::from(input_file));

    let jets = input[0].chars().map(|c| match c {
        '>' =>  1i16,
        '<' => -1i16,
        _   =>  0i16
    }).collect::<Vec<i16>>();

    let mut cave = Cave::new(
        &vec![
            vec![vec![1,1,1,1]],
            vec![vec![0,1,0],vec![1,1,1],vec![0,1,0]],
            vec![vec![0,0,1],vec![0,0,1],vec![1,1,1]],
            vec![vec![1],vec![1],vec![1],vec![1]],
            vec![vec![1,1],vec![1,1]]
        ],
        &vec![
            vec![1,1,1,1,1,1,1],
            vec![0,0,0,0,0,0,0],
            vec![0,0,0,0,0,0,0],
            vec![0,0,0,0,0,0,0],        
        ],
        &jets
    );

    let mut cave2 = cave.clone();

    let after0 = Instant::now();

    let start1 = Instant::now();

    let res1 = cave.step(2022);
    // let res1 = cave.get_height(true);

    let after1 = Instant::now();
    if print_result {
        println!("Part 1: {}", res1);
    }

    let start2 = Instant::now();

    let res2 = cave2.step(1000000000000);
    // let res2 = cave2.get_height(true);

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

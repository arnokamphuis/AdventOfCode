use std::time::{Instant};
use super::tools;
use std::collections::BTreeSet;
use std::cmp::Ordering;

#[derive(Eq, Clone)]
struct Vector {
    x: i64,
    y: i64,
    z: i64,
}


impl Ord for Vector {
    fn cmp(&self, other: &Self) -> Ordering {
        let Ox = self.x.cmp(&other.x);
        if Ox == Ordering::Less {
            return Ox
        } else {
            let Oy = self.y.cmp(&other.y);
            if Oy == Ordering::Less {
                return Oy
            } else {
                self.z.cmp(&other.z)
            }
        }
    }
}

impl PartialOrd for Vector {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

impl PartialEq for Vector {
    fn eq(&self, other: &Self) -> bool {
        self.x == other.x && self.y == other.y && self.z == other.z
    }
}


impl Vector {
    fn new() -> Vector {
        Vector{x:0, y:0, z:0}
    }

    fn energy(&self) -> i64 {
        self.x.abs() + self.y.abs() + self.z.abs()
    }
}

struct MoonSystem {
    positions: Vec<Vector>,
    velocities: Vec<Vector>,
    prev_state: Vec<BTreeSet<(Vector, Vector)>>,
}

impl MoonSystem {
    fn new() -> MoonSystem {
        MoonSystem {
            positions: vec![],
            velocities: vec![],
            prev_state: vec![],
        }
    }

    // <x=-8, y=-10, z=0>
    fn add_moon(&mut self, line: &String) {
        let mut iter = line.split(", ");
        let x_str = iter.next().unwrap()[3..].to_string();
        let y_str = iter.next().unwrap()[2..].to_string();
        let z_str_tmp = iter.next().unwrap()[2..].to_string();
        let z_str = z_str_tmp[..z_str_tmp.len()-1].to_string();

        let mut pos = Vector::new();
        let vel = Vector::new();

        if let Ok(x) = x_str.parse::<i64>() { pos.x = x; } 
        if let Ok(y) = y_str.parse::<i64>() { pos.y = y; } 
        if let Ok(z) = z_str.parse::<i64>() { pos.z = z; } 

        self.positions.push(pos.clone());
        self.velocities.push(vel.clone());

        self.prev_state.push(BTreeSet::new());
        
        let moon_count = self.positions.len();

        self.prev_state[moon_count-1].insert((pos.clone(), vel.clone()));
    }

    fn update_positions(&mut self) {
        for i in 0..self.positions.len() {
            self.positions[i].x += self.velocities[i].x;
            self.positions[i].y += self.velocities[i].y;
            self.positions[i].z += self.velocities[i].z;
        }

    }

    fn determine_velocities(&mut self) {
        for i in 0..self.velocities.len() {
            for j in 0..self.velocities.len() {
                if i!=j {
                    self.velocities[i].x += if self.positions[i].x > self.positions[j].x { -1 } else if self.positions[i].x < self.positions[j].x { 1 } else { 0 };
                    self.velocities[i].y += if self.positions[i].y > self.positions[j].y { -1 } else if self.positions[i].y < self.positions[j].y { 1 } else { 0 };
                    self.velocities[i].z += if self.positions[i].z > self.positions[j].z { -1 } else if self.positions[i].z < self.positions[j].z { 1 } else { 0 };
                }
            }
        }
    }

    fn calculate_energy(&self) -> i64 {
        let mut energy = 0;
        for i in 0..self.positions.len() {
            energy += self.positions[i].energy() * self.velocities[i].energy();
        }
        energy
    }

    fn print(&self) {
        println!("--------------------------------------------");
        for i in 0..self.positions.len() {
            println!("{} -> {} {} {} - {} {} {}",i, self.positions[i].x, self.positions[i].y, self.positions[i].z, self.velocities[i].x, self.velocities[i].y, self.velocities[i].z)
        }
        println!("--------------------------------------------");
    }

    fn find_repeats(&mut self) {
        let mut t = 0;
        loop {
            self.run(1);
            t += 1;
            for i in 0..self.positions.len() {
                if self.prev_state[i].contains(&(self.positions[i].clone(), self.velocities[i].clone())) {
                    println!(" found repetition pos {} at {}", i, t);
                }

                self.prev_state[i].insert( (self.positions[i].clone(), self.velocities[i].clone()) );
            }
        }
    }

    fn run(&mut self, timesteps: i64) {
        let mut t = 0;
        while t < timesteps {
            self.determine_velocities();
            self.update_positions();
            t+=1;
        }
    }
}

#[allow(dead_code)]
pub fn run() {
    println!("Day 12 of 2019");

    let start0 = Instant::now();

    // let input_file = "./input/day12_19_test.txt";
    let input_file = "./input/day12_19_real.txt";
    let input = tools::get_input(String::from(input_file));

    let mut system1 = MoonSystem::new();
    let mut system2 = MoonSystem::new();
    input.iter().for_each(|line| {
        system1.add_moon(line);
        system2.add_moon(line);
    });

    let after0 = Instant::now();
    println!("Init in {:?}", after0.duration_since(start0));

    let start1 = Instant::now();

    system1.run(1000);
    let res1 = system1.calculate_energy();

    let after1 = Instant::now();
    println!("Part 1: {}, in {:?}", res1, after1.duration_since(start1));

    let start2 = Instant::now();

    system2.find_repeats();

    let after2 = Instant::now();
    println!("Part 2: {}, in {:?}", 0, after2.duration_since(start2));
}

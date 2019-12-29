use super::tools;
use std::collections::BTreeMap;
use std::time::Instant;

#[derive(Clone, Eq, PartialEq)]
enum RTGelement {
    GENERATOR = 1,
    MICROCHIP,
}

#[derive(Clone, Eq, PartialEq)]
struct RTG {
    name: String,
    generator: i8,
    microchip: i8,
}

#[derive(Clone, Eq, PartialEq)]
struct Building {
    elevator: i8,
    rtgs: BTreeMap<String, RTG>,
}

impl RTG {
    #[allow(dead_code)]
    fn new(n: String, g: i8, m: i8) -> RTG {
        RTG {
            name: n,
            generator: g,
            microchip: m,
        }
    }

    #[allow(dead_code)]
    fn element_on_floor(&self, element: RTGelement) -> i8 {
        match element {
            RTGelement::MICROCHIP => self.microchip,
            RTGelement::GENERATOR => self.generator,
        }
    }
}

impl Building {
    #[allow(dead_code)]
    fn new() -> Building {
        Building {
            elevator: 0,
            rtgs: BTreeMap::new(),
        }
    }

    #[allow(dead_code)]
    fn add_rtg(&mut self, name: String, element: RTGelement, floor: i8) {
        if self.rtgs.contains_key(&name) {
            match element {
                RTGelement::GENERATOR => {
                    self.rtgs.get_mut(&name).unwrap().generator = floor;
                }
                RTGelement::MICROCHIP => {
                    self.rtgs.get_mut(&name).unwrap().microchip = floor;
                }
            }
        } else {
            match element {
                RTGelement::GENERATOR => {
                    self.rtgs
                        .insert(name.to_string(), RTG::new(name, floor, -1));
                }
                RTGelement::MICROCHIP => {
                    self.rtgs
                        .insert(name.to_string(), RTG::new(name, -1, floor));
                }
            }
        }
    }

    #[allow(dead_code)]
    fn print_state(&self) {
        println!("-------------------------------------------------");
        for floornr in (0..4).rev() {
            let mut floorstr = format!("F{} ", floornr + 1);
            if self.elevator == floornr {
                floorstr = format!("{}E ", floorstr);
            } else {
                floorstr = format!("{}  ", floorstr);
            }
            for rtg in &self.rtgs {
                if rtg.1.generator == floornr {
                    let namestr = rtg.1.name[..3].to_string();
                    floorstr = format!("{}G{} ", floorstr, namestr);
                    // floorstr.append(String::from(rtg.1.name));
                }
                if rtg.1.microchip == floornr {
                    let namestr = rtg.1.name[..3].to_string();
                    floorstr = format!("{}M{} ", floorstr, namestr);
                    // floorstr.append(String::from(rtg.1.name));
                }
            }
            println!("{}", floorstr);
        }
        println!("-------------------------------------------------");
    }
}

fn search(start: &Building, part: usize) -> u64 {
    let mut items_per_floor = vec![0;4];
    let mut total_count = start.rtgs.len() * 2;

    for rtg in &start.rtgs {
        items_per_floor[rtg.1.element_on_floor(RTGelement::GENERATOR) as usize] += 1;
        items_per_floor[rtg.1.element_on_floor(RTGelement::MICROCHIP) as usize] += 1;
    }

    if part==2 {
        items_per_floor[0] += 4;
        total_count += 4;
    }

    let mut moves = 0;
    let mut floor = 0;
    while items_per_floor[3] != total_count {
        moves += 2 * items_per_floor[floor] - 3;
        items_per_floor[floor+1] += items_per_floor[floor];
        floor += 1;
    }
    moves as u64
}

#[allow(dead_code)]
pub fn run() {
    println!("Day 11 of 2016");

    // let input_file = "./input/day11_16_test.txt";
    let input_file = "./input/day11_16_real.txt";

    let start1 = Instant::now();
    let input = tools::get_input(String::from(input_file));

    let mut building: Building = Building::new();
    for line in &input {
        //The first floor contains a hydrogen-compatible microchip and a lithium-compatible microchip.
        let mut tokens = line.split_whitespace().peekable();
        tokens.next(); // The
        let floor = tokens.next().unwrap();
        let mut floornr = 0;
        match floor {
            "first" => {
                floornr = 0;
            }
            "second" => {
                floornr = 1;
            }
            "third" => {
                floornr = 2;
            }
            "fourth" => {
                floornr = 3;
            }
            _ => {}
        }
        tokens.next(); // floor
        tokens.next(); // contains
        while let Some(_) = tokens.peek() {
            // a or and
            let a_or_nothing = tokens.next().unwrap(); // a or nothing
            if a_or_nothing == "and" { tokens.next(); }
            let mut naming = tokens.next().unwrap();
            if naming != "relevant." {
                if naming.contains("-") {
                    // microchip
                    let mcname = &naming[..naming.len() - 11];
                    naming = mcname;
                } else { // generator
                }
                let mut elementtype = tokens.next().unwrap();
                if elementtype.contains('.') || elementtype.contains(',') {
                    elementtype = &elementtype[..elementtype.len() - 1];
                }
                match elementtype {
                    "generator" => {
                        building.add_rtg(naming.to_string(), RTGelement::GENERATOR, floornr);
                    }
                    "microchip" => {
                        building.add_rtg(naming.to_string(), RTGelement::MICROCHIP, floornr);
                    }
                    _ => {}
                }
            }
        }
    }

    // building.print_state();
    // println!("building id: {}", building.gen_id());

    let res1 = search(&building,1);

    let after1 = Instant::now();
    println!("Part 1: {}, in {:?}", res1, after1.duration_since(start1));

    let start2 = Instant::now();
    let res2 = search(&building,2);

    let after2 = Instant::now();
    println!("Part 2: {}, in {:?}", res2, after2.duration_since(start2));
}

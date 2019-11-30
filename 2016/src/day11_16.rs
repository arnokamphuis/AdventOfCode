use std::collections::HashMap;
use std::time::Instant;
use super::tools;

macro_rules! hashmap {
    ($( $key: expr => $val: expr ),*) => {{
         let map = ::std::collections::HashMap::new();
         $( map.insert($key, $val); )*
         map
    }}
}

enum RTGelement {
    GENERATOR = 1,
    MICROCHIP
}

#[derive(Clone)]
struct RTG {
    name: String,
    generator: i8,
    microchip: i8,
}

#[derive(Clone)]
struct Building {
    elevator: i8,
    rtgs: HashMap<String, RTG>,
}

impl RTG {
    #[allow(dead_code)]
    fn new(n: String, g: i8, m: i8) -> RTG {
        RTG {
            name: n,
            generator: g,
            microchip: m
        }
    }

    #[allow(dead_code)]
    fn active(&self) -> bool {
        self.generator == self.microchip
    }

    #[allow(dead_code)]
    fn on_floor(&self) -> i8 {
        if self.active() { self.generator } else { -1 }
    }

    #[allow(dead_code)]
    fn element_on_floor(&self, element: RTGelement) -> i8 {
        match element {
            RTGelement::MICROCHIP => { self.microchip }
            RTGelement::GENERATOR => { self.generator }
        }
    }

    #[allow(dead_code)]
    fn gen_id(&self) -> String {
        format!("{}{}{}", self.name.to_string(), self.generator.to_string(), self.microchip.to_string())
    }
}

impl Building {
    #[allow(dead_code)]
    fn new() -> Building {
        Building { elevator: 0, rtgs: hashmap![] }
    }

    #[allow(dead_code)]
    fn add_rtg(&mut self, name: String, element: RTGelement, floor: i8) {
        if self.rtgs.contains_key(&name) {
            match element {
                RTGelement::GENERATOR => { self.rtgs.get_mut(&name).unwrap().generator = floor; }
                RTGelement::MICROCHIP => { self.rtgs.get_mut(&name).unwrap().microchip = floor; }
            }
        } else {
            match element {
                RTGelement::GENERATOR => { self.rtgs.insert(name.to_string(), RTG::new(name, floor, -1)); }
                RTGelement::MICROCHIP => { self.rtgs.insert(name.to_string(), RTG::new(name, -1, floor)); }
            }
        }
    }

    #[allow(dead_code)]
    fn move_elevator(&mut self, direction: i8, items: Vec<(String, RTGelement)>) -> bool {
        if items.len() > 2 || items.len()==0 {
            // println!("ERROR: elevator needs contents or is overloaded");
            false
        } else {
            self.elevator += direction;
            for item in &items {
                match item.1 {
                    RTGelement::GENERATOR => { self.rtgs.get_mut(&item.0).unwrap().generator += direction; }
                    RTGelement::MICROCHIP => { self.rtgs.get_mut(&item.0).unwrap().microchip += direction; }
                }
            }
            true
        }
    }

    #[allow(dead_code)]
    fn print_state(&self) {
        println!("-------------------------------------------------");
        for floornr in 0..4 {
            let mut floorstr = format!("F{} ", floornr+1);
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

    #[allow(dead_code)]
    fn is_valid(&self) -> bool {
        let mut valid = true;
        for floornr in 0..5 {
            for rtg in &self.rtgs {
                if rtg.1.on_floor() == floornr {
                    for ortg in &self.rtgs {
                        if ortg.0 != rtg.0 && ortg.1.element_on_floor(RTGelement::MICROCHIP) == floornr && ortg.1.element_on_floor(RTGelement::GENERATOR) != floornr {
                            valid = false;
                        }
                    }
                }
            }
        }
        valid
    }

    #[allow(dead_code)]
    fn gen_id(&self) -> String {
        let mut id = format!("{}",
            self.elevator.to_string() 
        );

        for rtg in &self.rtgs {
            id = format!("{}{}", id, rtg.1.gen_id());
        }

        id
    }
}

#[allow(dead_code)]
pub fn run() {
    println!("Day 11 of 2016");

    let input_file = "./input/day11_16_test.txt";
    // let input_file = "./input/day11_16_real.txt";

    let start1 = Instant::now();
    let input = tools::get_input(String::from(input_file));

    let mut building: Building = Building::new();
    for line in &input {
        //The first floor contains a hydrogen-compatible microchip and a lithium-compatible microchip.
        let mut tokens = line.split_whitespace();
        tokens.next(); // The
        let floor = tokens.next().unwrap();
        let mut floornr = 0;
        match floor {
            "first" => { floornr = 0; }
            "second" => { floornr = 1; }
            "third" => { floornr = 2; }
            "fourth" => { floornr = 3; }
            _ => {}
        }
        tokens.next(); // floor
        while let Some(_) = tokens.next() { // contains or and
            tokens.next(); // a or nothing
            let mut naming = tokens.next().unwrap();
            if naming != "relevant." {
                if naming.contains("-") { // microchip
                    let mcname = &naming[..naming.len()-11];
                    naming = mcname;
                } else { // generator
                }
                let mut elementtype = tokens.next().unwrap();
                if elementtype.contains('.') {
                    elementtype = &elementtype[..elementtype.len()-1];
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

    building.print_state();

    let after1 = Instant::now();
    println!(
        "Part 1: {}, in {:?}",
        0,
        after1.duration_since(start1)
    );

    let start2 = Instant::now();

    let after2 = Instant::now();
    println!(
        "Part 2: {}, in {:?}",
        0,
        after2.duration_since(start2)
    );
}

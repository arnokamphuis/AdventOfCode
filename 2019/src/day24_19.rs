use super::tools;
use std::collections::BTreeMap;
use std::collections::HashSet;
use std::time::Instant;
use tools::Image;

struct Field {
    map: Vec<bool>,
    history: HashSet<Vec<bool>>,
}

impl Field {
    fn new() -> Field {
        Field {
            map: vec![false; 25],
            history: HashSet::new(),
        }
    }

    fn add_line(&mut self, index: usize, line: &String) {
        line.chars()
            .enumerate()
            .for_each(|(i, c)| self.map[index * 5 + i] = c == '#');
    }

    fn update(&mut self) -> bool {
        let mut new_map = self.map.clone();
        for i in 0..25 {
            let mut count = 0;
            if i % 5 != 0 {
                count += if self.map[i - 1] { 1 } else { 0 };
            }
            if i % 5 != 4 {
                count += if self.map[i + 1] { 1 } else { 0 };
            }
            if i < 20 {
                count += if self.map[i + 5] { 1 } else { 0 };
            }
            if i > 4 {
                count += if self.map[i - 5] { 1 } else { 0 };
            }
            // print!("{} -> count: {} ", i, count);
            if self.map[i] {
                // bug alive
                if count != 1 {
                    new_map[i] = false;
                    // println!("dies");
                    // } else {
                    //     println!("nothing happens");
                }
            } else {
                // empty cell
                if count == 1 || count == 2 {
                    new_map[i] = true;
                    //     println!("born");
                    // } else {
                    //     println!("nothing happens");
                }
            }
        }
        self.map = new_map.clone();
        if self.history.contains(&new_map) {
            true
        } else {
            self.history.insert(new_map);
            false
        }
    }

    fn calc_biodiversity(&self) -> i128 {
        let mut res = 0;
        for i in 0..25 {
            res += if self.map[i] { 1 << i } else { 0 }
        }
        res
    }

    #[allow(dead_code)]
    fn print(&self) {
        println!("---------------------------------------------------------");
        self.map.iter().enumerate().for_each(|(i, b)| {
            if *b {
                print!("#");
            } else {
                print!(".");
            }
            if i % 5 == 4 {
                println!("");
            }
        });
        println!("---------------------------------------------------------");
    }
}

struct RecursiveField {
    map: BTreeMap<i64, Vec<bool>>,
    min_depth: i64,
    max_depth: i64,
}

impl RecursiveField {
    fn new() -> RecursiveField {
        let mut res = RecursiveField {
            map: BTreeMap::new(),
            min_depth: 0,
            max_depth: 0,
        };
        res.map.insert(0, vec![false; 25]);
        res
    }

    fn add_line(&mut self, index: usize, line: &String) {
        self.map.entry(0).and_modify(|f| {
            line.chars().enumerate().for_each(|(i, c)| {
                f[index * 5 + i] = c == '#';
            });
        });
    }

    fn count_bugs(&self) -> i64 {
        let mut count = 0;

        for level in self.min_depth..=self.max_depth {
            for i in 0..25 {
                if i != 12 {
                    if self.map[&level][i] {
                        count += 1;
                    }
                }
            }
        }

        count
    }

    #[allow(dead_code)]
    fn print(&self) {
        println!("*********************************************************");
        for level in self.min_depth..=self.max_depth {
            println!("---------------------------------------------------------");
            println!(" Depth {}", level);
            self.map[&level].iter().enumerate().for_each(|(i, b)| {
                if i == 12 {
                    print!("?");
                } else {
                    if *b {
                        print!("#");
                    } else {
                        print!(".");
                    }
                    if i % 5 == 4 {
                        println!("");
                    }
                }
            });
            println!("---------------------------------------------------------");
        }
        println!("*********************************************************");
    }

    fn update(&mut self) {
        self.min_depth -= 1;
        self.max_depth += 1;
        self.map.insert(self.min_depth, vec![false; 25]);
        self.map.insert(self.max_depth, vec![false; 25]);

        let mut new_map = self.map.clone();

        for level in self.min_depth..=self.max_depth {
            for i in 0..25 {
                if i != 12 {
                    let mut count = 0;

                    // check left neighbour
                    if i % 5 != 0 {
                        // not left border
                        if (i - 1) != 12 {
                            count += if self.map[&level][i - 1] { 1 } else { 0 };
                        }
                    } else {
                        // left border (need to traverse higher)
                        if level < self.max_depth {
                            if self.map[&(level + 1)][11] {
                                count += 1;
                            }
                        }
                    }

                    // check right neighbour
                    if i % 5 != 4 {
                        // not right border
                        if (i + 1) != 12 {
                            count += if self.map[&level][i + 1] { 1 } else { 0 };
                        }
                    } else {
                        // right border (need to traverse higher)
                        if level < self.max_depth {
                            if self.map[&(level + 1)][13] {
                                count += 1;
                            }
                        }
                    }

                    // check bottom neighbour
                    if i < 20 {
                        // not bottom border
                        if (i + 5) != 12 {
                            count += if self.map[&level][i + 5] { 1 } else { 0 };
                        }
                    } else {
                        // bottom border (need to traverse higher)
                        if level < self.max_depth {
                            if self.map[&(level + 1)][17] {
                                count += 1;
                            }
                        }
                    }

                    // check top neighbour
                    if i > 4 {
                        // not top border
                        if (i - 5) != 12 {
                            count += if self.map[&level][i - 5] { 1 } else { 0 };
                        }
                    } else {
                        // top border (need to traverse higher)
                        if level < self.max_depth {
                            if self.map[&(level + 1)][7] {
                                count += 1;
                            }
                        }
                    }

                    // check inner neighbours

                    // need to take A B C D E in consideration of lower level
                    if i == 7 {
                        if level > self.min_depth {
                            for ii in 0..5 {
                                if self.map[&(level - 1)][ii] {
                                    count += 1;
                                }
                            }
                        }
                    }

                    // need to take U V W X Y in consideration of lower level
                    if i == 17 {
                        if level > self.min_depth {
                            for ii in 20..25 {
                                if self.map[&(level - 1)][ii] {
                                    count += 1;
                                }
                            }
                        }
                    }

                    // need to take A F K P U in consideration of lower level
                    if i == 11 {
                        if level > self.min_depth {
                            for ii in 0..5 {
                                if self.map[&(level - 1)][5 * ii] {
                                    count += 1;
                                }
                            }
                        }
                    }

                    // need to take E J O T Y in consideration of lower level
                    if i == 13 {
                        if level > self.min_depth {
                            for ii in 0..5 {
                                if self.map[&(level - 1)][5 * ii + 4] {
                                    count += 1;
                                }
                            }
                        }
                    }

                    // print!("{} -> count: {} ", i, count);
                    if self.map[&level][i] {
                        // bug alive
                        if count != 1 {
                            new_map.entry(level).and_modify(|m| m[i] = false);
                            // println!("dies");
                            // } else {
                            //     println!("nothing happens");
                        }
                    } else {
                        // empty cell
                        if count == 1 || count == 2 {
                            new_map.entry(level).and_modify(|m| m[i] = true);
                            //     println!("born");
                            // } else {
                            //     println!("nothing happens");
                        }
                    }
                }
            }
        }

        if new_map[&self.min_depth].iter().filter(|b| **b).count() == 0 {
            new_map.remove(&self.min_depth);
            self.min_depth += 1;
        }
        if new_map[&self.max_depth].iter().filter(|b| **b).count() == 0 {
            new_map.remove(&self.max_depth);
            self.max_depth -= 1;
        }
        self.map = new_map.clone();
    }

    #[allow(dead_code)]
    fn save(&self, frame: &String) {
        // const MAX_DEPTH: u32 = 202;
        let w = 15 * 6; // 15 = sqrt(202)
        let mut img = Image::new(16 * w / 9, w, 20);
        img.clear((40, 40, 40, 255));
        let extra_offset = ((16 * w / 9) - w) / 2;

        let total_depth = self.max_depth - self.min_depth + 1;
        let mut level_offset_x = 0;
        let mut level_offset_y = 0;

        for level in -110..=110 {
            if self.map.contains_key(&level) {
                // println!("working on level {}", level);
                // loop from the highest depth down to the lower levels
                let depth = 110 - level;
                let remaining_levels = total_depth - depth;
                let cell_width = 6;
                level_offset_x = (depth % 15) * cell_width;
                level_offset_y = (depth / 15) * cell_width;

                for i in 0..25 {
                    if i != 12 {
                        // offset within this level
                        let xi = i % 5;
                        let yi = i / 5;

                        let x = xi + level_offset_x + extra_offset as i64;
                        let y = yi + level_offset_y;

                        let level_value: i64 = level;
                        let color = if self.map[&level_value][i as usize] {
                            if level == 0 {
                                (255, 0, 255, 255)
                            } else {
                                (255, 255, 255, 255)
                            }
                        } else {
                            (0, 0, 0, 255)
                        };
                        img.set_pixel(x as usize, y as usize, color);
                    }
                }
            }
        }

        img.save_png(&String::from(format!("movie/{}.png", frame)));
    }
}

#[allow(dead_code)]
pub fn run() {
    println!("Day 24 of 2019");

    let start0 = Instant::now();

    // let input_file = "./input/day24_19_test.txt";
    let input_file = "./input/day24_19_real.txt";
    let input = tools::get_input(String::from(input_file));

    let mut f1 = Field::new();
    let mut f2 = RecursiveField::new();
    input.iter().enumerate().for_each(|(i, line)| {
        f1.add_line(i, line);
        f2.add_line(i, line);
    });

    let after0 = Instant::now();
    println!("Init in {:?}", after0.duration_since(start0));

    let start1 = Instant::now();

    while !f1.update() {}
    let res1 = f1.calc_biodiversity();

    let after1 = Instant::now();
    println!("Part 1: {}, in {:?}", res1, after1.duration_since(start1));

    let start2 = Instant::now();

    for t in 0..200 {
        f2.update();
        f2.save(&format!("{:03}", t).to_string());
    }
    let res2 = f2.count_bugs();

    let after2 = Instant::now();
    println!("Part 2: {}, in {:?}", res2, after2.duration_since(start2));
}

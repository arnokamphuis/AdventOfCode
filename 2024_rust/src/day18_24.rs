use super::tools;
use std::time::Instant;
use itertools::Itertools;
use std::collections::HashSet;
use tools::Image;

struct ByteMap {
    map: HashSet::<(i32,i32)>,
    size: i32,
    byte_locations: Vec<(i32,i32)>,
    frame_counter: i32,
    img: tools::Image,
    lt: i32,
}

impl ByteMap {
    fn new(s: i32, bl: Vec<(i32,i32)>) -> ByteMap {
        let mut map = HashSet::<(i32,i32)>::new();
        for x in 0..s {
            for y in 0..s {
                map.insert((x,y));
            }
        }
        ByteMap {
            map: map,
            size: s,
            byte_locations: bl,
            frame_counter: 0,
            img: Image::new(s as usize, s as usize, 4),
            lt: 0,
        }
    }

    #[allow(dead_code)]
    fn make_frame(&mut self, time: i32, seen: &HashSet<(i32,i32)>) {
        self.img.clear((0, 0, 0, 255));
        for y in 0..self.size {
            for x in 0..self.size {
                if !self.map.contains(&(x,y)) {
                    self.img.set_pixel(x as usize, y as usize, (255, 255, 255, 255));
                }
            }
        }
        for i in 0..(time as usize) {
            let (x,y) = self.byte_locations[i];
            self.img.set_pixel(x as usize, y as usize, (255, 0, 0, 255));
        }
        let (x,y) = self.byte_locations[(time-1) as usize];
        self.img.set_pixel(x as usize, y as usize, (0, 255, 0, 255));
        
        for (x,y) in seen {
            self.img.set_pixel(*x as usize, *y as usize, (0, 0, 255, 255));
        }
        self.img.save_png(&format!("images/day18_24_{:06}.png", self.frame_counter));
        self.frame_counter += 1;
    }
    
    fn map_update(&mut self, time: i32) {
        let (x,y) = self.byte_locations[(time-1) as usize];
        self.map.remove(&(x,y));
    }
    
    fn map_after_ns(&mut self, time: i32) {
        for x in 0..self.size {
            for y in 0..self.size {
                self.map.insert((x,y));
            }
        }
        for t in 1..=time {
            self.map_update(t);
        }
        self.lt = time as i32;
    }

    fn find_path(&mut self, start: (i32,i32), end: (i32,i32)) -> i32 {
        let mut current = start;
        let mut distance;
        let mut queue = Vec::<((i32,i32),i32)>::new();
        let mut seen: HashSet::<(i32,i32)> = HashSet::new();
        queue.push((current, 0));
    
        while !queue.is_empty() {
            (current, distance) = queue.remove(0);
            if seen.contains(&current) {
                continue;
            }
            seen.insert(current);
            
            #[cfg(feature = "make_movie")]
            self.make_frame(self.lt, &seen);

            if current == end {
                return distance;
            }
            let neighbours = vec![(current.0-1,current.1),(current.0+1,current.1),(current.0,current.1-1),(current.0,current.1+1)];
            for neighbour in neighbours {
                if self.map.contains(&neighbour) {
                    queue.push((neighbour, distance+1));
                }
            }
        }
    
        i32::MAX
    }
  
    #[allow(dead_code)]
    fn print_map(&self) {
        for y in 0..self.size {
            for x in 0..self.size {
                if self.map.contains(&(x,y)) {
                    print!(".");
                } else {
                    print!("#");
                }
            }
            println!();
        }
    }
    
}

#[allow(dead_code)]
pub fn run(real: bool, print_result: bool) -> (u128, u128, u128) {
    let start0 = Instant::now();

    let input_file: &str = if !real {
        "./input/day18-test.txt"
    } else {
        "./input/day18-real.txt"
    };
    let input = tools::get_input(String::from(input_file));
    let byte_locations = input.iter()
        .map(|x| x.split(","))
        .map(|x| x.map(|y| y.parse::<i32>().unwrap()).collect::<Vec<i32>>())
        .map(|x| x.iter().map(|v| *v).collect_tuple().unwrap())
        .collect::<Vec<(i32, i32)>>();

    let size = if !real { 7 } else { 71 };
    let last_time = if !real { 12 } else { 1024 };

    let after0 = Instant::now();

    let start1 = Instant::now();

    let mut map = ByteMap::new(size, byte_locations.clone());
    map.map_after_ns(last_time);

    let res1 = map.find_path((0,0), (size-1,size-1));

    let after1 = Instant::now();
    if print_result {
        println!("Part 1: {}", res1);
    }

    let start2 = Instant::now();

    let mut search_range = (0_i32, byte_locations.len() as i32);
    while search_range.1 - search_range.0 > 1 {
        let t = (search_range.0 + search_range.1) / 2;
        map.map_after_ns(t);
        if map.find_path((0,0), (size-1,size-1)) != i32::MAX {
            search_range.0 = t;
        } else {
            search_range.1 = t;
        }
    }
    let byte_index = search_range.0 as usize;
    let byte = byte_locations[byte_index];

    let res2 = format!("{},{}", byte.0, byte.1);

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

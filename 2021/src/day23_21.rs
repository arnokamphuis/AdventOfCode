// use super::tools;
use std::time::Instant;
use std::collections::BinaryHeap;
use std::cmp::Ordering;
use std::collections::HashMap;

#[derive(Clone, Eq, PartialEq)]
struct State {
    energy : usize,
    top    : [char; 11],
    bottom : [[char;4]; 4],
}

impl State {

    fn new() -> State {
        State {
            energy: 0,
            top: ['.'; 11],
            bottom: [ ['A'; 4], ['B'; 4], ['C'; 4], ['D'; 4] ]
        }
    }

    fn cost(&self, c: char) -> usize {
        match c {
            'A' => {1},
            'B' => {10},
            'C' => {100},
            'D' => {1000},
            _ => {panic!();}
        }
    }

    fn room_index(&self, c: char) -> usize {
        (c as u32 - 'A' as u32) as usize
    }

    fn finished(&self) -> bool {
        self.is_done('A') && self.is_done('B') && self.is_done('C') && self.is_done('D') 
    }

    fn is_empty(&self, c: char) -> bool {
        self.bottom[self.room_index(c)].iter().all(|&ch| ch=='.')
    }

    fn get_first(&self, room: char) -> Option<(usize, char)> {
        for (depth, c) in self.bottom[self.room_index(room)].iter().enumerate() {
            if *c != '.' { return Some((depth, *c)); }
        }
        None
    }

    fn get_free_in_room(&self, room: char) -> Option<usize> {
        if self.bottom[self.room_index(room)].iter().all(|&ch| ch == '.' || ch == room) {
            for (depth, c) in self.bottom[self.room_index(room)].iter().enumerate() {
                if *c != '.' { return Some(depth-1); }
            }
            return Some(3);
        }
        None
    }

    fn is_done(&self, c: char) -> bool {
        self.bottom[self.room_index(c)].iter().all(|&ch| ch==c)
    }

    fn get_top_targets(&self) -> Vec<usize> {
        [0, 1, 3, 5, 7, 9, 10].iter().filter(|&index| self.top[*index] == '.').map(|&v|v).collect::<Vec<usize>>()
    }

    fn get_bottom_targets(&self) -> Vec<char> {
        ['A', 'B', 'C', 'D'].iter().filter(|&ch| self.is_empty(*ch)).map(|&c|c).collect::<Vec<char>>()
    }

    fn move_room_to_room(&mut self, from_room: char, to_room: char) -> Option<usize> {
        if from_room == to_room { return None; }

        let from_pos = 2 * (1 + self.room_index(from_room));
        let to_pos   = 2 * (1 + self.room_index(to_room));
        let left     = from_pos.min(to_pos);
        let right    = from_pos.max(to_pos);

        if self.top[left..=right].iter().all(|&c| c=='.') {
            if let Some((from_depth, from_ch)) = self.get_first(from_room) {
                if from_ch != to_room { return None; }

                if let Some(to_depth) = self.get_free_in_room(to_room) {
                    self.bottom[self.room_index(from_room)][from_depth] = '.';
                    self.bottom[self.room_index(to_room)][to_depth] = from_ch;
                    return Some((from_depth + to_depth + 2 * (right - left)) * self.cost(from_ch));
                }
            }
        }

        None
    }

    fn move_from_room(&mut self, room: char, to: usize) -> Option<usize> {
        if self.top[to] != '.' { return None; }
        let from_pos = 2 * (1 + self.room_index(room));
        if let Some((depth,ch)) = self.get_first(room) {
            if self.top[from_pos.min(to)+1..from_pos.max(to)].iter().all(|&c| c=='.') {
                self.top[to] = ch;
                self.bottom[self.room_index(room)][depth] = '.';
                return Some( (1+depth + from_pos.max(to) - from_pos.min(to)) * self.cost(ch) );
            }
        }
        None
    }

    fn move_to_room(&mut self, from: usize, room: char) -> Option<usize> {
        if self.top[from] == '.' { return None; }

        let movable = self.top[from];
        if movable != room { return None; }

        let to_pos = 2 * (1 + self.room_index(room));
        if let Some(depth) = self.get_free_in_room(room) {
            if self.top[to_pos.min(from)+1..to_pos.max(from)].iter().all(|&c| c=='.') {
                let ch = self.top[from];
                self.bottom[self.room_index(room)][depth] = ch;
                self.top[from] = '.';
                return Some( (1+depth + to_pos.max(from) - to_pos.min(from)) * self.cost(ch) );
            }
        }
        None
    }

    fn get_top_amphipods(&self) -> Vec<(char, usize)> {
        self.top.iter().enumerate().filter(|(_,&ch)| ch != '.').map(|(index,&ch)| (ch,index)).collect()
    }

}

impl Ord for State {
    fn cmp(&self, other: &Self) -> Ordering {
        other.energy.cmp(&self.energy)
    }
}

impl PartialOrd for State {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}


#[allow(dead_code)]
pub fn run(_real: bool, print_result: bool) -> (u128, u128, u128) {
    let start0 = Instant::now();

    let after0 = Instant::now();


    let start1 = Instant::now();
    // solved by hand
    let after1 = Instant::now();
    if print_result {
        println!("Part 1: {}", 15160);
    }

    let start2 = Instant::now();

    let mut initial_state = State::new();
    initial_state.bottom[0] = ['D', 'D', 'D', 'B'];
    initial_state.bottom[1] = ['C', 'C', 'B', 'C'];
    initial_state.bottom[2] = ['A', 'B', 'A', 'D'];
    initial_state.bottom[3] = ['B', 'A', 'C', 'A'];

    let mut least_energy = usize::MAX;
    let mut heap: BinaryHeap<State> = BinaryHeap::new();
    let mut energies: HashMap<([char;11], [[char;4];4]), usize> = HashMap::new();
    heap.push(initial_state);
    loop {
        if let Some(s) = heap.pop() {
            if s.finished() {
                least_energy = s.energy;
                break;
            }

            let movables = s.get_top_amphipods();
            let top_targets = s.get_top_targets();

            let mut insert_next_state = | s: &State | {
                if energies.contains_key(&(s.top, s.bottom)) {
                    if let Some(prev_energy) = energies.get(&(s.top, s.bottom)) {
                        if s.energy < *prev_energy {
                            *energies.entry((s.top, s.bottom)).or_insert(0) = s.energy;
                            heap.push(s.clone());
                        }
                    } else {
                        panic!();
                    }
                } else {
                    energies.insert((s.top, s.bottom), s.energy);
                    heap.push(s.clone());
                }
            };

            top_targets.iter().for_each(|tt| {
                ['A', 'B', 'C', 'D'].iter().for_each(|room| {
                    let mut next_state = s.clone();
                    if let Some(energy) = next_state.move_from_room(*room, *tt) {
                        next_state.energy += energy;
                        insert_next_state(&next_state);
                    } 
                });
            });

            ['A', 'B', 'C', 'D'].iter().for_each(|from_room| {
                s.get_bottom_targets().iter().for_each(|to_room| {
                    let mut next_state = s.clone();
                    if let Some(energy) = next_state.move_room_to_room(*from_room, *to_room) {
                        next_state.energy += energy;
                        insert_next_state(&next_state);
                    }
                });
            });

            movables.iter().for_each(|(c, from)| {
                let mut next_state = s.clone();
                if let Some(energy) = next_state.move_to_room(*from, *c) {
                    next_state.energy += energy;
                    insert_next_state(&next_state);
                }
            });
        } else {
            break;
        }

    }

    let after2 = Instant::now();
    if print_result {
        println!("Part 2: {}", least_energy);
    }

    (
        after0.duration_since(start0).as_nanos(),
        after1.duration_since(start1).as_nanos(),
        after2.duration_since(start2).as_nanos(),
    )
}

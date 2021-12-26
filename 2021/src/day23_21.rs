// use super::tools;
use std::time::Instant;
use std::collections::BinaryHeap;
use std::cmp::Ordering;
use std::collections::HashMap;

#[derive(Clone, Eq, PartialEq)]
struct State {
    energy      : usize,
    top         : [char; 11],
    bottom      : Vec<Vec<char>>,
    bottom_size : usize,
}

impl State {

    fn new(s: usize) -> State {
        State {
            energy: 0,
            top: ['.'; 11],
            bottom: vec![ vec!['A'; 4], vec!['B'; 4], vec!['C'; 4], vec!['D'; 4] ],
            bottom_size: s,
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
            return Some(self.bottom[0].len()-1);
        }
        None
    }

    fn is_done(&self, c: char) -> bool {
        self.bottom[self.room_index(c)].iter().all(|&ch| ch==c)
    }

    fn get_top_targets(&self) -> Vec<usize> {
        [0, 1, 3, 5, 7, 9, 10].iter().filter(|&index| self.top[*index] == '.').map(|&v|v).collect::<Vec<usize>>()
    }

    fn move_from_room(&mut self, room: char, to: usize) -> Option<usize> {
        if self.top[to] != '.' { return None; }
        let from_pos = 2 * (1 + self.room_index(room));
        if let Some((depth,ch)) = self.get_first(room) {
            if self.top[from_pos.min(to)+1..from_pos.max(to)].iter().all(|&c| c=='.') {
                self.top[to] = ch;
                let room_index = self.room_index(room);
                self.bottom[room_index][depth] = '.';
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
                let room_index = self.room_index(room);
                self.bottom[room_index][depth] = ch;
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

fn solve(initial_state: State) -> usize {
    let mut least_energy = usize::MAX;
    let mut heap: BinaryHeap<State> = BinaryHeap::new();
    let mut energies: HashMap<([char;11], Vec<Vec<char>>), usize> = HashMap::new();
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
                let amphi_state = (s.top, s.bottom.clone());
                if energies.contains_key(&amphi_state) {
                    if let Some(prev_energy) = energies.get(&amphi_state) {
                        if s.energy < *prev_energy {
                            *energies.entry(amphi_state.clone()).or_insert(0) = s.energy;
                            heap.push(s.clone());
                        }
                    } else {
                        panic!();
                    }
                } else {
                    energies.insert((s.top, s.bottom.clone()), s.energy);
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
    least_energy
}


#[allow(dead_code)]
pub fn run(_real: bool, print_result: bool) -> (u128, u128, u128) {
    let start0 = Instant::now();

    let after0 = Instant::now();


    let start1 = Instant::now();

    let mut initial_state = State::new(2);
    initial_state.bottom[0] = vec!['D', 'B'];
    initial_state.bottom[1] = vec!['C', 'C'];
    initial_state.bottom[2] = vec!['A', 'D'];
    initial_state.bottom[3] = vec!['B', 'A'];
    let res1 = solve(initial_state.clone());

    let after1 = Instant::now();
    if print_result {
        println!("Part 1: {}", res1);
    }

    let start2 = Instant::now();

    let mut initial_state = State::new(4);
    initial_state.bottom[0] = vec!['D', 'D', 'D', 'B'];
    initial_state.bottom[1] = vec!['C', 'C', 'B', 'C'];
    initial_state.bottom[2] = vec!['A', 'B', 'A', 'D'];
    initial_state.bottom[3] = vec!['B', 'A', 'C', 'A'];
    let res2 = solve(initial_state.clone());

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

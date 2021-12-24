// use super::tools;
use std::time::Instant;
use std::collections::BinaryHeap;
use std::cmp::Ordering;
use std::collections::HashMap;

#[derive(Clone, Eq, PartialEq)]
struct State {
    energy: u64,
    positions: [char; 15],
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
pub fn run(real: bool, print_result: bool) -> (u128, u128, u128) {
    let start0 = Instant::now();

    // let input_file: &str = if !real {
    //     "./input/day23_21_test.txt"
    // } else {
    //     "./input/day23_21_real.txt"
    // };
    // let input = tools::get_input(String::from(input_file));

    let after0 = Instant::now();

    let mut start_state = ['.';15];
    if !real {
    // test
        start_state[7]  = 'B'; start_state[8]  = 'A';
        start_state[9]  = 'C'; start_state[10] = 'D';
        start_state[11] = 'B'; start_state[12] = 'C';
        start_state[13] = 'D'; start_state[14] = 'A';
    } else {
        start_state[7]  = 'D'; start_state[8]  = 'B';
        start_state[9]  = 'C'; start_state[10] = 'C';
        start_state[11] = 'A'; start_state[12] = 'D';
        start_state[13] = 'B'; start_state[14] = 'A';
    }

    let mut goal_state = ['.';15];
    // goal
    goal_state[7]  = 'A'; goal_state[8]  = 'A';
    goal_state[9]  = 'B'; goal_state[10] = 'B';
    goal_state[11] = 'C'; goal_state[12] = 'C';
    goal_state[13] = 'D'; goal_state[14] = 'D';

    let finished = | s: &[char;15] | -> bool {
        *s == goal_state
    };

    let swap = | s: &mut[char;15], from: usize, to: usize | {
        let temp = s[from]; s[from] = s[to]; s[to] = temp;
    };

    let cost = | c: char | -> u64 {
        match c {
            'A' => {1},
            'B' => {10},
            'C' => {100},
            'D' => {1000},
            _ => {panic!();}
        }
    };

    let allowed = | s: &[char;15], from: usize, to: usize | -> (u64, bool) {
        if from == to { return (0, false); }
        if s[from] == '.' || (from < 7 && to < 7)  { return (0, false); }
        if s[to] != '.' { return (0, false); }

        if to >= 7 {
            if !((to-7)/2 == 0 && s[from] == 'A') { return (0, false); }
            if !((to-7)/2 == 1 && s[from] == 'B') { return (0, false); }
            if !((to-7)/2 == 2 && s[from] == 'C') { return (0, false); }
            if !((to-7)/2 == 3 && s[from] == 'D') { return (0, false); }
        }
        
        if from >= 7 && to >= 7 {
            let from_bottom = (from - 7) % 2;
            let to_bottom   = ( to  - 7) % 2;

            // println!("FROM {} ({}) TO {} ({})", from, s[from], to, s[to]);

            // println!("from_bottom {}  s[from-1] = {}", from_bottom, s[from-1]);
            if from_bottom == 1 && s[from-1] != '.' { return (0, false); }

            // println!("to_bottom {}  s[to-1] = {}", to_bottom, s[to-1]);
            if to_bottom   == 1 && s[to-1]   != '.' { return (0, false); }

            // println!("to_bottom {}  s[to+1] = {}  s[from] = {}", to_bottom, s[to+1], s[from]);
            if to_bottom   == 0 && s[to+1] != s[from] { return (0, false); }

            let from_room = (from - 7)/2;
            let to_room   = ( to  - 7)/2;
            if from_room == to_room { return (0, false); }

            let left_room = from_room.min(to_room);
            let right_room = from_room.max(to_room);

            // println!("left room: {}  right room: {}", left_room, right_room);
            let dist = ((right_room+1) - (left_room+2) + from_bottom+2 + to_bottom+2) as u64;
            // println!("   dist: {}  and checking {:?}", dist, &s[left_room+2..=right_room+1]);

            return (dist, s[left_room+2..=right_room+1].iter().all(|&c| c == '.'));
        }

        if (from > 6 && to <=6) || (to > 6 && from <=6) {            
            let hallway = from.min(to);
            let room = (from.max(to) - 7)/2;
            let bottom = (from.max(to) - 7) % 2;
            if to > 6 {
                let c = s[from];
                if c == 'A' && room != 0 { return (0, false); }
                if c == 'B' && room != 1 { return (0, false); }
                if c == 'C' && room != 2 { return (0, false); }
                if c == 'D' && room != 3 { return (0, false); }
                if bottom == 0 && s[8+2*room] == '.' { return (0, false); }
            }

            if bottom > 0 && s[7+2*room] != '.' { return (0, false); }

            if hallway > room+1 {
                // println!("  checking: {:?}    from {}  to {}  hallway {}    room {}", &s[room+2..=hallway], from, to, hallway, room);
                return ((bottom+2+hallway-(room+2)) as u64, s[room+2..=hallway].iter().all(|&c| c == '.'));
            } else {
                // println!("  checking: {:?}", &s[hallway..=room+1]);
                return ((bottom+2+(room+1)-hallway) as u64, s[hallway..=room+1].iter().all(|&c| c == '.'));
            } 
        }
        return (0, false);
    };

    let start1 = Instant::now();

    let mut least_energy = u64::MAX;
    let mut heap: BinaryHeap<State> = BinaryHeap::new();
    let mut visited: HashMap<[char;15], u64> = HashMap::new();
    heap.push(State{ energy: 0, positions: start_state });
    loop {
        if let Some(s) = heap.pop() {
            if finished(&s.positions) {
                least_energy = s.energy;
                break;
            }

            
            (0..15).for_each(|from| {
                let c = s.positions[from];
                if c != '.' {
                    (0..15).for_each(|to| {
                        if from != to {
                            let (dist, allow) = allowed(&s.positions, from, to);
                            if allow {
                                let mut next = s.clone();
                                swap(&mut next.positions, from, to);
                                next.energy += dist * cost(c);
                                if !visited.contains_key(&next.positions) || next.energy < visited[&next.positions] {
                                    println!("from: {:?} - {}", s.positions, s.energy);
                                    println!("to:   {:?} - {}", next.positions, next.energy);
                                    *visited.entry(next.positions).or_insert(0) = next.energy;
                                    heap.push(next);
                                }
                            }
                        }
                    });
                }
            });
        } else {
            break;
        }
    }

    let after1 = Instant::now();
    if print_result {
        println!("Part 1: {}", least_energy);
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

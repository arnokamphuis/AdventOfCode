use super::tools;
use std::time::Instant;
use std::collections::HashMap;

#[derive(Clone, Debug)]
struct TunnelNetwork {
    initial_valve: usize,
    tunnels: Vec<Vec<usize>>,
    flow_rates: Vec<usize>,
    states: HashMap<usize, usize>,
    max_time: usize,
}

impl TunnelNetwork {
    #[allow(dead_code)]
    fn find_max_flow(&mut self, time: usize, valve: usize, current_open: usize, elephant_active: bool) -> usize {
        if time == self.max_time {
            if elephant_active {
                return self.find_max_flow(1, self.initial_valve, current_open, false);
            } else {
                return 0;
            }
        }

        let valve_index = valve as usize;

        let state = current_open * self.flow_rates.len() * 31 * 2 + valve_index * 31 * 2 + (time as usize) * 2 + (elephant_active as usize);

        if self.states.contains_key(&state) {
            return self.states[&state];
        }
    
        let mut res: usize = 0;
    
        let valve_not_open = (current_open & (1 << valve)) == 0;
        if valve_not_open && self.flow_rates[valve_index]>0 {
            let new_open = current_open | (1 << valve);
            res = res.max( 
                ((self.max_time-time) as usize) * self.flow_rates[valve_index] + 
                self.find_max_flow(time+1, valve, new_open, elephant_active) 
            );
        }
    
        for next_valve in self.tunnels[valve_index].clone() {
            res = res.max( 
                self.find_max_flow(time+1, next_valve, current_open, elephant_active) );
        }
    
        self.states.insert(state, res);
        res    
    }
}

#[allow(dead_code)]
pub fn run(real: bool, print_result: bool) -> (u128, u128, u128) {
    let start0 = Instant::now();

    let input_file: &str = if !real {
        "./input/day16_22_test.txt"
    } else {
        "./input/day16_22_real.txt"
    };
    let input = tools::get_input(String::from(input_file));

    let mut index_map: HashMap<String, usize> = HashMap::new();
    let mut flow_rates: Vec<usize> = vec![];

    input.iter().for_each(|line| {
        let words  = line.split_whitespace().collect::<Vec<&str>>();
        let valve  = words[1].to_string();
        index_map.insert(valve.clone(),flow_rates.len() as usize);

        let rate   = words[4][5..].strip_suffix(";").unwrap().parse::<usize>().unwrap();
        flow_rates.push(rate);
    });

    let mut tunnels: Vec<Vec<usize>> = vec![vec![]; flow_rates.len()];

    input.iter().for_each(|line| {
        let words  = line.split_whitespace().collect::<Vec<&str>>();
        let valve  = index_map[words[1]];
        words[9..].to_vec().iter().map(|s| {
            if let Some(ss) = s.strip_suffix(",") {
                ss.to_string()
            } else {
                s.to_string()
            }}).for_each(|tunnel| { 
                let to = index_map[&tunnel];
                tunnels[valve as usize].push(to);
            });
    });
    
    let initial = index_map["AA"];

    let after0 = Instant::now();

    let start1 = Instant::now();

    let res1 = TunnelNetwork {
        initial_valve: initial,
        tunnels: tunnels.clone(),
        flow_rates: flow_rates.clone(),
        states: HashMap::new(),
        max_time: 30,            
    }.find_max_flow(1, initial, 0, false);

    let after1 = Instant::now();
    if print_result {
        println!("Part 1: {}", res1);
    }

    let start2 = Instant::now();

    let res2 = TunnelNetwork {
        initial_valve: initial,
        tunnels: tunnels.clone(),
        flow_rates: flow_rates.clone(),
        states: HashMap::new(),
        max_time: 26,
    }.find_max_flow(1, initial, 0, true);

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

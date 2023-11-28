use super::tools;
use std::time::Instant;
use priority_queue::PriorityQueue;
use std::cmp::Reverse;
use std::collections::HashMap;
use tools::Image;

#[allow(dead_code)]
pub fn run(real: bool, print_result: bool) -> (u128, u128, u128) {
    let start0 = Instant::now();

    let input_file: &str = if !real {
        "./input/day24_22_test.txt"
    } else {
        "./input/day24_22_real.txt"
    };
    let input = tools::get_input(String::from(input_file));

    let start  = [ 1i64, 0i64];
    let goal   = [ input[0].len() as i64 - 2, input.len() as i64 - 1 ];
    let max    = [ input[0].len() as i64 - 1, input.len() as i64 - 1 ];
    let period = (max[0]-1) * (max[1]-1);

    let blizzards: Vec<([i64;2],[i64;2])> = input
        .iter()
        .enumerate()
        .map(|(y,line)| {
            line
                .chars()
                .enumerate()
                .filter(|&(_,c)| c != '#' && c != '.' )
                .map(|(x, c)| {
                    ([x as i64, y as i64], match c {
                        '>' => [ 1, 0],
                        '<' => [-1, 0],
                        'v' => [ 0, 1],
                        '^' => [ 0,-1],
                        _ => panic!()
                    })})
                .collect::<Vec<([i64;2],[i64;2])>>() })
        .collect::<Vec<Vec<([i64;2],[i64;2])>>>()
        .iter()
        .flatten()
        .map(|&t| t)
        .collect::<Vec<([i64;2],[i64;2])>>();

    let mut history: HashMap<i64, Vec<[i64;2]>> = HashMap::new();
    let update_blizzards = | t: i64, blzds: &Vec<([i64;2],[i64;2])>, history: &mut HashMap<i64, Vec<[i64;2]>> | -> Vec<[i64;2]> {
        let t_rep = t % period;
        if history.contains_key(&t_rep) {
            return history[&t_rep].clone();
        } else {
            let v: Vec<[i64;2]> = blzds.iter().map(|blzd| { [ 
                (blzd.0[0] + blzd.1[0] * t_rep - 1).rem_euclid( max[0] - 1) + 1,
                (blzd.0[1] + blzd.1[1] * t_rep - 1).rem_euclid( max[1] - 1) + 1
            ]}).collect();
            history.insert(t_rep,v.clone());
            v
        }
    };

    let colors = blizzards.iter().map(|b| match b.1 {
        [-1, 0] => (255,255,0,255),
        [ 1, 0] => (255,0,255,255),
        [ 0,-1] => (0,255,255,255),
        [ 0, 1] => (100,100,100,255),
        _ => panic!(),
    }).collect::<Vec<(u8,u8,u8,u8)>>();

    let inside = | p: [i64;2] | -> bool { 
        (0..2).all(|i| p[i] > 0 && p[i] < max[i]) || p == start || p == goal
    };

    let directions = [ [-1i64, 0i64], [ 1i64, 0i64], [0i64, -1i64], [0i64, 1i64], [0i64, 0i64] ];

    let find = | start_pos: [i64;2], end_pos: [i64;2], time_offset: i64, history: &mut HashMap<i64, Vec<[i64;2]>>, paths: &mut HashMap<(i64,[i64;2]), (i64,[i64;2])> | -> i64 {
        let state = (time_offset, start_pos);
        let mut pq = PriorityQueue::new();
        pq.push( state, Reverse(time_offset) );
        while let Some(((t,pos), _prio)) = pq.pop() {
            if pos == end_pos { return t; }
            let positions = update_blizzards(t+1, &blizzards, history);
            for dir in &directions {
                let next_pos = [pos[0] + dir[0], pos[1] + dir[1]];
                if inside(next_pos) && !positions.contains(&next_pos) {
                    let state = (t+1, next_pos);
                    pq.push(state, Reverse(t+1));
                    paths.insert((t+1,next_pos), (t,pos));
                }
            }
        }
        0
    };

    let after0 = Instant::now();

    let start1 = Instant::now();

    let mut paths: HashMap<(i64,[i64;2]), (i64,[i64;2])> = HashMap::new();
    let res1 = find(start,goal,0, &mut history, &mut paths);

    let after1 = Instant::now();
    if print_result {
        println!("Part 1: {}", res1);
    }

    let start2 = Instant::now();

    let res2 = find(start,goal, find(goal,start,res1, &mut history, &mut paths), &mut history, &mut paths);

    let mut current = (res2, goal);
    let mut path: Vec<(i64,[i64;2])> = vec![current];
    while let Some(p) = paths.get(&current) {
        path.insert(0,*p);
        current = *p;
    }

    let mut counter = 0;
    let mut img = Image::new(max[0] as usize + 1, max[1] as usize + 1, 8);

    let clear = | img: &mut Image | {
        img.clear((250,250,250,255));
        (0..=max[0]).for_each(|x| { img.set_pixel(x as usize, 0, (0,0,0,255)); img.set_pixel(x as usize, max[1] as usize, (0,0,0,255)); } );
        (0..=max[1]).for_each(|y| { img.set_pixel(0, y as usize, (0,0,0,255)); img.set_pixel(max[0] as usize, y as usize, (0,0,0,255)); } );
        img.set_pixel( start[0] as usize, start[1] as usize, (255,0,0,255));
        img.set_pixel( goal[0] as usize, goal[1] as usize, (0,255,0,255));
    };

    let mut draw_scene = | img: &mut Image, path_element: &(i64, [i64;2]), path: &Vec<(i64, [i64;2])>, counter: &mut usize | {
        let bs = update_blizzards(path_element.0, &blizzards, &mut history);
        bs.iter().enumerate().for_each(|(i,p)| {
            img.set_pixel( p[0] as usize, p[1] as usize, colors[i]);            
        });

        path.iter().filter(|&(t,_)| *t < path_element.0).for_each(|&(_,pos)| {
            img.set_pixel( pos[0] as usize, pos[1] as usize, (10,10,10,100));            
        });

        img.set_pixel( path_element.1[0] as usize, path_element.1[1] as usize, (20,20,20,255));            
        img.save_png(&format!("images/day24_22/blizzards_{:05}.png", counter)); *counter += 1;    
    };

    path.iter().for_each(|pe| {
        clear(&mut img); draw_scene(&mut img, pe, &path, &mut counter);
    });

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

use std::collections::HashMap;

use glam::UVec2;

pub fn process<'a>(
    input: &'a str,
) -> u32 {

    let dimension = UVec2{ y: input.lines().count() as u32,
        x: input.lines().next().unwrap().len() as u32};

    let state: HashMap<UVec2, char> = input.lines().enumerate().map(|(y, line)| 
        line.chars().enumerate().filter_map(
            move |(x, c)| match c { 
                '.' => None,
                 _ => Some((UVec2{x: x.try_into().unwrap(), y: y.try_into().unwrap()}, c))
            } 
        ) 
    ).flatten().collect();

    let mut states = vec![];
    let mut loads = vec![];

    let mut new_state = state;
    let mut new_load: u32 = 1;
    
    // display_state(&dimension, &new_state);
    while ! states.contains(&new_state) { 
        states.push(new_state.clone());
        loads.push(new_load);
    // while old_load != new_load {
        new_state = tilt_north(dimension, &new_state);
        // display_state(&dimension, &new_state);
        new_state.keys().filter(|loc| loc.x >= dimension.x || loc.y >= dimension.y).inspect(|x| {dbg!(x);}).for_each(drop);
        assert!(new_state.keys().all(|loc| loc.x < dimension.x && loc.y < dimension.y));
        
        new_state = tilt_west(dimension, &new_state);
        // display_state(&dimension, &new_state);
        new_state.keys().filter(|loc| loc.x >= dimension.x || loc.y >= dimension.y).inspect(|x| {dbg!(x);}).for_each(drop);
        assert!(new_state.keys().all(|loc| loc.x < dimension.x && loc.y < dimension.y));
        
        new_state = tilt_south(dimension, &new_state);
        // display_state(&dimension, &new_state);
        new_state.keys().filter(|loc| loc.x >= dimension.x || loc.y >= dimension.y).inspect(|x| {dbg!(x);}).for_each(drop);
        assert!(new_state.keys().all(|loc| loc.x < dimension.x && loc.y < dimension.y));
        
        new_state = tilt_east(dimension, &new_state);
        // display_state(&dimension, &new_state);
        new_state.keys().filter(|loc| loc.x >= dimension.x || loc.y >= dimension.y).inspect(|x| {dbg!(x);}).for_each(drop);
        assert!(new_state.keys().all(|loc| loc.x < dimension.x && loc.y < dimension.y));
        
        // display_state(&dimension, &new_state);
        
        new_load = calculate_load(dimension, &new_state);
        println!("Load: {}",new_load);
    }
    let loop_start_point = states.iter().position(|state| *state == new_state).expect("Exiting the above while loop guarantees a result");
    dbg!(loop_start_point);
    let loop_length = states.iter().count() - loop_start_point;
    dbg!(loop_length);
    let result_position = (1000000000 - loop_start_point) % loop_length;
    dbg!(loads[dbg!(result_position + loop_start_point)])
}

fn calculate_load(dimension: UVec2, state: &HashMap<UVec2, char>) -> u32 {
    state.into_iter().filter_map(|(loc, c)| match c {
        'O' => Some(dimension.y - loc.y),
        _ => None
    }).sum()
}

fn tilt_north(dimension: UVec2, state: &HashMap<UVec2, char>) -> HashMap<UVec2, char> {
    let mut new_state = HashMap::<UVec2, char>::new();
    for x in 0..dimension.x {
        let mut last_blocked_position: Option<u32> = None;
        for y in 0..dimension.y {
            let location = UVec2{x, y};
            match state.get(&location) {
                None => continue,
                Some('#') => {
                    last_blocked_position = Some(location.y);
                    new_state.insert(location, '#');
                },
                Some('O') => {
                    last_blocked_position = last_blocked_position.map_or(Some(0), |y| Some(y + 1) );
                    let new_loc = UVec2{x: location.x, y: last_blocked_position.expect("Previous line guaranteers a value") };
                    if new_loc.x >= dimension.x || new_loc.y >= dimension.y {
                        panic!("{:?} : {} => {}", last_blocked_position, location, new_loc);
                    } 
                    new_state.insert(new_loc, 'O');
                },
                Some(value) => panic!("Invalid value recieved: {}", value)
            }
        }
    }
    new_state
}

fn tilt_west(dimension: UVec2, state: &HashMap<UVec2, char>) -> HashMap<UVec2, char> {
    let mut new_state = HashMap::<UVec2, char>::new();
    for y in 0..dimension.y {
        let mut last_blocked_position: Option<u32> = None;
        for x in 0..dimension.x {
            let location = UVec2{x, y};
            match state.get(&location) {
                None => continue,
                Some('#') => {
                    last_blocked_position = Some(location.x);
                    new_state.insert(location, '#');
                },
                Some('O') => {
                    last_blocked_position = last_blocked_position.map_or(Some(0), |x| Some(x + 1) );
                    new_state.insert(UVec2{x: last_blocked_position.expect("Previous line guaranteers a value"), y: location.y}, 'O');
                },
                Some(value) => panic!("Invalid value recieved: {}", value)
            }
        }
    }
    new_state
}

fn tilt_south(dimension: UVec2, state: &HashMap<UVec2, char>) -> HashMap<UVec2, char> {
    let mut new_state = HashMap::<UVec2, char>::new();
    for x in 0..dimension.x {
        let mut last_blocked_position: Option<u32> = None;
        for y in (0..dimension.y).rev() {
            let location = UVec2{x, y};
            match state.get(&location) {
                None => continue,
                Some('#') => {
                    last_blocked_position = Some(location.y);
                    new_state.insert(location, '#');
                },
                Some('O') => {
                    last_blocked_position = last_blocked_position.map_or(Some(dimension.y - 1), |y| Some(y - 1) );
                    new_state.insert(UVec2{x: location.x, y: last_blocked_position.expect("Previous line guaranteers a value") }, 'O');
                },
                Some(value) => panic!("Invalid value recieved: {}", value)
            }
        }
    }
    new_state
}

fn tilt_east(dimension: UVec2, state: &HashMap<UVec2, char>) -> HashMap<UVec2, char> {
    let mut new_state = HashMap::<UVec2, char>::new();
    for y in 0..dimension.y {
        let mut last_blocked_position: Option<u32> = None;
        for x in (0..dimension.x).rev() {
            let location = UVec2{x, y};
            match state.get(&location) {
                None => continue,
                Some('#') => {
                    last_blocked_position = Some(location.x);
                    new_state.insert(location, '#');
                },
                Some('O') => {
                    last_blocked_position = last_blocked_position.map_or(Some(dimension.x - 1), |x| Some(x - 1) );
                    new_state.insert(UVec2{x: last_blocked_position.expect("Previous line guaranteers a value"), y: location.y }, 'O');
                },
                Some(value) => panic!("Invalid value recieved: {}", value)
            }
        }
    }
    new_state
}

fn display_state(dimension: &UVec2, state: &HashMap<UVec2, char>) {
    for y in 0..dimension.y {
        for x in 0..dimension.x {
            match state.get(&UVec2{x, y}) {
                None => print!("."),
                Some(value) => print!("{}", value)
            }
        }
        print!("\n");
    }
    print!("\n");
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_process() {
        let lines = "O....#....
O.OO#....#
.....##...
OO.#O....O
.O.....O#.
O.#..O.#.#
..O..#O..O
.......O..
#....###..
#OO..#....";
        let result = process(lines);
        assert_eq!(result, 64);
    }
}

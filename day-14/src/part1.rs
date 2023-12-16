use std::collections::HashMap;

use glam::UVec2;

pub fn process<'a>(
    input: &'a str,
) -> u32 {

    let length = input.lines().count() as u32;
    let width = input.lines().next().unwrap().len() as u32;

    let platform: HashMap<UVec2, char> = input.lines().enumerate().map(|(y, line)| 
        line.chars().enumerate().filter_map(
            move |(x, c)| match c { 
                '.' => None,
                 _ => Some((UVec2{x: x.try_into().unwrap(), y: y.try_into().unwrap()}, c))
            } 
        ) 
    ).flatten().collect();


    let mut load = 0;    
    for x in 0..width {
        let mut most_recent_cube: Option<UVec2> = None;
        let mut number_of_rounds_since_last_cube = 0;
        for y in 0..length {
            let location = UVec2{x, y};
            match platform.get(&location) {
                None => continue,
                Some('#') => {
                    increment_load(most_recent_cube, length, &mut load, number_of_rounds_since_last_cube);
                    most_recent_cube = Some(location);
                    number_of_rounds_since_last_cube = 0;
                },
                Some('O') => {
                    number_of_rounds_since_last_cube += 1;
                },
                Some(value) => panic!("Invalid value recieved: {}", value)
            }
        }
        increment_load(most_recent_cube, length, &mut load, number_of_rounds_since_last_cube);
    }

    load
}

fn increment_load(most_recent_cube: Option<UVec2>, length: u32, load: &mut u32, number_of_rounds_since_last_cube: u32) {
    let cube_load: u32;
    if most_recent_cube.is_none() {
        cube_load = length + 1;
    } else {
        cube_load = length - most_recent_cube.expect("other half of if statment deals with None").y
    }
    *load += consecutive_sum(cube_load, number_of_rounds_since_last_cube);
}

/// sums the n consecutive digits which are strictly less than end 
fn consecutive_sum(end: u32, n: u32) -> u32{
    let start = end - n;
    ( end * ( end - 1 ) / 2 ) - ( start * (start - 1) / 2 )
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_consecutive_sum() {
        assert_eq!(consecutive_sum(10, 2), 9 + 8)
    }

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
        assert_eq!(result, 136);
    }
}

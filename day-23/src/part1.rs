use std::collections::HashMap;

use glam::IVec2;

pub fn process<'a>(
    input: &'a str,
) -> i32 {
    let start = IVec2::new(input.lines().next().unwrap().chars().enumerate().find(|(_, c)| *c == '.').unwrap().0 as i32, 0);

    let island: HashMap<IVec2, TileType> = garden(input).filter_map(|(location, c)| match c {
        '.' => Some((location, TileType::Path)),
        '^' => Some((location, TileType::Slope(Direction::North))),
        'v' => Some((location, TileType::Slope(Direction::South))),
        '>' => Some((location, TileType::Slope(Direction::East ))),
        '<' => Some((location, TileType::Slope(Direction::West ))),
        _ => None
    }).collect();
    
    longest_hike(&start, &island.iter().map(|(k, v)| (k, *v)).collect())
}

fn garden(input: &str) -> impl Iterator<Item = (IVec2, char)> + '_{
    input.lines().enumerate().flat_map(|(y, line)|{
        line.chars().enumerate().map(move |(x, c)|{
            (IVec2{x: x.try_into().unwrap(), y: y.try_into().unwrap()}, c)
        })
    })
}

fn longest_hike<'a>(location: &IVec2, island: &'a HashMap<&'a IVec2, TileType>) -> i32 {
    if let Some(neighbours) = neighbours(location, island) {
        let mut new_island = island.clone();
        new_island.remove(location);
        neighbours.iter()
            .map(|neighbour| 1 + longest_hike(neighbour, &new_island) )
            .max().expect("there should be a maximum")
    } else {
        0
    }
    
}

fn neighbours<'a>(location: &'a IVec2, island: &'a HashMap<&'a IVec2, TileType> ) -> Option<Vec<IVec2>> {
    let mut retvec = vec![];
    match island.get(location).expect("Location should exist") {
        TileType::Path => {
            retvec.push(*location + IVec2::X);
            retvec.push(*location + IVec2::Y);
            retvec.push(*location - IVec2::X);
            retvec.push(*location - IVec2::Y);
        }
        TileType::Slope(direction) => 
            match direction { 
                Direction::South => retvec.push(*location + IVec2::Y),
                Direction::North => retvec.push(*location - IVec2::Y),
                Direction::East  => retvec.push(*location + IVec2::X),
                Direction::West  => retvec.push(*location - IVec2::X),
            }
        }
    let retvec: Vec<_> = retvec.into_iter().filter(|location| {island.contains_key(location)}).collect();
    if retvec.is_empty() { None } else { Some(retvec) }
}

#[derive(Debug, Eq, PartialEq, Clone, Copy)]
enum TileType {
    Path,
    Slope(Direction)
}

#[derive(Debug, Eq, PartialEq, Clone, Copy)]
enum Direction {
    North,
    South,
    East,
    West
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_process() {
        let lines = "#.#####################
#.......#########...###
#######.#########.#.###
###.....#.>.>.###.#.###
###v#####.#v#.###.#.###
###.>...#.#.#.....#...#
###v###.#.#.#########.#
###...#.#.#.......#...#
#####.#.#.#######.#.###
#.....#.#.#.......#...#
#.#####.#.#.#########v#
#.#...#...#...###...>.#
#.#.#v#######v###.###v#
#...#.>.#...>.>.#.###.#
#####v#.#.###v#.#.###.#
#.....#...#...#.#.#...#
#.#########.###.#.#.###
#...###...#...#...#.###
###.###.#.###v#####v###
#...#...#.#.>.>.#.>.###
#.###.###.#.###.#.#v###
#.....###...###...#...#
#####################.#";
        let result = process(lines);
        assert_eq!(result, 94);
    }
}

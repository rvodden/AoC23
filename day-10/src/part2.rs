use std::collections::HashMap;

use glam::IVec2;

const NORTH: &IVec2 = &IVec2{x:  0, y: -1};
const SOUTH: &IVec2 = &IVec2{x:  0, y:  1};
const EAST:  &IVec2 = &IVec2{x:  1, y:  0};
const WEST:  &IVec2 = &IVec2{x: -1, y:  0};

fn symbol_to_exit(c: &char) -> Vec<&IVec2> {
    match c {
        '|' => vec![ NORTH, SOUTH ],
        '-' => vec![  EAST,  WEST ],
        'L' => vec![ NORTH,  EAST ],
        'J' => vec![ NORTH,  WEST ],
        '7' => vec![ SOUTH,  WEST ],
        'F' => vec![ SOUTH,  EAST ],
        'S' => vec![ NORTH, SOUTH, EAST, WEST ],
        '.' => vec![],
        value => panic!("Found invalid character: '{}'", value)
    }
}

fn exits<'a>(location: &'a IVec2, symbol: &'a char) -> Vec<IVec2> {
    symbol_to_exit(symbol).iter().map(|delta| (**delta + *location) ).collect()
}

pub fn process<'a>(
    input: &'a str,
) -> u32 {

    let nodes: HashMap<IVec2, Vec<IVec2>> = input.lines()
        .enumerate()
        .flat_map({ |(y, line)| 
            line.chars()
                .enumerate()
                .map(move |(x, c)| {
                    let x: i32 = x.try_into().unwrap();
                    let y: i32 = y.try_into().unwrap();
                    let location = IVec2{x, y};
                    (location, exits(&location, &c))
                })
        })
        .collect();

    let start_node: IVec2 = *nodes.iter().find_map(|(location, exits)| if exits.len() == 4 {Some(location)} else {None} ).expect("there should be a start node");
    dbg!(&start_node);
    
    let path = nodes[&start_node].iter()
        .filter(|x| nodes.contains_key(&x) )
        .flat_map(|next_node| {
        if nodes[next_node].contains(&start_node) {
            find_loop(vec![start_node], *next_node, &start_node, &nodes)
        } else {
            None
        }
    }).next().expect("There should be a loop");

    dbg!(&path);

    let pairs_of_points = path.iter().skip(1).zip(path.iter().skip(2));
    let area_including_perimeter = pairs_of_points.fold(0,
        |area: i32, (a, b): (&IVec2, &IVec2)| {
            let vec1 = *a - start_node;
            let vec2 = *b - start_node;
            area + vec2.perp_dot(vec1)
        }).abs() as u32;

    ( area_including_perimeter - (path.len() as u32) + 1 ) / 2
}

fn find_loop(visited_nodes: Vec<IVec2>, node: IVec2, start_node: &IVec2, nodes: &HashMap<IVec2, Vec<IVec2>> ) -> Option<Vec<IVec2>> {
    // if we're back at the start then we're done
    if nodes[&node].contains(start_node) && visited_nodes.len() > 2 { return Some(visited_nodes) };
    
    let next_nodes = nodes[&node].iter().filter(|n| ! visited_nodes.contains(n) ).collect::<Vec<&IVec2>>();
    // if there's no-where else to go, then we're done;
    if next_nodes.len() == 0 { return None };


    let mut new_visited_nodes = visited_nodes.clone();
    new_visited_nodes.push(node);

    for next_node in next_nodes {
        let result = find_loop(new_visited_nodes.clone(), *next_node, start_node, nodes);
        if result.is_some() { return result };
    }
    None
}

#[cfg(test)]
mod tests {
    use super::*;

    use rstest::rstest;

    #[rstest]
    #[case("...........
.S-------7.
.|F-----7|.
.||.....||.
.||.....||.
.|L-7.F-J|.
.|..|.|..|.
.L--J.L--J.
...........", 4)]
    #[case("...........
.S-------7.
.|F-----7|.
.||.....||.
.||.....||.
.|L-7.F-J|.
.|..|.|..|.
.L--J.L--J.
...........",4)]
    #[case(".F----7F7F7F7F-7....
.|F--7||||||||FJ....
.||.FJ||||||||L7....
FJL7L7LJLJ||LJ.L-7..
L--J.L7...LJS7F-7L7.
....F-J..F7FJ|L7L7L7
....L7.F7||L7|.L7L7|
.....|FJLJ|FJ|F7|.LJ
....FJL-7.||.||||...
....L---J.LJ.LJLJ...",8)]
    #[case("FF7FSF7F7F7F7F7F---7
L|LJ||||||||||||F--J
FL-7LJLJ||||||LJL-77
F--JF--7||LJLJ7F7FJ-
L---JF-JLJ.||-FJLJJ7
|F|F-JF---7F7-L7L|7|
|FFJF7L7F-JF7|JL---7
7-L-JL7||F7|L7F-7F7|
L.L7LFJ|||||FJL7||LJ
L7JLJL-JLJLJL--JLJ.L", 10)]
    fn test_process(
        #[case] input: &str,
        #[case] expected: u32,
    ) {
        let result = process(input);
        assert_eq!(result, expected);
    }
}

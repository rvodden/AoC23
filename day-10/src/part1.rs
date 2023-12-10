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
        .map({ |(y, line)| 
            line.chars()
                .enumerate()
                .map(move |(x, c)| {
                    let x: i32 = x.try_into().unwrap();
                    let y: i32 = y.try_into().unwrap();
                    let location = IVec2{x, y};
                    (location, exits(&location, &c))
                })
        })
        .flatten()
        .collect();

    let start_node: IVec2 = *nodes.iter().find_map(|(location, exits)| if exits.len() == 4 {Some(location)} else {None} ).expect("there should be a start node");
    dbg!(&start_node);
    
    (nodes[&start_node].iter().map(|next_node| {
        // dbg!(&next_node);
        if nodes[next_node].contains(&start_node) {
            find_loop(vec![start_node], *next_node, &start_node, &nodes)
        } else {
            None
        }
    })
    .inspect(|x| {dbg!(x); })
    .flatten().next().unwrap().len() as u32 + 1) / 2
}

fn find_loop(visited_nodes: Vec<IVec2>, node: IVec2, start_node: &IVec2, nodes: &HashMap<IVec2, Vec<IVec2>> ) -> Option<Vec<IVec2>> {
    // if we're back at the start then we're done
    if nodes[&node].contains(start_node) && visited_nodes.len() > 2 { return Some(visited_nodes) };
    
    let next_nodes = nodes[&node].iter().filter(|n| ! visited_nodes.contains(n) ).collect::<Vec<&IVec2>>();
    // if there's no-where else to go, then we're done;
    if next_nodes.len() == 0 { return None };

    // dbg!(&next_nodes);

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
    #[case(".....
.S-7.
.|.|.
.L-J.
.....
", 4)]
    #[case("..F7.
.FJ|.
SJ.L7
|F--J
LJ...",8)]
    fn test_process(
        #[case] input: &str,
        #[case] expected: u32,
    ) {
        let result = process(input);
        assert_eq!(result, expected);
    }
}

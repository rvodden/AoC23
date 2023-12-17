use std::collections::{HashMap, HashSet};

use glam::IVec2;

pub fn process<'a>(
    input: &'a str,
) -> u32 {
    let grid = input.lines().zip(0..).map(|(line, y)|
        line.chars().zip(0..).map( |(char, x)| {
            use MirrorOrientation::*;
            use SplitterOrientation::*;
            use CellType::*;
            let position = IVec2{x,y};
            match char {
                '.' => (position, EmptySpace),
                '\\' => (position, Mirror(TopLeft)),
                '/' => (position, Mirror(TopRight)),
                '-' => (position, Splitter(EastWest)),
                '|' => (position, Splitter(NorthSouth)),
                value => panic!("Received invalid character {}", value)
            }
        }).collect::<Vec<_>>()
        ).flatten().collect::<HashMap<_, _>>();


    get_energized_cells(&grid, State{ position: IVec2::ZERO, direction: EAST })
}

fn get_energized_cells(grid: &HashMap<IVec2, CellType>, state: State) -> u32 {
    let mut steps = HashSet::<State>::new();

    let mut states: Vec<State> = vec![state];
    while states.len() > 0 {
        steps.extend(states);
        let new_states = steps.iter().filter_map(|state| 
            step(*state, &grid).and_then(|ss| Some(ss.filter(|s| ! steps.contains(s))))
        ).flatten().collect();
        states = new_states;
    }

    let positions = HashSet::<IVec2>::from_iter(steps.into_iter().filter_map(
        |state| {
            grid.get(&state.position)?;
            Some(state.position)
        }
    ));
    positions.into_iter().count() as u32
}

fn step(state: State, grid: &HashMap<IVec2, CellType>) -> Option<Box<dyn Iterator<Item = State>>> {
    use MirrorOrientation::*;
    use SplitterOrientation::*;
    use CellType::*;
    match grid.get(&state.position)? {
        EmptySpace => {
            move_forward(state)
        },
        Mirror(TopLeft) => match state.direction {
            NORTH => turn(WEST, state),
            SOUTH => turn(EAST, state),
            EAST  => turn(SOUTH, state),
            WEST  => turn(NORTH, state),
            value => panic!("Invalid direction {}", value),
        },
        Mirror(TopRight) => match state.direction {
            NORTH => turn(EAST, state),
            SOUTH => turn(WEST, state),
            EAST  => turn(NORTH, state),
            WEST  => turn(SOUTH, state),
            value => panic!("Invalid direction {}", value),
        },
        Splitter(EastWest) => match state.direction {
            NORTH => turn_east_and_west(state),
            SOUTH => turn_east_and_west(state), 
            EAST | WEST => move_forward(state),
            value => panic!("Invalid direction {}", value),
        },
        Splitter(NorthSouth) => match state.direction {
            EAST => turn_north_and_south(state),
            WEST => turn_north_and_south(state),
            NORTH | SOUTH => move_forward(state),
            value => panic!("Invalid direction {}", value),
        },
    }
}

fn turn_east_and_west(state: State) -> Option<Box<dyn Iterator<Item = State>>> {
    Some(Box::new(turn(EAST, state)?.chain(turn(WEST, state)?)))
}

fn turn_north_and_south(state: State) -> Option<Box<dyn Iterator<Item = State>>> {
    Some(Box::new(turn(NORTH, state)?.chain(turn(SOUTH, state)?)))
}

fn turn(direction: IVec2, state: State) -> Option<Box<dyn Iterator <Item = State>>> {
    let mut new_state = state;
    new_state.direction = direction;
    new_state.position += new_state.direction;
    Some(Box::new(vec![new_state].into_iter()))
}

fn move_forward(state: State) -> Option<Box<dyn Iterator <Item = State>>> {
    let mut new_state = state;
    new_state.position += new_state.direction;
    Some(Box::new(vec![new_state].into_iter()))
}

const NORTH: IVec2 = IVec2{x:  0, y: -1};
const SOUTH: IVec2 = IVec2{x:  0, y:  1};
const EAST:  IVec2 = IVec2{x:  1, y:  0};
const WEST:  IVec2 = IVec2{x: -1, y:  0};

#[derive(Debug, Copy, Clone, Eq, PartialEq, Hash)]
struct State {
    position: IVec2,
    direction: IVec2
}

enum SplitterOrientation {
    NorthSouth,
    EastWest,
}

enum MirrorOrientation {
    TopLeft,
    TopRight
}

enum CellType {
    EmptySpace,
    Mirror(MirrorOrientation),
    Splitter(SplitterOrientation)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_process() {
        let lines = r".|...\....
|.-.\.....
.....|-...
........|.
..........
.........\
..../.\\..
.-.-/..|..
.|....-|.\
..//.|....";
        let result = process(lines);
        assert_eq!(result, 46);
    }
}

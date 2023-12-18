use std::collections::{HashMap, BinaryHeap};

use glam::IVec2;

pub fn process<'a>(input: &'a str) -> u32 {
    let grid = input
        .lines()
        .zip(0..)
        .map(|(line, y)| {
            line.chars()
                .zip(0..)
                .map(|(char, x)| (IVec2 { x, y }, char.to_digit(10).unwrap()))
                .collect::<Vec<_>>()
        })
        .flatten()
        .collect::<HashMap<IVec2, u32>>();

    let dimension = IVec2 {
        x: input.lines().next().unwrap().len() as i32,
        y: input.lines().count() as i32,
    };
    let target = dimension - IVec2 { x: 1, y: 1 };

    dijkstra(&IVec2::ZERO, &target, &grid).unwrap()
}

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
struct Node {
    position: IVec2,
    direction: IVec2,
    run_length: u32,
}

impl Node {
    fn new(position: IVec2, direction: IVec2, run_length: u32) -> Self {
        Self {
            position,
            direction,
            run_length,
        }
    }

    fn r#move(&self, direction: &IVec2) -> Option<Self> {
        // we can't go back on ourselves
        if *direction == - self.direction {
            return None
        }

        let mut new_node = self.clone();
        new_node.direction = *direction;
        new_node.position = self.position + *direction;
        
        if *direction == self.direction {
            if self.run_length < 3 {
                new_node.run_length += 1;
            } else {
                return None;
            }
        } else {
            new_node.run_length = 1;
        }
        Some(new_node)
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
struct State {
    node: Node,
    heat_loss: u32,
}

impl Ord for State {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        other.heat_loss.cmp(&self.heat_loss)
    }
}

impl PartialOrd for State {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        Some(self.cmp(other))
    }
}

fn dijkstra(
    start: &IVec2,
    target: &IVec2,
    grid: &HashMap<IVec2, u32>
) -> Option<u32> {
    let mut heat_losses = HashMap::<Node, u32>::from([
        (Node::new(start.clone(), IVec2::X, 0), 0),
        (Node::new(start.clone(), IVec2::Y, 0), 0),
    ]);
    let mut back_steps = HashMap::<Node, Node>::new();
    let mut frontier = BinaryHeap::<State>::new();
    for (node, _) in heat_losses.iter() {
        frontier.push(
            State{ 
                heat_loss: 0, 
                node: node.clone()
            });
    }

    while let Some(State{node, heat_loss}) = frontier.pop() {
        if node.position == *target {
            // println!("Done!");
            // display_path(&back_steps, target);
            return Some(heat_loss) 
        };

        for next_node in get_next_nodes(&node, &grid) {
            if back_steps.contains_key(&next_node) { continue };
            let heat_loss_at_next_node = heat_loss + grid[&next_node.position];

            // if we can already get to this neighbour faster, then give up on this route
            if let Some(existing_heat_loss) = heat_losses.get(&next_node) {
                if heat_loss_at_next_node >= *existing_heat_loss {
                    continue;
                }
            }

            // otherwise 
            heat_losses.insert(node.clone(), heat_loss_at_next_node);
            frontier.push(State {
                heat_loss: heat_loss_at_next_node,
                node: next_node.clone()
            });
            back_steps.insert(next_node, node.clone());
        }
        // dbg!(&frontier);
    }
    None
}

fn get_next_nodes<'a>(node: &'a Node, grid: &'a HashMap<IVec2, u32>) -> impl Iterator<Item = Node> + 'a {
    vec![
        node.r#move(&IVec2::X),
        node.r#move(&IVec2::Y),
        node.r#move(&IVec2::NEG_X),
        node.r#move(&IVec2::NEG_Y),
    ]
    .into_iter()
    .filter_map(|x| {
        if grid.contains_key(&x.clone()?.position) {
            x
        } else {
            None
        }
    })
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_process() {
        let lines = "2413432311323
3215453535623
3255245654254
3446585845452
4546657867536
1438598798454
4457876987766
3637877979653
4654967986887
4564679986453
1224686865563
2546548887735
4322674655533";
        let result = process(lines);
        assert_eq!(result, 102);
    }
}

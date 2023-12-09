use std::collections::BTreeMap;
use nom::{sequence::{separated_pair, delimited}, character::complete::alphanumeric1, bytes::complete::tag, IResult};

#[derive(PartialEq, Eq, Debug, Clone, Copy)]
struct Node<'a> {
    left: &'a str,
    right: &'a str
}

#[derive(PartialEq, Eq, Debug)]
struct Nodes<'a> {
    nodes: BTreeMap::<&'a str, Node<'a>>,
}

impl<'a> Nodes<'a> {
    fn new(input: impl Iterator<Item = &'a str>) -> Nodes<'a> {
        let mut nodes = Nodes{nodes: BTreeMap::<&str, Node>::new() };

        for l in input {
            let (_, (node, (left, right))) = line(l).expect("it should parse");
            nodes.nodes.insert(node, Node{left, right});
        }

        nodes
    }
    
    pub fn next_node(&'a self, node: &str, direction: char) -> &'a &'a str {
        match direction {
            'L' => &self.nodes[node].left,
            'R' => &self.nodes[node].right,
            value => unreachable!("This should not happen - recieved {}", value),
        }
    }

    pub fn start_nodes(&self) -> Vec<&&str> {
        self.nodes.keys().filter(|s| s.ends_with('A')).collect()
    }
}

pub fn process(
    input: &str,
) -> u64 {
    let mut lines = input.lines();
    let directions = lines.next().expect("there will be a line here");
    lines.next();
    let nodes = Nodes::new(lines);

    let start_nodes = nodes.start_nodes();
    // dbg!(&start_nodes);

    let cycle_lengths = start_nodes.into_iter()
        .map(|node| {
        let mut visited_nodes = vec![*node];
        let mut current_node = *node;

        directions
            .chars()
            .cycle()
            .enumerate()
            .find_map(|(index, direction)| {
                let next_node = nodes.next_node(current_node, direction);
                // println!("{} : {} = {}", current_node, direction, next_node);
                if next_node.ends_with('Z') {
                    Some((index + 1) as u64)
                } else {
                    visited_nodes.push(next_node);
                    current_node = next_node;
                    None
                }
            })
            .expect("a cycle to exist")
    })
    // .inspect(|x| { dbg!(x); })
    .collect::<Vec<u64>>();

    lowest_common_multiple(&cycle_lengths)
}

pub fn lowest_common_multiple(numbers: &[u64]) -> u64 {
    if numbers.len() == 1 {
        return numbers[0];
    }
    let a = numbers[0];
    let b = lowest_common_multiple(&numbers[1..]);
    a * b / gcd_of_two_numbers(a, b)
}

fn gcd_of_two_numbers(a: u64, b: u64) -> u64 {
    if b == 0 {
        return a;
    }
    gcd_of_two_numbers(b, a % b)
}

fn line(line: &str) -> IResult<&str, (&str, (&str, &str))> {
    separated_pair(
        alphanumeric1, 
        tag(" = "), 
        delimited(
            tag("("), 
            separated_pair(
                alphanumeric1, 
                tag(", "), 
                alphanumeric1
            ), 
            tag(")")
        ) 
    )(line)
}


#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_process() {
        let lines = "LR

11A = (11B, XXX)
11B = (XXX, 11Z)
11Z = (11B, XXX)
22A = (22B, XXX)
22B = (22C, 22C)
22C = (22Z, 22Z)
22Z = (22B, 22B)
XXX = (XXX, XXX)";
        let result = process(lines);
        assert_eq!(result, 6);
    }

}

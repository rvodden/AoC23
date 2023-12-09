use std::collections::BTreeMap;
use nom::{sequence::{separated_pair, delimited}, character::complete::alpha1, bytes::complete::tag, IResult};

#[derive(PartialEq, Eq, Debug)]
struct Node<'a> {
    left: &'a str,
    right: &'a str
}

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
    
    pub fn next_node(&self, node: &str, direction: &char) -> &'a str {
        match direction {
            'L' => self.nodes[node].left,
            'R' => self.nodes[node].right,
            value => panic!("This should not happen - recieved {}", value),
        }
    }
}

pub fn process(
    input: &str,
) -> u32 {
    let mut lines = input.lines();
    let directions = lines.next().expect("there will be a line here");
    lines.next();
    let nodes = Nodes::new(lines);


    directions.chars().cycle().scan("AAA", |node, direction| {
        if *node == "ZZZ" { return None };
        let next = nodes.next_node(node, &direction);
        *node = next;
        Some(next)
    })
    .count() as u32

}


fn line(line: &str) -> IResult<&str, (&str, (&str, &str))> {
    separated_pair(
        alpha1, 
        tag(" = "), 
        delimited(
            tag("("), 
            separated_pair(
                alpha1, 
                tag(", "), 
                alpha1
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
        let lines = "RL

AAA = (BBB, CCC)
BBB = (DDD, EEE)
CCC = (ZZZ, GGG)
DDD = (DDD, DDD)
EEE = (EEE, EEE)
GGG = (GGG, GGG)
ZZZ = (ZZZ, ZZZ)";
        let result = process(lines);
        assert_eq!(result, 2);
    }
   #[test]
    fn test_process_again() {
        let lines = "LLR

AAA = (BBB, BBB)
BBB = (AAA, ZZZ)
ZZZ = (ZZZ, ZZZ)";
        let result = process(lines);
        assert_eq!(result, 6);
    }

}

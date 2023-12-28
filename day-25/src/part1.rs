use std::collections::{HashSet, HashMap};

use petgraph::graph::UnGraph;

use nom::{sequence::separated_pair, multi::separated_list1, character::complete::{space1, alpha1, newline}, IResult, bytes::complete::tag};
use rustworkx_core::connectivity::stoer_wagner_min_cut;


pub fn process(
    input: &str,
) -> i32 {
    let (_, graph) = parse_input(input).expect("should parse!");
    let total_nodes = graph.node_count() as i32;
    let cut: rustworkx_core::Result<_> = stoer_wagner_min_cut(&graph, |_| Ok(1));
    let (_, nodes_in_partition) = cut.unwrap().unwrap();
    
    (total_nodes - nodes_in_partition.len() as i32) * (nodes_in_partition.len() as i32)
}

fn parse_input(input: &str) -> IResult<&str, UnGraph<&str,()>>{
    let (input, result) = separated_list1(newline, node)(input)?;
    let mut graph = UnGraph::default();

    // get a unique list of nodes
    let nodes: HashSet<&str> = result.iter().flat_map(|(node, adjacencies)| {
        let mut nodes = adjacencies.clone();
        nodes.push(node);
        nodes
    }).collect();
    
    // create the nodes in the graph
    let node_map: HashMap<_,_>= nodes.iter()
        .map(|node| (*node, graph.add_node(*node)))
        .collect();

    // add the edges
    for (node, adjacencies) in result {
        for adjacency in adjacencies {
            graph.add_edge(
                node_map[node],
                node_map[adjacency], 
                ());
        }
    }

    Ok((input, graph))
}

// cmg: qnr nvd lhk bvb
fn node(input: &str) -> IResult<&str, (&str, Vec<&str>)> {
    let (input, (node, adjacencies)) = separated_pair(alpha1, tag(": "), separated_list1(space1, alpha1))(input)?;
    Ok((input, (node, adjacencies.into_iter().collect())))
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_process() {
        let lines = "jqt: rhn xhk nvd
rsh: frs pzl lsr
xhk: hfx
cmg: qnr nvd lhk bvb
rhn: xhk bvb hfx
bvb: xhk hfx
pzl: lsr hfx nvd
qnr: nvd
ntq: jqt hfx bvb xhk
nvd: lhk
lsr: lhk
rzs: qnr cmg lsr rsh
frs: qnr lhk lsr";
        let result = process(lines);
        assert_eq!(result,54);
    }
}

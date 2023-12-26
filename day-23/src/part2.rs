use std::collections::{HashSet, HashMap};

use glam::IVec2;

pub fn process(
    input: &str,
) -> u32 {
    let start = IVec2::new(input.lines().next().unwrap().chars().enumerate().find(|(_, c)| *c == '.').unwrap().0 as i32, 0);
    let end = IVec2::new(input.lines().last().unwrap().chars().enumerate().find(|(_, c)| *c == '.').unwrap().0 as i32, input.lines().count() as i32 - 1);

    let island: HashSet<IVec2> = garden(input).filter_map(|(location, c)| match c {
        '.' | '^' | 'v' | '>' | '<' => Some(location),
        _ => None
    }).collect();

    let island_map: HashMap<&IVec2, Option<Vec<&IVec2>>> = island.iter()
        .map(|location| 
            (location, neighbours(location, &island))
        ).collect();
    
    dbg!(&island_map.len());
    
    let mut optimized_map: HashMap<&IVec2, HashSet<(&IVec2, u32)>> = HashMap::new();
    let mut to_visit = vec![&start];

    while let Some(location) = to_visit.pop() {
        if let Some(Some(neighbours)) = island_map.get(location) {
            let distant_neighbours = neighbours.iter().flat_map(|neighbour| get_next_forks(neighbour, &island_map, &end, HashSet::new()));
            optimized_map.insert(location, 
                distant_neighbours.clone()
                .filter(|x| x.0 != location )
                .collect());
            to_visit.extend(distant_neighbours.filter_map(|n| if !optimized_map.contains_key(n.0) {Some(n.0)} else {None} ));
        }
    }

    // render_map(optimized_map);
    longest_hike((&start, 0), &end, &optimized_map, HashSet::new()).expect("there to be a solution")
}

fn get_next_forks<'a>(location: &'a IVec2, island_map: &'a HashMap<&'a IVec2, Option<Vec<&IVec2>>>, end: &IVec2, mut visited: HashSet<&'a IVec2>) -> Vec<(&'a IVec2, u32)> {
    if location == end { return vec![(location, visited.len() as u32 + 1)] }
    if let Some(Some(nbrs)) = island_map.get(location) {
        if nbrs.len() > 2 { return vec![(location, visited.len() as u32 + 1)] };
        
        visited.insert(location);
        nbrs.iter().filter(|x| ! visited.contains(*x)).flat_map( |next_location| {
            get_next_forks(next_location, island_map, end, visited.clone()).into_iter()
        }).collect()
    } else {
        vec![]
    }
}

fn garden(input: &str) -> impl Iterator<Item = (IVec2, char)> + '_{
    input.lines().enumerate().flat_map(|(y, line)|{
        line.chars().enumerate().map(move |(x, c)|{
            (IVec2{x: x.try_into().unwrap(), y: y.try_into().unwrap()}, c)
        })
    })
}

fn longest_hike<'a>((location, cost): (&'a IVec2, u32), end: &IVec2, optimized_map: &HashMap<&IVec2, HashSet<(&IVec2, u32)>>, mut visited: HashSet<&'a IVec2>) -> Option<u32> {
    // println!("{}",location);

    if location == end { 
        return Some(cost)
    }

    if let Some(neighbours) = optimized_map.get(location) {
        visited.insert(location);
        let filtered_neighbours = neighbours.iter().filter(|(neighbour, _)| ! visited.contains(neighbour)).collect::<Vec<_>>();
        
        if filtered_neighbours.is_empty() {
            // println!("Neighours are empty!");
            return None 
        }; 

        filtered_neighbours.into_iter()
            .filter_map(|neighbour| longest_hike(*neighbour, end, optimized_map, visited.clone()) )
            .max().map(|m| m + cost)
    } else {
        None
    }
}

fn neighbours<'a>(location: &'a IVec2, island: &'a HashSet<IVec2> ) -> Option<Vec<&'a IVec2>> {
    let retvec: Vec<&IVec2> = [
        *location + IVec2::X,
        *location + IVec2::Y,
        *location - IVec2::X,
        *location - IVec2::Y,
    ]
        .iter()
        .filter_map(|location| 
            island.get(location)
        )
        .collect();
    (!retvec.is_empty()).then_some(retvec)
}

fn render_map( map: HashMap<&IVec2, HashSet<(&IVec2, u32)>> ) {
    println!("digraph {{");
    for (key, values) in map {   
        for (location, cost) in values {
            println!("\"{:}\" -> \"{:}\"[label=\"{}\"]", key, location, cost)
        }
    }
    println!("}}");
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
        assert_eq!(result, 154);
    }
}

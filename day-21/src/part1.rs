use std::{collections::HashSet, iter::from_fn};

use glam::IVec2;

pub fn process(
    input: &str, steps: usize
) -> usize {
    let mut start: Option<IVec2> = None;

    let garden_plots: HashSet<_> = garden(input).filter_map(|(location, c)| match c {
        '.' => Some(location),
        'S' => {
            start = Some(location);
            Some(location)
        }
        _ => None
    }).collect();


    let mut current_neighbours = HashSet::from([start.expect("There should be a start position.")]);
    from_fn(|| {
        current_neighbours = current_neighbours
            .iter()
            .flat_map(|neighbour| neighbours(neighbour, &garden_plots ).collect::<HashSet<_>>())
            // .inspect(|x| { dbg!(x); })
            .collect::<HashSet<_>>();
        Some(current_neighbours.len())
    }).take(steps).last().unwrap()
}

fn garden(input: &str) -> impl Iterator<Item = (IVec2, char)> + '_{
    input.lines().enumerate().flat_map(|(y, line)|{
        line.chars().enumerate().map(move |(x, c)|{
            (IVec2{x: x.try_into().unwrap(), y: y.try_into().unwrap()}, c)
        })
    })
}

fn neighbours<'a>(location: &'a IVec2, garden_plots: &'a HashSet<IVec2> ) -> impl Iterator<Item = IVec2> + 'a {
    vec![
        IVec2{x: location.x + 1, ..*location},
        IVec2{y: location.y + 1, ..*location},
        IVec2{x: location.x - 1, ..*location},
        IVec2{y: location.y - 1, ..*location},
    ].into_iter().filter(|location| {garden_plots.contains(location)})
}


#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_process() {
        let lines = "...........
.....###.#.
.###.##..#.
..#.#...#..
....#.#....
.##..S####.
.##..#...#.
.......##..
.##.#.####.
.##..##.##.
...........";
        let result = process(lines, 6);
        assert_eq!(result, 16);
    }
}

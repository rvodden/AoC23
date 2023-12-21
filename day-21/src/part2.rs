use std::collections::HashSet;

use glam::IVec2;

pub fn process(input: &str, steps: i32) -> i64 {
    let dimension = IVec2 {
        y: input.lines().count() as i32,
        x: input.lines().next().unwrap().len() as i32,
    };

    let mut start: Option<IVec2> = None;

    let garden_plots: HashSet<_> = garden(input)
        .filter_map(|(location, c)| match c {
            '.' => Some(location),
            'S' => {
                start = Some(location);
                Some(location)
            }
            _ => None,
        })
        .collect();

    let mut points = vec![];
    let mut current_neighbours: Box<dyn Iterator<Item = IVec2>> = Box::new(
        vec![start.expect("should be a start node")]
            .into_iter()
            .flat_map(|neighbour| neighbours(neighbour, &garden_plots, &dimension)),
    );
    let mut num_points = 0;
    
    for i in 2..steps {
        current_neighbours = Box::new(current_neighbours
            .flat_map(|neighbour| neighbours(neighbour, &garden_plots, &dimension))
            .collect::<HashSet<_>>()
            .into_iter()
            // .inspect(|x| { print!("{}", x); } )
        );
        if i.rem_euclid(dimension.y) == steps.rem_euclid(dimension.y) {
            let current_neighbours_vec = current_neighbours.collect::<Vec<_>>();
            points.push(
                IVec2{
                    x: i, 
                    y: current_neighbours_vec.len() as i32
                }
            );
            current_neighbours = Box::new(current_neighbours_vec.into_iter());
            num_points += 1;
            if num_points == 3 {
                break;
            }
        }
    }
    dbg!(&points);
    quadratic_extrapolate(points.into_iter(), steps / dimension.y)
}

fn quadratic_extrapolate(mut points: impl Iterator<Item = IVec2>, x: i32 ) -> i64 {
    let y0 = points.next().expect("Quadratic extrapolation needs at least three points").y as i64;
    let y1 = points.next().expect("Quadratic extrapolation needs at least three points").y as i64;
    let y2 = points.next().expect("Quadratic extrapolation needs at least three points").y as i64;

    let a = (y2 + y0 - 2*y1) / 2;
    let b = y1 - y0 - a;
    let c = y0;

    let x = x as i64;
    a * x * x + b * x + c
}

fn garden(input: &str) -> impl Iterator<Item = (IVec2, char)> + '_ {
    input.lines().enumerate().flat_map(|(y, line)| {
        line.chars().enumerate().map(move |(x, c)| {
            (
                IVec2 {
                    x: x.try_into().unwrap(),
                    y: y.try_into().unwrap(),
                },
                c,
            )
        })
    })
}

fn neighbours<'a>(
    location: IVec2,
    garden_plots: &'a HashSet<IVec2>,
    dimension: &'a IVec2,
) -> impl Iterator<Item = IVec2> + 'a {
    vec![
        IVec2 {
            x: location.x + 1,
            ..location
        },
        IVec2 {
            y: location.y + 1,
            ..location
        },
        IVec2 {
            x: location.x - 1,
            ..location
        },
        IVec2 {
            y: location.y - 1,
            ..location
        },
    ]
    .into_iter()
    .filter(|location| {
        let reduced_location = IVec2 {
            x: location.x.rem_euclid(dimension.x),
            y: location.y.rem_euclid(dimension.y),
        };
        garden_plots.contains(&reduced_location)
    })
}

#[cfg(test)]
mod tests {
    use super::*;

    use rstest::rstest;

    #[rstest]
    #[case(2, 4)]
    #[case(6, 16)]
    #[case(10, 50)]
    #[case(50, 1594)]
    #[case(100, 6536)]

    fn test_process(#[case] steps: i32, #[case] expected: i64) {
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
        let result = process(lines, steps);
        assert_eq!(result, expected);
    }
}

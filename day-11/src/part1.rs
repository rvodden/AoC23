use std::{collections::HashMap, ffi::NulError, str::Chars};
use itertools::Itertools;

use glam::IVec2;

pub fn process<'a>(
    input: &'a str,
) -> i32 {
    let image = image(input);
    let (empty_row_numbers, empty_column_numbers) = get_empty_row_and_column_numbers(input);

    let unexpanded_galaxies = image.filter_map(|(location, c)| match c {
        '#' => Some(location),
        _ => None
    });

    let galaxies = unexpanded_galaxies.map(|location| {
        let x_expansion = empty_column_numbers.clone().filter(|column_number| location.x > ( *column_number as i32 ) ).count();
        let y_expansion = empty_row_numbers.clone().filter(|row_number| location.y > (*row_number as i32) ).count();
        location + IVec2{x: x_expansion.try_into().unwrap(), y: y_expansion.try_into().unwrap()}
    });

    galaxies.combinations(2).map(|combination| {
        let difference = (combination[0] - combination[1]).abs();
        difference.x + difference.y
    }).sum()
}

fn image<'a>(input: &'a str) -> impl Iterator<Item = (IVec2, char)> + 'a {
    input.lines().enumerate().map(|(y, line)|{
        line.chars().enumerate().map(move |(x, c)|{
            (IVec2{x: x.try_into().unwrap(), y: y.try_into().unwrap()}, c)
        })
    })
    .flatten()
}

fn get_empty_row_and_column_numbers<'a>(input: &'a str) -> (impl Iterator <Item = usize> + Clone + 'a, impl Iterator <Item= usize> + Clone + 'a) {
    let empty_row_numbers = input.lines()
        .enumerate()
        .filter_map(
            |(row_number, line)|
                if line.chars().all(|c| c == '.') { Some(row_number) }
                else { None }
        );

    let number_of_columns = input.lines().next().unwrap().len();

    let empty_column_numbers = (0..number_of_columns).filter(|n|{
        input.lines().map( |line| line.chars().skip(*n).next().unwrap() ).all(|c| c == '.')
    });

    (empty_row_numbers, empty_column_numbers)
}


#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_process() {
        let lines = "...#......
.......#..
#.........
..........
......#...
.#........
.........#
..........
.......#..
#...#.....";
        let result = process(lines);
        assert_eq!(result, 374);
    }
}

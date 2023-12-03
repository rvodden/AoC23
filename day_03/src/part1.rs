use std::collections::HashMap;

pub fn process(
    input: &str,
) -> u32 {

    let lines = input.lines();

    let y_upper_bound = lines.clone().count() as isize;
    let x_upper_bound = lines.clone().next().unwrap().len() as isize;

    let upper_bounds = (x_upper_bound, y_upper_bound);

    let character_map = coord_character_map(lines.clone());
    let symbols = get_symbols(character_map.iter());
    let numbers = get_numbers(lines);

    let part_numbers = numbers.iter()
        .filter(|n| is_part_number(n, symbols.keys(), upper_bounds))
        .inspect(|x| {dbg!(x);} ).for_each(drop);

    // "467..114.." ("467", 0), ("117", 5)

    todo!();
}

fn is_part_number<'a>((number, (x, y)): &(&&str, &(isize, isize)), symbols: impl Iterator<Item = &'a&'a(isize, isize)> + Clone, upper_bounds: (isize, isize)) -> bool{
    let unfiltered_surrounding_coord = generate_surrounding_coordinates(number.len() as isize, (*x, *y));
    let surround_coords = unfiltered_surrounding_coord.iter().filter(
        |(x, y)| 0 <= *x && *x < upper_bounds.0 && 0 <= *y && *y < upper_bounds.1
    );

    surround_coords.any(|number_coord| symbols.clone().any(|symbol_coord| number_coord == *symbol_coord))
}

fn coord_character_map<'a>(lines: impl Iterator<Item = &'a str>) -> HashMap<(isize, isize), char> {
    lines
        .map(|line| line.chars().enumerate().collect())
        .enumerate()
        .flat_map(|(x, y_char) : (isize, Vec<(isize, char)>)| {
            y_char.iter().map(|(y, char)| ((x, *y), *char) ).collect::<Vec<((isize, isize), char)>>()
        }).collect()
}

fn get_symbols<'a>(coord_characters: impl Iterator<Item = (&'a(isize, isize), &'a char)>) -> HashMap<&'a(isize, isize), &'a char> {
    coord_characters.into_iter()
        .filter(|(_, char)| **char != '.' && !char.is_digit(10) )
        .collect()
}

fn get_numbers<'a>(lines: impl Iterator<Item = &'a str>) -> HashMap<&'a str, (isize, isize)>{
    lines.map(get_numbers_in_line)
        .enumerate()
        .flat_map(|(y, numbers_x)| {
            numbers_x.iter().map(|(number, x)| (*number, (*x, y))).collect::<Vec<(&str, (isize, isize))>>()
        })
        .collect()
}

fn generate_surrounding_coordinates(length: isize, (x, y): (isize, isize)) -> Vec<(isize, isize)> {
    let mut surrounding_coordinates = Vec::<(isize, isize)>::new();

    let x_lower_bound = x as isize - 1;
    let x_upper_bound = x as isize + length as isize;
    let y_signed = y as isize;

    for i in x_lower_bound..x_upper_bound {
        surrounding_coordinates.push((i , y_signed - 1));
        surrounding_coordinates.push((i , y_signed + 1));
    }
    surrounding_coordinates.push((x_lower_bound, y_signed));
    surrounding_coordinates.push((x_upper_bound, y_signed));

    surrounding_coordinates
}

fn get_numbers_in_line<'a>(line: &'a str) -> Vec<(&'a str, isize)> {
    let mut chars = line.chars();
    let mut tuples = Vec::<(&str, isize)>::new();

    let mut position: isize = 0;
    while let Some(c) = chars.next() {
        if c.is_digit(10) {
            let start = position;
            while let Some(c) = chars.next() {
                position += 1;
                if ! c.is_digit(10) {
                    break;
                }
            }
            tuples.push((line.get(start..position).unwrap(), start));
        }
        position += 1;
    }

    tuples
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_get_words_in_line() {
        assert_eq!(
            get_numbers_in_line("467..114.."),
            vec![("467", 0), ("114", 5)]
        );
    }

    #[test]
    fn test_day_01a() {
        let input = "467..114..
...*......
..35..633.
......#...
617*......
.....+.58.
..592.....
......755.
...$.*....
.664.598..";
        let result = process(input);
        assert_eq!(result, 8);
    }
}

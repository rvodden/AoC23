pub fn process(
    input: &str,
) -> u32 {

    let lines = input.lines();

    let y_upper_bound = lines.clone().count() as isize;
    let x_upper_bound = lines.clone().next().unwrap().len() as isize;

    let upper_bounds = (x_upper_bound, y_upper_bound);

    let numbers = get_numbers(lines.clone());
    let character_coords = coord_character_map(lines);
    let symbols_coords_map = get_symbols(character_coords.iter());
    let symbols = symbols_coords_map.iter().map(|s| s.0);


    let part_numbers : Vec<u32> = numbers.iter()
        .filter(|n| is_part_number(n, symbols.clone(), upper_bounds))
        .map(|(x, _)| x.parse::<u32>().expect("should be a number"))
        .collect();

    
    part_numbers.iter().sum()
}

fn is_part_number((number, (x, y)): &&(&str, (isize, isize)), symbols: impl Iterator<Item = (isize, isize)> + Clone, upper_bounds: (isize, isize)) -> bool{
    let unfiltered_surrounding_coord = generate_surrounding_coordinates(number.len() as isize, (*x, *y));
    let surround_coords: Vec<&(isize, isize)> = unfiltered_surrounding_coord.iter().filter(
        |(x, y)| 0 <= *x && *x < upper_bounds.0 && 0 <= *y && *y < upper_bounds.1
    )
    .collect();

    let symbol_vec : Vec<(isize, isize)> = symbols.collect();

    surround_coords.iter().any(
        |number_coord| symbol_vec.iter().any(
            |symbol_coord| *number_coord == symbol_coord
        )
    )
}

fn coord_character_map<'a>(lines: impl Iterator<Item = &'a str>) -> Vec<((isize, isize), char)> {
    lines
        .map(|line| line.chars().enumerate().collect())
        .enumerate()
        .flat_map(|(y, x_char) : (usize, Vec<(usize, char)>)| {
            x_char.iter().map(|(x, char)| ((*x as isize, y as isize), *char) ).collect::<Vec<((isize, isize), char)>>()
        }).collect()
}

fn get_symbols<'a>(coord_characters: impl Iterator<Item = &'a((isize, isize), char)>) -> Vec<&'a((isize, isize), char)> {
    coord_characters.into_iter()
        .filter(|(_, char)| *char != '.' && !char.is_ascii_digit() )
        .collect()
}

fn get_numbers<'a>(lines: impl Iterator<Item = &'a str>) -> Vec<( &'a str, (isize, isize))>{
    lines.map(get_numbers_in_line)
        .enumerate()
        .flat_map(|(y, numbers_x)| {
            numbers_x.iter().map(|(number, x)| (*number, (*x, y as isize))).collect::<Vec<(&str, (isize, isize))>>()
        })
        .collect()
}

fn generate_surrounding_coordinates(length: isize, (x, y): (isize, isize)) -> Vec<(isize, isize)> {
    let mut surrounding_coordinates = Vec::<(isize, isize)>::new();

    let x_lower_bound = x - 1;
    let x_upper_bound = x + length + 1;
    let y_signed = y;

    for i in x_lower_bound..x_upper_bound {
        surrounding_coordinates.push((i , y_signed - 1));
        surrounding_coordinates.push((i , y_signed + 1));
    }
    surrounding_coordinates.push((x_lower_bound, y_signed));
    surrounding_coordinates.push((x_upper_bound - 1, y_signed));

    surrounding_coordinates
}

fn get_numbers_in_line(line: &str) -> Vec<(&str, isize)> {
    let mut chars = line.chars().peekable();
    let mut tuples = Vec::<(&str, isize)>::new();

    let mut position: isize = 0;
    while let Some(c) = chars.next() {
        if c.is_ascii_digit() {
            let start = position;
            for c in chars.by_ref() {
                position += 1;
                if ! c.is_ascii_digit() {
                    break;
                }
            }
            if chars.peek().is_none() && line.chars().last().unwrap().is_ascii_digit() {
                position += 1;
            }
            tuples.push((&line[start as usize..position as usize], start));
        }
        position += 1;
    }

    tuples
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_get_numbers_in_line() {
        assert_eq!(
            get_numbers_in_line("467..114.."),
            vec![("467", 0), ("114", 5)]
        );
    }
    
    #[test]
    fn test_get_numbers_at_end_of_line() {
        assert_eq!(
            get_numbers_in_line("467..114"),
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
        assert_eq!(result, 4361);
    } 
}

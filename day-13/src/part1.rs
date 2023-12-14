use std::fmt::Debug;

pub fn process<'a>(
    input: &'a str,
) -> usize {
    let patterns = input.split("\n\n");
    let rotated_patterns = patterns.clone().map(|pattern|{
        rotate_pattern(pattern)
    });

    let horizontal_mirrors : usize = patterns.filter_map(|puzzle_string| process_puzzle(puzzle_string.lines().map(|line| line.chars()))).sum();
    println!("vertical");
    let vertical_mirrors : usize  = rotated_patterns.filter_map(|puzzle_string| process_puzzle( puzzle_string.map(|line| line.into_iter() ) ) ).sum();

    vertical_mirrors + horizontal_mirrors * 100
}

fn rotate_pattern(pattern: &str) -> impl Iterator<Item = Vec<char>> + Debug + Clone + '_  {
    let mut char_vecs = pattern.lines().map( |line| line.chars() ).collect::<Vec<_>>();
    std::iter::from_fn(move || {
        let mut column = vec![];
        for iter in &mut char_vecs {
            match iter.next() {
                Some(item) => column.push(item),
                None => return None
            }
        }
        Some(column)
    })
}

fn process_puzzle(puzzle: impl Iterator<Item = impl DoubleEndedIterator<Item = char>>) -> Option<usize> {
    let puzzle_integers = convert_puzzle_to_integers(puzzle);
    dbg!(find_reflection(&puzzle_integers))
}

fn convert_puzzle_to_integers(puzzle: impl Iterator< Item = impl DoubleEndedIterator<Item = char>>) -> Vec<usize> {
    puzzle.map(|line| {
        line.rev().enumerate().filter(|(_, c)| *c == '#')
            .fold(0, |acc, (bit, _)| acc | (1 << bit) )
    }).collect()
}


fn find_reflection(integers: &[usize]) -> Option<usize> {
    for i in 1..integers.len() {

        let forward = integers[..i].iter().rev();
        let back = &integers[i..];

        if forward.zip(back).all(|(a,b)| a == b ) {
            return Some(i)
        }
    }
    None
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_process() {
        let lines = "#.##..##.
..#.##.#.
##......#
##......#
..#.##.#.
..##..##.
#.#.##.#.

#...##..#
#....#..#
..##..###
#####.##.
#####.##.
..##..###
#....#..#";
        let result = process(lines);
        assert_eq!(result, 405);
    }
}

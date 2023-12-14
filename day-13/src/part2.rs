use itertools::Itertools;
use std::fmt::Debug;

pub fn process(input: &str) -> usize {
    let patterns = input.split("\n\n");

    let (horizontal_mirrors, vertical_mirrors): (Vec<Option<usize>>, Vec<Option<usize>>) = patterns.map(process_pattern).unzip();
    (horizontal_mirrors.into_iter().flatten().sum::<usize>() * 100)
        + vertical_mirrors.into_iter().flatten().sum::<usize>()
}

fn rotate_pattern(pattern: &str) -> impl Iterator<Item = Vec<char>> + Debug + Clone + '_ {
    let mut char_vecs = pattern.lines().map(|line| line.chars()).collect::<Vec<_>>();
    std::iter::from_fn(move || {
        let mut column = vec![];
        for iter in &mut char_vecs {
            match iter.next() {
                Some(item) => column.push(item),
                None => return None,
            }
        }
        Some(column)
    })
}

fn process_pattern(pattern: &str) -> (Option<usize>, Option<usize>) {
    let horizontal_integers = convert_pattern_to_integers(pattern.lines().map(|line| line.chars()));
    let vertical_pattern = rotate_pattern(pattern)
        .collect::<Vec<_>>()
        .into_iter()
        .map(|x| x.into_iter());
    let vertical_integers = convert_pattern_to_integers(vertical_pattern);

    let horizontal_reflection = find_smudged_reflection(horizontal_integers);
    let vertical_reflection = find_smudged_reflection(vertical_integers);

    (horizontal_reflection, vertical_reflection)
}

fn find_smudged_reflection(integers: Vec<usize>) -> Option<usize> {
    // get a list of all the combinations of pairs of enumerated integers
    let potential_smudges = integers
        .iter()
        .enumerate()
        .combinations(2)
        .filter(|a| usize::count_ones(a[0].1 ^ a[1].1) == 1)
        .map(|a| (a[0].0, usize::BITS - (a[0].1 ^ a[1].1).leading_zeros() - 1));

    let original_reflection = find_reflections(&integers).map(|x| x[0]);

    for (index, bit) in potential_smudges {
        let mut new_integers = integers.clone();
        new_integers[index] ^= 1 << bit;
        let Some(mut reflections) = find_reflections(&new_integers) else {
            continue
        };

        if let Some(original_reflection) = original_reflection {
            if let Some(original_reflection_position) = reflections.iter().position(|x| *x == original_reflection) {
                reflections.remove(original_reflection_position);
                if reflections.is_empty() { continue };
            }
        };

        if reflections.len() > 1 {
            panic!("Found more than one reflection!");
        };

        return Some(*reflections.first().unwrap());
    }
    None
}

#[allow(dead_code)]
fn display(integers: Vec<usize>) {
    let width = integers
        .iter()
        .map(|x| usize::BITS - x.leading_zeros())
        .max()
        .unwrap() as usize;
    for integer in integers {
        let mut string = format!("{:0width$b}", integer, width = width);
        string = string.replace('0', ".");
        string = string.replace('1', "#");
        println!("{}", string);
    }
}

fn convert_pattern_to_integers(
    puzzle: impl Iterator<Item = impl DoubleEndedIterator<Item = char>>,
) -> Vec<usize> {
    puzzle
        .map(|line| {
            line.rev()
                .enumerate()
                .filter(|(_, c)| *c == '#')
                .fold(0, |acc, (bit, _)| acc | (1 << bit))
        })
        .collect()
}

fn find_reflections(integers: &[usize]) -> Option<Vec<usize>> {
    let mut retval = vec![];
    for i in 1..integers.len() {
        let forward = integers[..i].iter().rev();
        let back = &integers[i..];

        if forward.zip(back).all(|(a, b)| a == b) {
            retval.push(i);
        }
    }
    if retval.is_empty() { return None }
    Some(retval)
}

#[cfg(test)]
mod tests {
    use super::*;
    use rstest::rstest;

    #[rstest]
    #[case("#.##..##.
..#.##.#.
##......#
##......#
..#.##.#.
..##..##.
#.#.##.#.", (Some(3), None))]
    #[case("#...##..#
#....#..#
..##..###
#####.##.
#####.##.
..##..###
#....#..#", (Some(1), None))]

#[case("#...#..#.
#...#..#.
.####..#.
..#..#..#
...#....#
##..###..
#...####.
#...####.
##..##...
...#....#
..#..#..#
.####..#.
#...#..#.", (Some(7), None))]
    #[test]
    fn test_patterns(#[case] pattern: &str, #[case] expected: (Option<usize>, Option<usize>)) {
        assert_eq!(process_pattern(pattern), expected)
    }

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
        assert_eq!(result, 400);
    }
}

use std::iter::Iterator;

pub fn process<Line: AsRef<str>, Lines: Iterator<Item=Line>>(lines: Lines) -> u32 {
    lines.map(process_line).sum::<u32>()
}

fn process_line<Line: AsRef<str>>(line: Line) -> u32 {
    let new_line = line.as_ref()
        .replace("one", "o1e")
        .replace("two", "t2o")
        .replace("three", "t3e")
        .replace("four", "4")
        .replace("five", "5e")
        .replace("six", "6")
        .replace("seven", "7n")
        .replace("eight", "e8t")
        .replace("nine", "n9e");

    let mut iterator = new_line.chars().filter(|c| c.is_digit(10));

    let first = iterator.next().expect("Should be a digit!");

    iterator
        .last()
        .map_or_else(
            || format!("{first}{first}"),
            |last| format!("{first}{last}"),
        )
        .parse::<u32>()
        .expect("Should be a number")
}

#[cfg(test)]
mod tests {
    use super::*;
    use rstest::rstest;

    #[rstest]
    #[case["two1nine", 29]]
    #[case["eightwothree", 83]]
    #[case["abcone2threexyz", 13]]
    #[case["xtwone3four", 24]]
    #[case["4nineeightseven2", 42]]
    #[case["zoneight234", 14]]
    #[case["7pqrstsixteen", 76]]

    fn line_test(#[case] input: String, #[case] output: u32) {
        assert_eq!(process_line(input), output)
    }

    #[test]
    fn test_day_01b() {
        let lines = "two1nine
eightwothree
abcone2threexyz
xtwone3four
4nineeightseven2
zoneight234
7pqrstsixteen".lines();
        let result = process(lines.into_iter());
        assert_eq!(result, 281);
    }
}
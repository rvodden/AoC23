use std::fs::File;
use std::io::BufRead;
use std::io::BufReader;
use std::iter::Iterator;

fn read_file_line_by_line(filepath: &str) -> Vec<String> {
    let file = match File::open(filepath) {
        Ok(file) => file,
        Err(e) => panic!("Panic! {}", e),
    };
    let reader = BufReader::new(file);
    reader.lines().map(|l| l.unwrap()).collect()
}

fn day_01a(lines: impl Iterator<Item = String>) -> u32 {
    lines.map(process_line_a).sum::<u32>()
}

fn process_line_a(line: String) -> u32 {
    let mut iterator = line.chars().filter(|c| c.is_digit(10));

    let first = iterator.next().expect("Should be a digit!");

    match iterator.last() {
        Some(last_character) => format!("{first}{last_character}"),
        None => format!("{first}{first}"),
    }
    .parse::<u32>()
    .expect("Should be a number")
}

fn day_01b(lines: impl Iterator<Item = String>) -> u32 {
    lines.map(process_line_b).sum::<u32>()
}

fn process_line_b(line: String) -> u32 {
    let new_line = line
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

    #[test]
    fn test_day_01a() {
        let lines = read_file_line_by_line("test_input_a.txt");
        let result = day_01a(lines.into_iter());
        assert_eq!(result, 142);
    }

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
        assert_eq!(process_line_b(input), output)
    }

    #[test]
    fn test_day_01b() {
        let lines = read_file_line_by_line("test_input_b.txt");
        let result = day_01b(lines.into_iter());
        assert_eq!(result, 281);
    }
}

fn main() {
    let lines = read_file_line_by_line("input.txt");
    println!("{}", day_01a(lines.clone().into_iter()));
    println!("{}", day_01b(lines.into_iter()));
}

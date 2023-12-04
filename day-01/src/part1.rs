use std::iter::Iterator;

pub fn process<Item: AsRef<str>, Container: Iterator<Item=Item>>(lines: Container) -> u32 {
    lines.map(process_line).sum::<u32>()
}

fn process_line<Item: AsRef<str>>(line: Item) -> u32 {
    let mut iterator = line.as_ref().chars().filter(|c| c.is_digit(10));

    let first = iterator.next().expect("Should be a digit!");

    match iterator.last() {
        Some(last_character) => format!("{first}{last_character}"),
        None => format!("{first}{first}"),
    }
    .parse::<u32>()
    .expect("Should be a number")
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_day_01a() {
        let lines = "1abc2
pqr3stu8vwx
a1b2c3d4e5f
treb7uchet".lines();
        let result = process(lines);
        assert_eq!(result, 142);
    }
}


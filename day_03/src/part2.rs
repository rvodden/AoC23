
pub fn process(
    input: &str,
) -> u32 {
    todo!()
}


#[cfg(test)]
mod tests {
    use super::*;

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
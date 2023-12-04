use std::iter::Iterator;

use crate::parse::{card, Card};

pub fn process( input: &str ) -> u32 {
    let lines = input.lines();
    lines.map(|line| card(line).expect("should parse").1).map(points).sum()
}

fn points(card: Card) -> u32 {
    let number_of_winning_numbers = card.numbers.iter().filter(
        |number| card.winning_numbers.iter().any(|winning_number| winning_number == *number)
    )
    // .inspect(|x| {dbg!(x);})
    .count() as u32;
    // dbg!(number_of_winning_numbers);

    if number_of_winning_numbers == 0 { 0 } else { 2u32.pow(number_of_winning_numbers - 1) }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_points() {
        assert_eq!(
            points(Card{
                id: 1,
                winning_numbers: vec![41,48,83,86,17],
                numbers: vec![83,86,6,31,17,9,48,53]
            }),
            8
        )
    }

    #[test]
    fn test_day_01a() {
        let lines = "Card 1: 41 48 83 86 17 | 83 86  6 31 17  9 48 53
Card 2: 13 32 20 16 61 | 61 30 68 82 17 32 24 19
Card 3:  1 21 53 59 44 | 69 82 63 72 16 21 14  1
Card 4: 41 92 73 84 69 | 59 84 76 51 58  5 54 83
Card 5: 87 83 26 28 32 | 88 30 70 12 93 22 82 36
Card 6: 31 18 13 56 72 | 74 77 10 23 35 67 36 11";
        let result = process(lines);
        assert_eq!(result, 13);
    }
}

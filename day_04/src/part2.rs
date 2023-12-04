use std::iter::{Iterator, repeat};

use crate::parse::{card, Card};

pub fn process( input: &str ) -> u32 {
    let lines = input.lines();
    let mut counts : Vec<usize> = repeat(1).take(lines.clone().count() + 1).collect();
    let cards = lines
        .map(|line| card(line).expect("should parse").1);
    
    for card in cards {
        let number_of_winning_numbers = number_of_winning_numbers(&card);
        let number_of_current_card = counts[card.id as usize];
        for card_to_copy in (card.id + 1)..(card.id + number_of_winning_numbers + 1) {
            counts[card_to_copy as usize] += number_of_current_card
        }
    }

    counts.iter().sum::<usize>() as u32 - 1
}

fn number_of_winning_numbers<'a>(card: &'a Card) -> u32 {
    card.numbers.iter().filter(
        |number| card.winning_numbers.iter().any(|winning_number| winning_number == *number)
    )
    // .inspect(|x| {dbg!(x);})
    .count() as u32
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_winning_numbers() {
        assert_eq!(
            number_of_winning_numbers(&Card{
                id: 1,
                winning_numbers: vec![41,48,83,86,17],
                numbers: vec![83,86,6,31,17,9,48,53]
            }),
            4
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
        assert_eq!(result, 30);
    }
}

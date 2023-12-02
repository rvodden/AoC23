use std::iter::Iterator;
use std::fmt::Debug;

use crate::parse::{
    game,
    Round
};

pub fn process<Item: AsRef<str> + Copy, Container: Iterator<Item = Item> + Debug>(
    lines: Container,
) -> u32 {
    lines.map(|line| game(line.as_ref()).expect("Should Parse").1).filter_map(
        |game| {
            match game.rounds.iter().all(
                is_possible
            ) {
                false => None,
                true =>  Some(game.id)
            }
        }
    ).sum()
}

pub fn is_possible(round: &Round) -> bool {
    round.red <= 12 && round.green <= 13 && round.blue <= 14
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_day_01a() {
        let lines = "Game 1: 3 blue, 4 red; 1 red, 2 green, 6 blue; 2 green
Game 2: 1 blue, 2 green; 3 green, 4 blue, 1 red; 1 green, 1 blue
Game 3: 8 green, 6 blue, 20 red; 5 blue, 4 red, 13 green; 5 green, 1 red
Game 4: 1 green, 3 red, 6 blue; 3 green, 6 red; 3 green, 15 blue, 14 red
Game 5: 6 red, 1 blue, 3 green; 2 blue, 1 red, 2 green"
            .lines();
        let result = process(lines);
        assert_eq!(result, 8);
    }
}

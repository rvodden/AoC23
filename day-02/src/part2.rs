use std::cmp::max;
use std::fmt::Debug;
use std::iter::Iterator;

use crate::parse::{
    game,
    Game,
    Round
};


pub fn process<Item: AsRef<str> + Copy, Container: Iterator<Item = Item> + Debug>(
    lines: Container,
) -> u32 {
    lines.map(|line| game(line.as_ref()).expect("Should Parse").1)
        .map(power).sum()
}

fn power(game: Game) -> u32 {
    let max_round = game.rounds.iter().fold(
        Round{red: 0,green: 0,blue: 0},
        |acc, round| Round {
            red: max(acc.red, round.red),
            green: max(acc.green, round.green),
            blue: max(acc.blue, round.blue)
        }
    );
    max_round.blue * max_round.red * max_round.green
}



#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_day_01b() {
        let lines = "Game 1: 3 blue, 4 red; 1 red, 2 green, 6 blue; 2 green
Game 2: 1 blue, 2 green; 3 green, 4 blue, 1 red; 1 green, 1 blue
Game 3: 8 green, 6 blue, 20 red; 5 blue, 4 red, 13 green; 5 green, 1 red
Game 4: 1 green, 3 red, 6 blue; 3 green, 6 red; 3 green, 15 blue, 14 red
Game 5: 6 red, 1 blue, 3 green; 2 blue, 1 red, 2 green"
            .lines();
        let result = process(lines);
        assert_eq!(result, 2286);
    }

    #[test]
    fn test_game_power() {
        assert_eq!(
            power(Game{
                id: 1,
                rounds: vec![
                    Round{blue: 3, red: 4, green: 0}, 
                    Round{green: 2, blue: 6, red: 1}, 
                    Round{green: 2, red: 0, blue: 0}]
            }),
            48
        )
    }
}

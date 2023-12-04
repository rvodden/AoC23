use std::collections::HashMap;
use nom::{
    bytes::complete::tag,
    character::complete::{self, alpha1, digit1},
    combinator::map_res,
    multi::separated_list1,
    sequence::{preceded, separated_pair},
    IResult,
};

#[derive(Eq, PartialEq, Debug)]
pub struct Round {
    pub red: u32,
    pub green: u32,
    pub blue: u32,
}

#[derive(Eq, PartialEq, Debug)]
pub struct Game {
    pub id: u32,
    pub rounds: Vec<Round>,
}

// Game 1: 3 red, 4 blue; 5 green, 7 red, 3 blue
pub fn game(input: &str) -> IResult<&str, Game> {
    map_res(
        separated_pair(
            preceded(tag("Game "), digit1),
            tag(": "),
            separated_list1(tag("; "), round),
        ),
        |(id_str, rounds)| match id_str.parse() {
            Ok(id) => Ok(Game { id, rounds }),
            Err(e) => Err(e),
        },
    )(input)
}

// 3 red, 4 blue
fn round(input: &str) -> IResult<&str, Round> {
    let (input, result) = separated_list1(tag(", "), cube)(input).expect("Should parse!");
    Ok((
        input,
        Round::from_hash_map(result.iter().map(|x| (x.1, x.0)).collect()),
    ))
}

impl Round {
    pub fn from_hash_map(source: HashMap<&str, u32>) -> Round {
        Round {
            red: *source.get("red").unwrap_or(&0),
            green: *source.get("green").unwrap_or(&0),
            blue: *source.get("blue").unwrap_or(&0),
        }
    }
}

// 3 red
fn cube(input: &str) -> IResult<&str, (u32, &str)> {
    separated_pair(complete::u32, tag(" "), alpha1)(input)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_parse_round() {
        assert_eq!(
            round("32 red"),
            Ok((
                "",
                Round {
                    red: 32,
                    green: 0,
                    blue: 0
                }
            ))
        );
        assert_eq!(
            round("3 green"),
            Ok((
                "",
                Round {
                    red: 0,
                    green: 3,
                    blue: 0
                }
            ))
        );
        assert_eq!(
            round("126 blue"),
            Ok((
                "",
                Round {
                    red: 0,
                    green: 0,
                    blue: 126
                }
            ))
        );
        assert_eq!(
            round("126 blue, 23 green, 8 red"),
            Ok((
                "",
                Round {
                    red: 8,
                    green: 23,
                    blue: 126
                }
            ))
        );
    }

    #[test]
    fn test_parse_game() {
        assert_eq!(
            game("Game 2: 1 blue, 2 green; 3 green, 4 blue, 1 red; 1 green, 1 blue")
                .unwrap()
                .1,
            Game {
                id: 2,
                rounds: vec![
                    Round {
                        green: 2,
                        blue: 1,
                        red: 0
                    },
                    Round {
                        blue: 4,
                        red: 1,
                        green: 3
                    },
                    Round {
                        green: 1,
                        blue: 1,
                        red: 0
                    },
                ]
            }
        )
    }
}

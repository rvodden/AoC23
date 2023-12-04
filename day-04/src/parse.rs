use nom::{multi::separated_list1, IResult, character::complete::{self, multispace1}, sequence::{separated_pair, pair, delimited}, bytes::complete::tag, combinator::map};

#[derive(PartialEq, Debug, Clone)]
pub struct Card {
    pub id: u32,
    pub winning_numbers: Vec<u32>,
    pub numbers: Vec<u32>
}

// 83 86  6 31 17  9 48 53
fn list_of_u32(input: &str) -> IResult<&str,Vec<u32>> {
    separated_list1(multispace1, complete::u32)(input)
}

// 41 48 83 86 17 | 83 86  6 31 17  9 48 53
fn numbers(input: &str) -> IResult<&str, (Vec<u32>,Vec<u32>)> {
    separated_pair(
        list_of_u32,
        pair(tag(" |"), multispace1),
        list_of_u32
    )(input)
}


// Card 1
fn card_id(input: &str) -> IResult<&str, u32> {
    delimited(pair(tag("Card"), multispace1), complete::u32, pair(tag(":"), multispace1))(input)
}

// Card 1: 41 48 83 86 17 | 83 86  6 31 17  9 48 53
pub fn card(input: &str) -> IResult<&str, Card> {
    map (
        pair(
            card_id,
            numbers
        ),
        |(id, (winning_numbers, numbers))| Card{id, winning_numbers, numbers}
    )(input)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_list_of_u32() {
        assert_eq!(
            list_of_u32("83 86  6 31 17  9 48 53").expect("should parse"),
            ("", vec![83,86,6,31,17,9,48,53])
        )
    }

    #[test]
    fn test_numbers() {
        assert_eq!(
            numbers("41 48 83 86 17 | 83 86  6 31 17  9 48 53").expect("should parse"),
            ("", (vec![41,48,83,86,17], vec![83,86,6,31,17,9,48,53]))
        )
    }

    #[test]
    fn test_card_id() {
        assert_eq!(
            card_id("Card 1: ").expect("Should parse"),
            ("", 1)
        )
    }

    #[test]
    fn test_card() {
        assert_eq!(
            card("Card 1: 41 48 83 86 17 | 83 86  6 31 17  9 48 53").expect("it to parse"),
            ("",
            Card{
                id: 1,
                winning_numbers: vec![41,48,83,86,17],
                numbers: vec![83,86,6,31,17,9,48,53]
            })
        )
    }

}

use std::ops::Deref;

use itertools::Itertools;

pub fn process<'a>(input: &'a str) -> u32 {
    input
        .lines()
        .map(|line| {
            let (hand, bid) = line.split_once(" ").unwrap();
            (
                hand,
                bid.parse::<u32>().expect("should be a number"),
                score_hand(hand),
            )
        })
        .sorted_by_key(|x| x.2)
        .enumerate()
        // .inspect(|x| { dbg!(x);} )
        .map(|(rank, (_, bid, _))| (rank + 1) as u32 * bid)
        .sum()
}
#[derive(PartialEq, Eq, PartialOrd, Ord, Debug, Clone, Copy)]
enum HandType {
    HighCard,
    OnePair,
    TwoPair,
    ThreeOfAKind,
    FullHouse,
    FourOfAKind,
    FiveOfAKind,
}

fn to_tuple(hand: &str) -> (u32, u32, u32, u32, u32) {
    hand.chars()
        .map(|c| match c {
            'A' => 14,
            'K' => 13,
            'Q' => 12,
            'T' => 10,
            'J' => 1,
            value => value.to_digit(10).expect("should be a digit"),
        })
        .collect_tuple()
        .expect("should be a tuple")
}

fn score_hand(hand: &str) -> (HandType, (u32, u32, u32, u32, u32)) {
    use HandType::*;

    let counts = hand.chars().counts();

    let mut values = counts.values().sorted().join("");
    if counts.get(&'J') .is_some(){
        let jless_counts = hand.chars().filter(|c| *c != 'J').counts();
        let jless_values = jless_counts.values().sorted().join("");
        values = match jless_values.deref() {
            // 1 Joker
            "4" => "5".to_string(),
            "22" => "23".to_string(),
            "13" => "14".to_string(),
            "112" => "113".to_string(),
            "1111" => "1112".to_string(),
            // 2 Jokers
            "3" => "5".to_string(),
            "12" => "14".to_string(),
            "111" => "113".to_string(),
            // 3 Jokers
            "2" => "5".to_string(),
            "11" => "14".to_string(),
            // 4 Jokers
            "1" => "5".to_string(),
            // 5 Jokers,
            "" => "5".to_string(),
            value => panic!("This should never happen, got `{value}`")

        }
    }


    let hand_type = match values.deref() {
        "5" => FiveOfAKind,
        "14" => FourOfAKind,
        "23" => FullHouse,
        "113" => ThreeOfAKind,
        "122" => TwoPair,
        "1112" => OnePair,
        "11111" => HighCard,
        value => panic!("should never happen. Encountered `{}`", value),
    };

    (hand_type, to_tuple(hand))
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_process() {
        let lines = "32T3K 765
T55J5 684
KK677 28
KTJJT 220
QQQJA 483";
        let result = process(lines);
        assert_eq!(result, 5905);
    }
}

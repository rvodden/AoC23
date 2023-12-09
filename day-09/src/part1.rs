use nom::{bytes::complete::tag, character::complete, multi::separated_list1, IResult, sequence};
use std::iter::repeat_with;

pub fn process<'a>(
    input: &'a str,
) -> i32 {
    let sequences: Vec<Vec<i32>> = input.lines()
        .map(|x| sequence(x).expect("should parse").1
        ).collect();

    sequences.iter().map(extend_sequence).sum()
}

fn sequence(input: &str) -> IResult<&str, Vec<i32>> {
    separated_list1(
        tag(" "), 
        complete::i32
    )(input)
}

fn extend_sequence(sequence: &Vec<i32>) -> i32 {

    let mut current_sequence = sequence.clone();
    repeat_with(|| {
            let tmp = current_sequence.clone();
            current_sequence = current_sequence.iter().skip(1).zip(current_sequence.iter()).map(|(a, b)| a - b).collect();
            tmp
        })
        .take_while(|sequence: &Vec<i32> | ! sequence.iter().all(|x| *x == 0) )
        .map(|x| *x.iter().last().expect("these should not be empty") )
        .collect::<Vec<i32>>().iter().rev().fold(0, |acc, x| acc + x)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_reduce_sequence() {
        let val = extend_sequence(&vec![0,3,6,9,12,15]);
        assert_eq!(val, 18)
    }

    #[test]
    fn test_process() {
        let lines = "0 3 6 9 12 15
1 3 6 10 15 21
10 13 16 21 30 45";
        let result = process(lines);
        assert_eq!(result, 114);
    }
}

use std::collections::VecDeque;

use nom::{character::complete::{alpha1, self}, bytes::complete::tag, branch::alt, combinator::opt};

pub fn process<'a>(
    input: &'a str,
) -> u32 {

    let mut lens_boxes: [VecDeque<Lens>; 256] = std::array::from_fn(|_| VecDeque::<Lens>::new());
    let operations = input.split(",").map(parse);

    for operation in operations {
        use Operation::*;
        match operation {
            Insert(position, label, focal_length) => {
                let lens_box = &mut lens_boxes[position];
                match lens_box.iter().position(|lens| lens.label == label) {
                    None => lens_box.push_back(Lens{label, focal_length}),
                    Some(idx) => lens_box[idx].focal_length = focal_length,
                };
            },
            Remove(position, label) => {
                let lens_box = &mut lens_boxes[position];
                lens_box.iter().position(|lens| lens.label == label).and_then(|idx| lens_box.remove(idx));
            }
        }
    }

    lens_boxes.iter().enumerate().map(|(box_number, lens_box)| {
        lens_box.iter().enumerate().map(move |(lens_position, lens)| {
            (box_number as u32 + 1) * (lens_position as u32+ 1) * lens.focal_length
        }).sum::<u32>()
    }).sum()
}

fn hash(cs: &str) -> u32 {
    cs.chars().fold(0, |acc, c| ((acc + c as u32) * 17) % 256 )
}

#[derive(Debug, PartialEq, Eq, Clone, Copy)]
struct Lens<'a> {
    label: &'a str,
    focal_length: u32
}

#[derive(Debug, Clone, Copy)]
enum Operation<'a> {
    Insert(usize, &'a str, u32),
    Remove(usize, &'a str)
}

fn parse(input: &str) -> Operation {
    let (input, label) = alpha1::<_, nom::error::Error<_>>(input).expect("should parse");
    let (input, operation) = alt::<_, _, nom::error::Error<_>,_>((tag("="), tag("-")))(input).expect("should parse");
    let (_, focal_length) = opt::<_, _, nom::error::Error<_>, _>(complete::u32)(input).expect("should parse");
    let hash = hash(label) as usize;

    match operation { 
        "=" => Operation::Insert(hash, label, focal_length.expect("Insert operations should provide focal lengths")),
        "-" => Operation::Remove(hash, label),
        value => panic!("Received invalid character: {}", value)
    }
}


#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_process() {
        let lines = "rn=1,cm-,qp=3,cm=2,qp-,pc=4,ot=9,ab=5,pc-,pc=6,ot=7";
        let result = process(lines);
        assert_eq!(result, 145);
    }
}
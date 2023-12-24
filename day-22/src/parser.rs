use std::{cell::RefCell, rc::Rc};

use nom::{character::complete::{self, newline}, bytes::complete::tag, IResult, sequence::Tuple, multi::separated_list1};
use glam::UVec3;
use crate::domain::{Brick, Bricks, BrickRef};


pub fn bricks(input: &str) -> IResult<&str, Bricks> {
    let (input, brick_list) = separated_list1(newline, brick)(input)?;
    Ok((input, brick_list.into_iter().map(|brick| BrickRef(Rc::new(RefCell::new(brick)))).collect()))
}

// 1,0,1~1,2,1
fn brick(input: &str) -> IResult<&str, Brick> {
    let (input, (start, _, end)) = (uvec3, tag("~"), uvec3).parse(input)?;
    Ok((input, Brick::new(start, end)))
}

// 1,0,1
fn uvec3(input: &str) -> IResult<&str, UVec3> {
    let (input, (x,_,y,_,z)) = (complete::u32, tag(","), complete::u32, tag(","), complete::u32).parse(input)?;
    Ok((input, UVec3 { x, y, z }))
}

#[cfg(test)]
mod tests {
    use crate::domain::BrickRef;

    use super::*;
    use rstest::rstest;

    #[rstest]
    #[case("1,0,1~1,2,1
0,0,2~2,0,2
0,2,3~2,2,3
0,0,4~0,2,4
2,0,5~2,2,5
0,1,6~2,1,6
1,1,8~1,1,9", vec![
    BrickRef::from_brick(Brick{x: 1..2, y: 0..3, z: 1..2 }),
    BrickRef::from_brick(Brick{x: 0..3, y: 0..1, z: 2..3 }),
    BrickRef::from_brick(Brick{x: 0..3, y: 2..3, z: 3..4 }),
    BrickRef::from_brick(Brick{x: 0..1, y: 0..3, z: 4..5 }),
    BrickRef::from_brick(Brick{x: 2..3, y: 0..3, z: 5..6 }),
    BrickRef::from_brick(Brick{x: 0..3, y: 1..2, z: 6..7 }),
    BrickRef::from_brick(Brick{x: 1..2, y: 1..2, z: 8..10}),
    ])]
    fn test_bricks(
        #[case] input: &str,
        #[case] expected: Vec<BrickRef>
    ) {
        let (input, result) = bricks(input).expect("Should parse");
        assert_eq!(input, "");
        assert_eq!(result, expected);
    }

    #[rstest]
    #[case("1,0,1~1,2,1", Brick{x: 1..2, y: 0..3, z: 1.. 2} )]
    #[case("0,0,2~2,0,2", Brick{x: 0..3, y: 0..1, z: 2.. 3} )]
    #[case("0,2,3~2,2,3", Brick{x: 0..3, y: 2..3, z: 3.. 4} )]
    #[case("0,0,4~0,2,4", Brick{x: 0..1, y: 0..3, z: 4.. 5} )]
    #[case("2,0,5~2,2,5", Brick{x: 2..3, y: 0..3, z: 5.. 6} )]
    #[case("0,1,6~2,1,6", Brick{x: 0..3, y: 1..2, z: 6.. 7} )]
    #[case("1,1,8~1,1,9", Brick{x: 1..2, y: 1..2, z: 8..10} )]
    fn test_brick(
        #[case] input: &str,
        #[case] expected: Brick
    ) {
        let (input, result) = brick(input).expect("Should parse");
        assert_eq!(input, "");
        assert_eq!(result, expected);
    }

    #[rstest]
    #[case("1,0,1", UVec3{ x: 1, y: 0, z: 1 })]
    #[case("1,2,1", UVec3{ x: 1, y: 2, z: 1 })]
    #[case("0,0,2", UVec3{ x: 0, y: 0, z: 2 })]
    #[case("2,0,2", UVec3{ x: 2, y: 0, z: 2 })]
    #[case("0,2,3", UVec3{ x: 0, y: 2, z: 3 })]
    #[case("2,2,3", UVec3{ x: 2, y: 2, z: 3 })]
    #[case("0,0,4", UVec3{ x: 0, y: 0, z: 4 })]
    #[case("0,2,4", UVec3{ x: 0, y: 2, z: 4 })]
    #[case("2,0,5", UVec3{ x: 2, y: 0, z: 5 })]
    #[case("2,2,5", UVec3{ x: 2, y: 2, z: 5 })]
    #[case("0,1,6", UVec3{ x: 0, y: 1, z: 6 })]
    #[case("2,1,6", UVec3{ x: 2, y: 1, z: 6 })]
    #[case("1,1,8", UVec3{ x: 1, y: 1, z: 8 })]
    #[case("1,1,9", UVec3{ x: 1, y: 1, z: 9 })]
    fn test_uvec3(
        #[case] input: &str,
        #[case] expected: UVec3
    ) {
        let (input, result) = uvec3(input).expect("Should parse");
        assert_eq!(input, "");
        assert_eq!(result, expected);
    }

}
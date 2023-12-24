use std::cmp::{min, max};
use std::ops::Range;
use glam::UVec3;

use crate::parser::bricks;
use crate::domain::{Brick, Bricks, BrickRef};

pub fn process(
    input: &str,
) -> u32 {
    let (_, bricks): (_, Bricks) = bricks(input).expect("should parse");
    
    stabilize(&bricks);
    
    let mut vapourisable_bricks_count = 0;
    for brick in bricks.iter() {
        let test_bricks: Bricks = bricks.iter().filter(|x| *x.borrow() != *brick.borrow() ).cloned().collect();
        if is_stable(&test_bricks) {
            vapourisable_bricks_count += 1
        }
    }

    vapourisable_bricks_count
}

fn stabilize(bricks: &Bricks) {
    while !is_stable(bricks) {
        let mut newer_bricks = bricks.clone();
        for brick in newer_bricks.iter_mut() {
            if !is_supported(brick.clone(), bricks) {
                brick.borrow_mut().move_down();
            }
        }
    }
}

fn space_below_brick(brick: BrickRef) -> Option<Brick> {
    let brick = brick.borrow().clone();
    if brick.z.start == 1 { return None };
    Some(Brick {
        z: brick.z.start - 1 .. brick.z.end -1,
        ..brick
    })
}

fn overlap<Idx>(lhs: &Range<Idx>, rhs: &Range<Idx>) -> bool where Idx: Ord {
    lhs.start < rhs.end && rhs.start < lhs.end
}

fn intersect(lhs: &Brick, rhs: BrickRef) -> bool {
    let rhs = rhs.borrow();
    overlap(&lhs.x, &rhs.x) && overlap(&lhs.y, &rhs.y) && overlap(&lhs.z, &rhs.z)
}

fn is_stable(bricks: &Bricks) -> bool {
    for brick in bricks.iter() {
        if ! is_supported(brick.clone(), bricks) {
            return false;
        }
    }
    true
}

fn is_supported(brick: BrickRef, bricks: &Bricks) -> bool {
    if let Some(space) = space_below_brick(brick.clone()) {
        for other_brick in bricks {
            if brick == *other_brick { continue };
            if intersect(&space, other_brick.clone()) { return true };
        }
        false
    } else {
        // if there is no space below the brick, then it is supported by the floor!
        true
    }
}

#[allow(dead_code)]
fn bounds<'a>(bricks: impl Iterator<Item = &'a Brick>) -> (UVec3, UVec3) {
    let mut maximum = UVec3::ZERO;
    let mut minimum = UVec3::MAX;

    for brick in bricks {
        maximum = UVec3 {
            x: max(maximum.x, brick.x.end),
            y: max(maximum.y, brick.y.end),
            z: max(maximum.z, brick.z.end),
        };
        minimum = UVec3 {
            x: min(minimum.x, brick.x.start),
            y: min(minimum.y, brick.y.start),
            z: min(minimum.z, brick.z.start),
        };
    }
    (minimum, maximum)
}

#[cfg(test)]
mod tests {
    use super::*;
    use rstest::rstest;

    #[test]
    fn test_process() {
        let lines = "1,0,1~1,2,1
0,0,2~2,0,2
0,2,3~2,2,3
0,0,4~0,2,4
2,0,5~2,2,5
0,1,6~2,1,6
1,1,8~1,1,9";
        let result = process(lines);
        assert_eq!(result, 5);
    }

    #[rstest]
    #[case(0..6, 0..6, true)]
    #[case(0..1, 0..1, true)]
    #[case(0..5, 2..3, true)]
    #[case(0..5, 2..9, true)]
    #[case(0..5, 5..9, false)]
    #[case(0..2, 6..9, false)]
    fn test_overlap(
        #[case] lhs: Range<u32>,
        #[case] rhs: Range<u32>,
        #[case] expected: bool
    ){
        assert_eq!(overlap(&lhs, &rhs), expected)
    }
}

use std::cmp::{min, max};
use std::collections::{HashSet, HashMap};
use std::ops::{Range, Not};

use glam::UVec3;

use crate::parser::bricks;
use crate::domain::{Brick, Bricks, BrickRef};

pub fn process(
    input: &str,
) -> u32 {
    let (_, bricks): (_, Bricks) = bricks(input).expect("should parse");
    stabilize(&bricks);
    let support_structure = analyse_support_structure(&bricks);
    let removable_bricks = support_structure.iter()
    .filter_map(|(_, obs)| match obs {
        Some(bs) => (bs.len() == 1).then_some( bs.first().unwrap() ),
        None => None
    }).cloned()
    .collect::<HashSet<_>>();

    println!("{}", removable_bricks.len());

    removable_bricks.into_iter().map(|removable_brick|
        count_falling_bricks(removable_brick, support_structure.clone())
    ).sum()

}

#[allow(dead_code)]
fn display(bricks: &Bricks) {
    for z in (0..7).rev() {
        for y in 0..3 {
            for x in 0..3 {
                if bricks.iter().any(|brick| brick.borrow().x.contains(&x) && brick.borrow().y.contains(&y) && brick.borrow().z.contains(&z) ) {
                    print!("#");
                } else {
                    print!(".");
                }
            }
            println!()
        }
        println!()
    }
}

fn count_falling_bricks(removed_brick: BrickRef, mut support_structure: HashMap<BrickRef, Option<Bricks>>) -> u32 {
    let mut falling_bricks = vec![removed_brick.clone()];
    let mut brick_count = 0;
    dbg!(removed_brick);

    while !falling_bricks.is_empty() {
        
        // find bricks supported only by falling bricks, or by bricks which do not exist anymore
        let new_falling_bricks = support_structure.iter()
        .filter_map(|(brick, supporting_bricks)| supporting_bricks.as_ref().map(|sb |(brick, sb)) )
        .filter_map(|(brick, supporting_bricks)|
            ( supporting_bricks.iter().all( |supporting_brick| support_structure.contains_key(supporting_brick).not() || falling_bricks.contains(supporting_brick) ) ).then_some(brick)
        ).cloned().collect::<Vec<_>>();
        dbg!(&new_falling_bricks);

        dbg!(brick_count);
        for falling_brick in falling_bricks {
            support_structure.remove(&falling_brick);
        }        

        falling_bricks = new_falling_bricks;
        brick_count += falling_bricks.len();
    }
    brick_count as u32 / 2
}

fn analyse_support_structure(bricks: &Bricks) -> HashMap<BrickRef, Option<Bricks>> {
    bricks.iter().map(|brick| (brick.clone(), get_supporting_bricks(brick.clone(), bricks))).collect()
}

fn stabilize(bricks: &Bricks) {
    while !is_stable(bricks) {
        let mut newer_bricks = bricks.clone();
        for brick in newer_bricks.iter_mut() {
            if get_supporting_bricks(brick.clone(), bricks).is_some_and(|bs| bs.is_empty()) {
                brick.borrow_mut().move_down();
            }
        }
    }
}

fn is_stable(bricks: &Bricks) -> bool {
    for brick in bricks.iter() {
        if get_supporting_bricks(brick.clone(), bricks).is_some_and(|bs| bs.is_empty()) {
            return false;
        }
    }
    true
}

fn get_supporting_bricks(brick: BrickRef, bricks: &Bricks) -> Option<Bricks> {
    let mut supporting_bricks = vec![];
    if let Some(space) = space_below_brick(brick.clone()) {
        for other_brick in bricks {
            if brick == *other_brick { continue };
            if intersect(&space, other_brick.clone()) { supporting_bricks.push(other_brick.clone()) };
        }
    } else {
        return None;
    }
    Some(supporting_bricks)
}

fn space_below_brick(brick: BrickRef) -> Option<Brick> {
    let brick = brick.borrow().clone();
    if brick.z.start == 1 { return None };
    Some(Brick {
        z: brick.z.start - 1 .. brick.z.end -1,
        ..brick
    })
}

fn intersect(lhs: &Brick, rhs: BrickRef) -> bool {
    let rhs = rhs.borrow();
    overlap(&lhs.x, &rhs.x) && overlap(&lhs.y, &rhs.y) && overlap(&lhs.z, &rhs.z)
}

fn overlap<Idx>(lhs: &Range<Idx>, rhs: &Range<Idx>) -> bool where Idx: Ord {
    lhs.start < rhs.end && rhs.start < lhs.end
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
        assert_eq!(result, 7);
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

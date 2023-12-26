use glam::DVec2;
use nom::bytes::complete::tag;
use nom::character::complete::{self, space1, newline};
use nom::multi::separated_list1;
use nom::sequence::{Tuple, separated_pair, terminated, preceded};
use nom::IResult;
use itertools::Itertools;

pub fn process<'a>(
    input: &'a str, lower_bound: f64, upper_bound: f64
) -> u32 {
    let (_, hailstones) = separated_list1(newline,hailstone)(input).expect("should parse");

    hailstones.into_iter().combinations(2).filter_map(|h| {
        collision_point(&h[0], &h[1])
 
    })
    .filter(|collision_point| lower_bound <= collision_point.x &&
        lower_bound <= collision_point.y &&
        collision_point.x <= upper_bound &&
        collision_point.y <= upper_bound)
    .count() as u32
}

fn collision_point(h0: &HailStone, h1: &HailStone) -> Option<DVec2> {
    let denominator = h0.velocity.perp_dot(h1.velocity);
    if dbg!(denominator) == 0.0 { println!("Singular matrix!"); return None };
    
    let t0 = (h1.position - h0.position).perp_dot(h1.velocity) / denominator;
    if dbg!(t0) < 0.0 { println!("Colision in past for h0!"); return None };

    let t1 = (h1.position - h0.position).perp_dot(h0.velocity) / denominator;
    if dbg!(t1) < 0.0 { println!("Collisionn in past for h1!"); return None };

    let collision_point = h0.position + t0 * h0.velocity;
    Some(dbg!(collision_point))
}


#[derive(PartialEq, Debug, Clone)]
struct HailStone {
    position: DVec2,
    velocity: DVec2
}

impl HailStone {
    fn new(px: f64, py: f64, _: f64, vx: f64, vy: f64, _: f64) -> Self{
        Self {
            position: DVec2 { x: px, y: py },
            velocity: DVec2 { x: vx, y: vy }
        }
    }
}

// 19, 13, 30 @ -2,  1, -2
fn hailstone(input: &str) -> IResult<&str, HailStone> {
    let (input, ((px,py,pz),(vx,vy,vz))) =  
        separated_pair(
            |input| (terminated(complete::i64, preceded(tag(","), space1)), 
            terminated(complete::i64, preceded(tag(","), space1)), 
            complete::i64).parse(input),
            tag(" @ "), 
            |input| (terminated(complete::i64, preceded(tag(","), space1)), 
            terminated(complete::i64, preceded(tag(","), space1)), 
            complete::i64).parse(input)
            )(input)?;

    Ok((
        input, 
        HailStone::new(px as f64, py as f64, pz as f64, vx as f64, vy as f64, vz as f64)
    ))
}

#[cfg(test)]
mod tests {
    use super::*;

    use rstest::rstest;

    #[rstest]
    #[case(HailStone::new(19.0, 13.0, 30.0, -2.0, 1.0, -2.0), HailStone::new(18.0, 19.0, 22.0, -1.0, -1.0, -2.0), true)]
    #[case(HailStone::new(19.0, 13.0, 30.0, -2.0, 1.0, -2.0), HailStone::new(20.0, 25.0, 34.0, -2.0, -2.0, -4.0), true)]
    #[case(HailStone::new(19.0, 13.0, 30.0, -2.0, 1.0, -2.0), HailStone::new(12.0, 31.0, 28.0, -1.0, -2.0, -1.0), true)]
    #[case(HailStone::new(19.0, 13.0, 30.0, -2.0, 1.0, -2.0), HailStone::new(20.0, 19.0, 15.0,  1.0, -5.0, -3.0), false)]
    fn test_crossing_point(
        #[case] h0: HailStone, 
        #[case] h1: HailStone, 
        #[case] expected: bool
    ) {
        let result = collision_point(&h0, &h1);
        assert_eq!(result.is_some(), expected)
    }

    #[rstest]
    #[case("19, 13, 30 @ -2,  1, -2", HailStone::new(19.0, 13.0, 30.0, -2.0,  1.0, -2.0))]
    fn test_hailstone(
        #[case] input: &str,
        #[case] expected: HailStone
    ) {
        let (input, hail_stone) = hailstone(input).expect("Should Parse");
        assert_eq!(input, "");
        assert_eq!(hail_stone, expected);
    }

    #[test]
    fn test_process() {
        let lines = "19, 13, 30 @ -2,  1, -2
18, 19, 22 @ -1, -1, -2
20, 25, 34 @ -2, -2, -4
12, 31, 28 @ -1, -2, -1
20, 19, 15 @  1, -5, -3";
        let result = process(lines, 7.0, 27.0);
        assert_eq!(result, 2);
    }
}

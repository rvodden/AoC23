use nalgebra::{Vector3, Matrix6, Vector6};
use nom::bytes::complete::tag;
use nom::character::complete::{self, space1, newline};
use nom::multi::separated_list1;
use nom::sequence::{Tuple, separated_pair, terminated, preceded};
use nom::IResult;

pub fn process(
    input: &str, _: f64, _: f64
) -> f64 {
    let (_, h) = separated_list1(newline,hailstone)(input).expect("should parse");

    let n = 0; let m = 1; let l = 2;

    let v_n = h[n].velocity;
    let v_m = h[m].velocity;
    let v_l = h[l].velocity;

    let p_n = h[n].position;
    let p_m = h[m].position;
    let p_l = h[l].position;

    // see writup.ipynb for what on earth is going on here :-)
    let s = Matrix6::new(
             0.0      , v_n.z - v_m.z,  v_m.y - v_n.y,       0.0     ,  p_m.z - p_n.z, p_n.y - p_m.y,
        v_m.z - v_n.z ,      0.0     ,  v_n.x - v_m.x,  p_n.z - p_m.z,      0.0      , p_m.x - p_n.x,
        v_n.y - v_m.y,  v_m.x - v_n.x,       0.0     ,  p_m.y - p_n.y,  p_n.x - p_m.x,      0.0,
             0.0      , v_n.z - v_l.z,  v_l.y - v_n.y,       0.0     ,  p_l.z - p_n.z, p_n.y - p_l.y,
        v_l.z - v_n.z ,      0.0     ,  v_n.x - v_l.x,  p_n.z - p_l.z,      0.0      , p_l.x - p_n.x,
        v_n.y - v_l.y,  v_l.x - v_n.x,       0.0     ,  p_l.y - p_n.y,  p_n.x - p_l.x,      0.0,
    );

    let t = Vector6::new(
        p_m.y * v_m.z + p_n.z * v_n.y - p_m.z * v_m.y - p_n.y * v_n.z,
        p_m.z * v_m.x + p_n.x * v_n.z - p_m.x * v_m.z - p_n.z * v_n.x,
        p_m.x * v_m.y + p_n.y * v_n.x - p_m.y * v_m.x - p_n.x * v_n.y,
        p_l.y * v_l.z + p_n.z * v_n.y - p_l.z * v_l.y - p_n.y * v_n.z,
        p_l.z * v_l.x + p_n.x * v_n.z - p_l.x * v_l.z - p_n.z * v_n.x,
        p_l.x * v_l.y + p_n.y * v_n.x - p_l.y * v_l.x - p_n.x * v_n.y,
    );

    let answer = s.try_inverse().unwrap() * t;
    let answer = Vector3::from(answer.fixed_view::<3, 1>(0, 0));
    dbg!(answer);

    answer.sum()

}

#[derive(PartialEq, Debug, Clone)]
struct HailStone {
    position: Vector3<f64>,
    velocity: Vector3<f64>
}

impl HailStone {
    fn new(px: f64, py: f64, pz: f64, vx: f64, vy: f64, vz: f64) -> Self{
        Self {
            position: Vector3::new(px, py, pz),
            velocity: Vector3::new(vx, vy, vz)
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
        assert_eq!(result, 47.0);
    }
}

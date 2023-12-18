use glam::I64Vec2;
use itertools::Itertools;

pub fn process<'a>(
    input: &'a str,
) -> i64 {
    let instructions = input.lines().map(|line| {
        let (_, instruction) = line.split("#").collect_tuple().unwrap();
        let distance = &instruction[0..5];
        let direction = &instruction[5..6];
        Instruction{
            direction: match direction {
                "2" => I64Vec2::NEG_X,
                "0" => I64Vec2::X,
                "3" => I64Vec2::NEG_Y,
                "1" => I64Vec2::Y,
                value => panic!("Got invalid character: {}", value)
            },
            distance: i64::from_str_radix(&distance, 16).unwrap(),
        }
    });
    
    let path = instructions.clone().scan(I64Vec2::ZERO, |state, instruction|{
        *state = *state + instruction.direction * instruction.distance;
        Some(state.clone())
    }).collect::<Vec<_>>();

    let pairs_of_points = path.iter().zip(path.iter().skip(1));
    
    let perimeter = instructions.map(|x| x.distance).sum::<i64>();

    dbg!(perimeter);

    let area = pairs_of_points.fold(0,
    |area: i64, (a, b): (&I64Vec2, &I64Vec2)| {
        area + b.perp_dot(*a)
    }).abs() / 2;

    area + (perimeter / 2) + 1
}

#[derive(Debug)]
struct Instruction {
    direction: I64Vec2,
    distance: i64,
}


#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_process() {
        let lines = "R 6 (#70c710)
D 5 (#0dc571)
L 2 (#5713f0)
D 2 (#d2c081)
R 2 (#59c680)
D 2 (#411b91)
L 5 (#8ceee2)
U 2 (#caa173)
L 1 (#1b58a2)
U 2 (#caa171)
R 2 (#7807d2)
U 3 (#a77fa3)
L 2 (#015232)
U 2 (#7a21e3)";
        let result = process(lines);
        assert_eq!(result, 952408144115);
    }
}

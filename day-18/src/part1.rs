use glam::IVec2;
use itertools::Itertools;

pub fn process<'a>(
    input: &'a str,
) -> i32 {
    let instructions = input.lines().map(|line| {
        let (direction, distance, _) = line.split(" ").collect_tuple().unwrap();
        Instruction{
            direction: match direction {
                "L" => IVec2::NEG_X,
                "R" => IVec2::X,
                "U" => IVec2::NEG_Y,
                "D" => IVec2::Y,
                value => panic!("Got invalid character: {}", value)
            },
            distance: distance.parse().unwrap()
        }
    });
    
    let path = instructions.clone().scan(IVec2::ZERO, |state, instruction|{
        *state = *state + instruction.direction * instruction.distance;
        Some(state.clone())
    }).collect::<Vec<_>>();

    let pairs_of_points = path.iter().zip(path.iter().skip(1));
    
    let perimeter = instructions.map(|x| x.distance).sum::<i32>();

    dbg!(perimeter);

    let area = pairs_of_points.fold(0,
    |area: i32, (a, b): (&IVec2, &IVec2)| {
        area + b.perp_dot(*a)
    }).abs() / 2;

    area + (perimeter / 2) + 1
}

#[derive(Debug)]
struct Instruction {
    direction: IVec2,
    distance: i32,
}


#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_little_process() {
        let lines = "R 6 (#70c710)
D 2 (#0dc571)
L 6 (#5713f0)
U 2 (#d2c081)";
        let result = process(lines);
        assert_eq!(result, 21);
    }

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
        assert_eq!(result, 62);
    }
}

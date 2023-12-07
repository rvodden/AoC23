use std::iter::zip;

use nom::{
    bytes::complete::tag,
    character::complete::{self, newline, space1},
    multi::separated_list1,
    sequence::{pair, preceded, separated_pair},
    IResult,
};

pub fn process<'a>(input: &'a str) -> u32 {
    let (_, races) = races(input).expect("should parse");
    races.iter().map(number_of_solutions).product()
}

fn number_of_solutions(race: &Race) -> u32 {
    (0..=race.allowed_time)
        .map(|t| distance_travelled(t, race.allowed_time))
        .filter(|t| *t > race.record_distance)
        .count() as u32
}

fn distance_travelled(acceleration_time: u32, total_time: u32) -> u32 {
    (total_time - acceleration_time) * acceleration_time
}

#[derive(PartialEq, Debug)]
struct Race {
    allowed_time: u32,
    record_distance: u32,
}

fn races(input: &str) -> IResult<&str, Vec<Race>> {
    let (input, (times, distances)) =
        separated_pair(times, newline, distances)(input).expect("to parse");

    Ok((
        input,
        zip(times, distances)
            .map(|(allowed_time, record_distance)| Race {
                allowed_time,
                record_distance,
            })
            .collect(),
    ))
}

// Time:      7  15   30
fn times(input: &str) -> IResult<&str, Vec<u32>> {
    preceded(
        pair(tag("Time:"), space1),
        separated_list1(space1, complete::u32),
    )(input)
}

// Distance:  9  40  200
fn distances(input: &str) -> IResult<&str, Vec<u32>> {
    preceded(
        pair(tag("Distance:"), space1),
        separated_list1(space1, complete::u32),
    )(input)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_races() {
        let result = races(
            "Time:      7  15   30
Distance:  9  40  200",
        )
        .expect("should parse");
        assert_eq!(result.0, "");
        assert_eq!(
            result.1,
            vec![
                Race {
                    allowed_time: 7,
                    record_distance: 9
                },
                Race {
                    allowed_time: 15,
                    record_distance: 40
                },
                Race {
                    allowed_time: 30,
                    record_distance: 200
                },
            ]
        );
    }

    #[test]
    fn test_time() {
        let result = times("Time:      7  15   30").expect("should parse");
        assert_eq!(result.0, "");
        assert_eq!(result.1, vec![7, 15, 30]);
    }

    #[test]
    fn test_ditance() {
        let result = distances("Distance:      9  40  200").expect("should parse");
        assert_eq!(result.0, "");
        assert_eq!(result.1, vec![9, 40, 200]);
    }

    #[test]
    fn test_process() {
        let lines = "Time:      7  15   30
Distance:  9  40  200";
        let result = process(lines);
        assert_eq!(result, 288);
    }
}

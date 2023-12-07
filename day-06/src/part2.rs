use nom::{
    bytes::complete::tag,
    character::complete::{newline, space1, not_line_ending},
    sequence::{pair, preceded, separated_pair},
    IResult,
};

pub fn process<'a>(input: &'a str) -> u64 {
    let (_, race) = race(input).expect("should parse");
    number_of_solutions(&race)
}

fn number_of_solutions(race: &Race) -> u64 {
    (0..=race.allowed_time)
        .map(|t| distance_travelled(t, race.allowed_time))
        .filter(|t| *t > race.record_distance)
        .count() as u64
}

fn distance_travelled(acceleration_time: u64, total_time: u64) -> u64 {
    (total_time - acceleration_time) * acceleration_time
}

#[derive(PartialEq, Debug)]
struct Race {
    allowed_time: u64,
    record_distance: u64,
}

fn race(input: &str) -> IResult<&str, Race> {
    let (input, (allowed_time, record_distance)) =
        separated_pair(times, newline, distances)(input).expect("to parse");

    Ok((
        input,
        Race {
            allowed_time,
            record_distance,
        }
    ))
}

// Time:      7  15   30
fn times(input: &str) -> IResult<&str, u64> {
    let result : Result<(_,_), nom::Err<nom::error::Error<_>>>= preceded(
        pair(tag("Time:"), space1),
        not_line_ending,
    )(input);
    let (input, chars) = result.expect("should parse");

    Ok((input, chars
    .chars()
    .filter(|c| !c.is_whitespace()).collect::<String>().parse().expect("Should be digit")))
}

// Distance:  9  40  200
fn distances(input: &str) -> IResult<&str, u64> {
    let result : Result<(_,_), nom::Err<nom::error::Error<_>>>= preceded(
        pair(tag("Distance:"), space1),
        not_line_ending,
    )(input);
    let (input, chars) = result.expect("should parse");

    Ok((input, chars
    .chars()
    .filter(|c| !c.is_whitespace()).collect::<String>().parse().expect("Should be digit")))
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_race() {
        let result = race(
            "Time:      7  15   30
Distance:  9  40  200",
        )
        .expect("should parse");
        assert_eq!(result.0, "");
        assert_eq!(
            result.1,
            Race {
                allowed_time: 71530,
                record_distance: 940200
            },
        );
    }

    #[test]
    fn test_time() {
        let result = times("Time:      7  15   30").expect("should parse");
        assert_eq!(result.0, "");
        assert_eq!(result.1, 71530);
    }

    #[test]
    fn test_distance() {
        let result = distances("Distance:      9  40  200").expect("should parse");
        assert_eq!(result.0, "");
        assert_eq!(result.1, 940200);
    }

    #[test]
    fn test_process() {
        let lines = "Time:      7  15   30
Distance:  9  40  200";
        let result = process(lines);
        assert_eq!(result, 71503);
    }
}

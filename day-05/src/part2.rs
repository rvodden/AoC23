use std::ops::RangeInclusive;

use nom::{
    bytes::complete::tag,
    character::complete::{self, alpha1, newline, space1},
    multi::{separated_list1, many1},
    sequence::{pair, preceded, separated_pair, terminated, tuple},
    IResult,
};

pub fn process(input: &str) -> u64 {
    let (_, (seeds, range_maps)) = parse_input(input).expect("Should Parse");
    *seeds.into_iter().flat_map(|value|
        follow_map(vec![value], "seed", range_maps.clone()).0
    )
    .min_by(|a, b| a.start().cmp(b.start()))
    .unwrap()
    .start()
}

fn follow_map<'a>(ranges: Vec<RangeInclusive<u64>>, destination: &'a str, range_maps: Vec<RangeMap<'a>>) -> (Vec<RangeInclusive<u64>>, &'a str) {
    // print!("{} {:10} ", destination, value);
    if let Some(range_map) = range_maps.iter().find(|range_map| range_map.source == destination) {
        let new_ranges = ranges.iter().flat_map(|range| range_map.get(range.clone())).collect();
        let new_destination = range_map.destination;
        follow_map(new_ranges, new_destination, range_maps)
    } else {
        // println!();
        (ranges, destination)
    }
}


#[derive(Debug, PartialEq, Clone)]
struct RangeMap<'a> {
    pub source: &'a str,
    pub destination: &'a str,
    data: Vec<(RangeInclusive<u64>, u64)>,
}

impl RangeMap<'_> {
    #[allow(dead_code)]
    fn new<'a>(source: &'a str, destination: &'a str) -> RangeMap<'a> {
        RangeMap {
            source,
            destination,
            data: Vec::new(),
        }
    }

    #[allow(dead_code)]
    fn insert(&mut self, range: RangeInclusive<u64>, value: u64) {
        self.data.push((range, value));
    }

    pub fn get(&self, index_range: RangeInclusive<u64>) -> Vec<RangeInclusive<u64>> {
        // Either the lower bound is included in a mapped range, or not.
        if let Some((range, value)) = self.data.iter().find(|(range, _)| range.contains(index_range.start())) {
            if index_range.end() <= range.end() { // our range is entirely included within a range, so we are done.
                let start = range.start();
                vec![(value + index_range.start() - start)..=(value + index_range.end() - start)]
            } else { // the first part of our range is included in a range, chop that off and recur
                let start = range.start();
                let mut retval = vec![(value + index_range.start() - start)..=(value + range.end() - start)];
                retval.append(&mut self.get((*range.end() + 1)..=*index_range.end()));
                retval
            }
        } else {
            match self.data.iter().find(|(range, _)| index_range.start() < range.start() && range.start() < index_range.end() ) {
                Some((range, _)) => {
                    // there is an unmapped chunk at the start of our range, chop that off and recur
                    let mut retval = vec![*index_range.start()..=(range.start() - 1)];
                    retval.append(&mut self.get(*range.start()..=*index_range.end()));
                    retval

                }
                None => { // our range doesn't intersect with any mapped ranges, so we are done.
                    vec![index_range]
                }
            }
        }
    }

    pub fn map(&self, index: u64) -> u64 {
        for (inclusive_range, value) in self.data.iter() {
            if inclusive_range.contains(&index) {
                if let Some(min) = inclusive_range.clone().min() {
                    return value + (index - min);
                }
            }
        }
        index
    }
}

fn parse_input(input: &str) -> IResult<&str, (Vec<RangeInclusive<u64>>, Vec<RangeMap>)> {
    separated_pair(
        seeds, 
        pair(
            newline,
            newline
        ),
        separated_list1(
            newline,
            range_map
        )
    )
    (input)
}

// seeds: 79 14 55 13
fn seeds(input: &str) -> IResult<&str, Vec<RangeInclusive<u64>>> {
    let (input, ranges) = preceded(
        tag("seeds: "),
        separated_list1(
            space1,
            separated_pair(complete::u64, space1, complete::u64)
        )
    )(input)?;
    Ok((input, ranges.into_iter().map(|(start, length)| start..=(start + length - 1)).collect()))
}

fn range_map(input: &str) -> IResult<&str, RangeMap> {
    let (input, ((source, destination), inclusive_range_values)) = separated_pair(
        range_map_title,
        newline,
        many1(inclusive_range_value),
    )(input)?;

    let mut range_map = RangeMap {
        source,
        destination,
        data: Vec::<(RangeInclusive<u64>, u64)>::new(),
    };

    for inclusive_range_value in inclusive_range_values {
        range_map.data.push(inclusive_range_value)
    }

    Ok((input, range_map))
}

// seed-to-soil map:
fn range_map_title(input: &str) -> IResult<&str, (&str, &str)> {
    terminated(
        separated_pair(alpha1, tag("-to-"), alpha1),
        pair(space1, tag("map:")),
    )(input)
}

// 50 98 2
fn inclusive_range_value(input: &str) -> IResult<&str, (RangeInclusive<u64>, u64)> {
    let (input, (value, min, length)) = tuple((terminated(complete::u64, space1), terminated(complete::u64, space1), terminated(complete::u64, newline)))(input)?;

    Ok((
        input,
        (
            min..=(min + length - 1),
            value,
        ),
    ))
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_seeds() {
        let result = seeds("seeds: 79 14 55 13").expect("Should parse");
        assert_eq!(result.0, "");
        assert_eq!(result.1, vec![79..=82, 55..=67]);
        let result = seeds("seeds: 79 14 55 13

seed-to-soil map:
50 98 2
52 50 48").expect("Should parse");
    }

    #[test]
    fn test_input() {
        let result = separated_list1(newline, range_map)("seed-to-soil map:
50 98 2
52 50 48

soil-to-fertilizer map:
0 15 37
37 52 2
39 0 15

fertilizer-to-water map:
49 53 8
0 11 42
42 0 7
57 7 4

water-to-light map:
88 18 7
18 25 70

light-to-temperature map:
45 77 23
81 45 19
68 64 13

temperature-to-humidity map:
0 69 1
1 0 69

humidity-to-location map:
60 56 37
56 93 4").expect("should parse");
    }

    #[test]
    fn test_range_map_again() {
        let (input, result) = range_map(
            "seed-to-soil map:
50 98 2
52 50 48
",
        )
        .expect("should parse");
        assert_eq!(input, "");
        dbg!(result);
    }
    
    #[test]
    fn test_range_map() {
        let (input, result) = range_map(
            "seed-to-soil map:
3788621315 24578909 268976974
3633843608 2672619957 154777707
1562003446 2827397664 767899879
2618130896 293555883 1015712712
178572254 3595297543 462300746
640873000 1553961386 921130446
2373438105 1435027522 118933864
2492371969 1309268595 125758927
2329903325 2629085177 43534780
24578909 2475091832 153993345
",
        )
        .expect("should parse");
        assert_eq!(input, "");
        dbg!(&result);
        dbg!(result.map(929142010));
    }

    #[test]
    fn test_range_map_title() {
        let result = range_map_title("seed-to-soil map:").expect("should parse");
        assert_eq!(result.0, "");
        assert_eq!(result.1, ("seed", "soil"));
    }

    #[test]
    fn test_range_map_get() {
        let mut under_test = RangeMap::new("thing", "otherthing");
        under_test.insert(98..=99, 50);
        under_test.insert(
            50..=(50 + 48 - 1),
            52,
        );

        assert_eq!(under_test.get(1..=10), vec![1..=10]);
        assert_eq!(under_test.get(2..=51), vec![2..=49,52..=53]);
        assert_eq!(under_test.get(48..=100), vec![48..=97,50..=51, 100..=100]);

    }

    #[test]
    fn test_range_map_map() {
        let mut under_test = RangeMap::new("thing", "otherthing");
        under_test.insert(98..=99, 50);
        under_test.insert(
            50..=(50 + 48 - 1),
            52,
        );

        assert_eq!(under_test.map(98), 50);
        assert_eq!(under_test.map(99), 51);

        assert_eq!(under_test.map(53), 55);
        assert_eq!(under_test.map(10), 10);
    }

    #[test]
    fn test_process() {
        let lines = "seeds: 79 14 55 13

seed-to-soil map:
50 98 2
52 50 48

soil-to-fertilizer map:
0 15 37
37 52 2
39 0 15

fertilizer-to-water map:
49 53 8
0 11 42
42 0 7
57 7 4

water-to-light map:
88 18 7
18 25 70

light-to-temperature map:
45 77 23
81 45 19
68 64 13

temperature-to-humidity map:
0 69 1
1 0 69

humidity-to-location map:
60 56 37
56 93 4";
        let result = process(lines);
        assert_eq!(result, 46);
    }
}

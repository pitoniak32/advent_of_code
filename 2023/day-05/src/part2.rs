use std::ops::Range;

use anyhow::Result;
use nom::{
    bytes::complete::take_until,
    character::complete::{self, line_ending, space1},
    multi::{many1, separated_list1},
    sequence::{separated_pair, tuple},
    IResult, Parser,
};
use nom_supreme::{tag::complete::tag, ParserExt};

// 125742456 = just right
impl Almanac {
    fn calc_min_loc(&self) -> u64 {
        let locs = self
            .seeds
            .iter()
            .flat_map(|seed_range| seed_range.clone())
            .map(|seed| {
                self.entries
                    .iter()
                    .fold(seed, |acc, entry| entry.get_dest(acc))
            })
            .collect::<Vec<u64>>();

        *locs.iter().min().expect("should have a min")
    }
}

#[derive(Clone, Debug)]
pub struct Almanac {
    pub seeds: Vec<Range<u64>>,
    pub entries: Vec<Entry>,
}

#[derive(Clone, Debug, PartialEq)]
pub struct Entry {
    pub ranges: Vec<(Range<u64>, Range<u64>)>,
}

impl Entry {
    fn get_dest(&self, source: u64) -> u64 {
        let valid = self.ranges.iter().find(|(sr, _)| sr.contains(&source));

        let Some((sr, dr)) =
            valid
        else {
            return source;
        };

        let offset = source - sr.start;
        dr.start + offset
    }
}

pub fn parse_entry(input: &str) -> IResult<&str, Entry> {
    let (o, items) = take_until("map:")
        .precedes(tag("map:"))
        .precedes(
            many1(
                line_ending.precedes(
                    tuple((
                        complete::u64,
                        complete::u64.preceded_by(tag(" ")),
                        complete::u64.preceded_by(tag(" ")),
                    ))
                    .map(|(dest, source, len)| (source..source + len, dest..dest + len)),
                ),
            )
            .map(|ranges| Entry { ranges }),
        )
        .parse(input)?;

    Ok((o, items))
}

fn parse_seeds_entries(input: &str) -> IResult<&str, (Vec<Range<u64>>, Vec<Entry>)> {
    let (input, seeds) = tag("seeds: ")
        .precedes(separated_list1(
            space1,
            separated_pair(complete::u64, tag(" "), complete::u64)
                .map(|(start, offset)| start..(start + offset)),
        ))
        .parse(input)?;
    let (input, maps) = many1(parse_entry)(input)?;

    Ok((input, (seeds, maps)))
}

pub fn parse_almanac(input: &str) -> IResult<&str, Almanac> {
    let (input, (seeds, entries)) = parse_seeds_entries(input)?;
    Ok((input, Almanac { seeds, entries }))
}

pub fn process(input: &'static str) -> Result<String> {
    let (_, almanac) = parse_almanac(input)?;

    let loc = almanac.calc_min_loc();

    Ok(loc.to_string())
}

#[cfg(test)]
mod tests {
    use super::*;
    use pretty_assertions::assert_eq;

    #[test]
    fn test_process() -> Result<()> {
        let input = "seeds: 79 14 55 13

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
        assert_eq!(process(input)?, "46");
        Ok(())
    }
}

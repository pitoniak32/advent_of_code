use std::{collections::HashMap, ops::Range};

use anyhow::Result;
use nom::{
    bytes::complete::{tag, take_while},
    character::complete::{line_ending, space1, u32, u64},
    multi::separated_list1,
    sequence::{preceded, separated_pair},
    IResult,
};

// 196_167_384
impl<'a> Almanac<'a> {
    fn calc_min_loc(&mut self) -> u64 {
        let mut locs = vec![];
        for seed in self.seeds.clone() {
            dbg!(&seed);
            let soil = self
                .entries
                .get_mut("seed-to-soil")
                .expect("should have s-2-s entry")
                .get_dest(seed as u64);
            dbg!(&soil);
            let fert = self
                .entries
                .get_mut("soil-to-fertilizer")
                .expect("should have s-2-f entry")
                .get_dest(soil);
            dbg!(fert);
            let water = self
                .entries
                .get_mut("fertilizer-to-water")
                .expect("should have f-2-w entry")
                .get_dest(fert);
            dbg!(water);
            let light = self
                .entries
                .get_mut("water-to-light")
                .expect("should have w-2-l entry")
                .get_dest(water);
            dbg!(light);
            let temp = self
                .entries
                .get_mut("light-to-temperature")
                .expect("should have l-2-t entry")
                .get_dest(light);
            dbg!(temp);
            let humidity = self
                .entries
                .get_mut("temperature-to-humidity")
                .expect("should have t-2-h entry")
                .get_dest(temp);
            dbg!(humidity);
            let loc = self
                .entries
                .get_mut("humidity-to-location")
                .expect("should have h-2-l entry")
                .get_dest(humidity);
            dbg!(loc);
            locs.push(loc);
        }
        *locs.iter()
            .min()
            .expect("should have at least one value")
    }
}

#[derive(Clone, Debug)]
pub struct Almanac<'a> {
    pub seeds: Vec<u32>,
    pub entries: HashMap<&'a str, Entry>,
}

#[derive(Clone, Debug, PartialEq)]
pub struct Entry {
    pub ranges: Vec<(Range<u64>, Range<u64>)>,
    pub entries: Vec<Vec<u64>>,
}

impl Entry {
    fn new(entries: Vec<Vec<u64>>) -> Self {
        let mut final_ranges: Vec<(Range<u64>, Range<u64>)> = Vec::new();
        for value in entries.iter() {
            let dest_start = *value.first().expect("should have dest start");
            let source_start = *value.get(1).expect("should have source start");
            let range_len = *value.get(2).expect("should have range len");

            let source_range = source_start..source_start + range_len;
            let dest_range = dest_start..dest_start + range_len;

            final_ranges.push((source_range, dest_range));
        }

        Entry {
            entries,
            ranges: final_ranges,
        }
    }

    fn get_dest(&mut self, source: u64) -> u64 {
        let valid = self.ranges.iter().find(|(sr, _)| sr.contains(&source));

        if let Some((sr, dr)) = valid {
            let offset = source - sr.start;
            dr.start + offset
        } else {
            source
        }
    }
}

pub fn parse_entries(input: &str) -> IResult<&str, HashMap<&str, Entry>> {
    let (o, items) = separated_list1(
        tag("\n\n"),
        separated_pair(
            take_while(|c| c != ' '),
            tag(" map:\n"),
            separated_list1(line_ending, separated_list1(space1, u64)),
        ),
    )(input)?;

    let folded = items.iter().fold(
        HashMap::new(),
        |mut acc: HashMap<&str, Entry>, (title, value)| {
            let _ = acc.insert(title, Entry::new(value.clone()));
            acc
        },
    );

    Ok((o, folded))
}

pub fn parse_seeds(input: &str) -> IResult<&str, Vec<u32>> {
    let (o, seeds) = preceded(tag("seeds: "), separated_list1(space1, u32))(input)?;
    dbg!(&seeds);
    Ok((o, seeds))
}

pub fn parse_almanac(input: &str) -> IResult<&str, Almanac> {
    let (input, (seeds, entries)) = separated_pair(parse_seeds, tag("\n\n"), parse_entries)(input)?;
    Ok((input, Almanac { seeds, entries }))
}

pub fn process(input: &'static str) -> Result<String> {
    let (_, mut almanac) = parse_almanac(input)?;

    let loc = almanac.calc_min_loc();

    Ok(loc.to_string())
}

#[cfg(test)]
mod tests {
    use super::*;
    use pretty_assertions::assert_eq;

    #[test]
    fn test_parse_seeds() -> Result<()> {
        let input = "seeds: 79 14 55 13";
        let (_, result) = parse_seeds(input)?;
        assert_eq!(vec![79, 14, 55, 13], result);
        Ok(())
    }

    #[test]
    fn test_parse_entry() -> Result<()> {
        // Arrange
        let input = "seed-to-soil map:
50 98 2
52 50 5

";
        let mut expected = HashMap::new();
        let ranges = vec![(98..100, 50..52), (50..55, 52..57)];
        expected.insert(
            "seed-to-soil",
            Entry {
                entries: vec![vec![50, 98, 2], vec![52, 50, 5]],
                ranges,
            },
        );

        // Act
        let (_, result) = parse_entries(input)?;

        // Assert
        assert_eq!(result, expected);
        Ok(())
    }

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
        assert_eq!(process(input)?, "35");
        Ok(())
    }
}

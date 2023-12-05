use std::collections::HashMap;

use anyhow::Result;
use nom::{
    bytes::complete::{tag, take_while},
    character::complete::{alpha1, digit1, line_ending, space1},
    multi::{fold_many1, separated_list1, many_till},
    sequence::{preceded, separated_pair, tuple, delimited},
    IResult, combinator::map,
};

impl<'a> Almanac<'a> {
    fn calc_min_loc(&self) -> String {
        "".to_string()
    }
}

#[derive(Clone, Debug)]
pub struct Almanac<'a> {
    pub seeds: Vec<&'a str>,
    pub entries: HashMap<&'a str, Vec<Vec<&'a str>>>, // seed_soil: AlmanacEntry,
                                                      // soil_fert: AlmanacEntry,
                                                      // fert_water: AlmanacEntry,
                                                      // water_light: AlmanacEntry,
                                                      // light_temp: AlmanacEntry,
                                                      // temp_humid: AlmanacEntry,
                                                      // humid_loc: AlmanacEntry,
}

pub fn parse_entries(input: &str) -> IResult<&str, HashMap<&str, Vec<Vec<&str>>>> {
    let (o, items) = separated_list1(
        tag("\n\n"),
        separated_pair(
            take_while(|c| c != ' '),
            tag(" map:\n"),
            separated_list1(line_ending, separated_list1(space1, digit1)),
        ),
    )(input)?;

    Ok((
        o,
        items.iter().fold(
            HashMap::new(),
            |mut acc: HashMap<&str, Vec<Vec<&str>>>, (title, value)| {
                acc.insert(title, value.clone());
                acc
            },
        ),
    ))
}

pub fn parse_seeds(input: &str) -> IResult<&str, Vec<&str>> {
    let (o, seeds) = preceded(tag("seeds: "), separated_list1(space1, digit1))(input)?;
    Ok((o, seeds))
}

pub fn parse_almanac(input: &str) -> IResult<&str, Almanac> {
    let (input, (seeds, entries)) = separated_pair(parse_seeds, tag("\n\n"), parse_entries)(input)?;
    Ok((input, Almanac { seeds, entries }))
}

pub fn process(input: &'static str) -> Result<String> {
    let (_, almanac) = parse_almanac(input)?;
    
    let loc = dbg!(almanac).calc_min_loc();

    Ok(loc)
}

#[cfg(test)]
mod tests {
    use super::*;
    use pretty_assertions::assert_eq;

    #[test]
    fn test_parse_seeds() -> Result<()> {
        let input = "seeds: 79 14 55 13";
        let (_, result) = parse_seeds(input)?;
        assert_eq!(vec!["79", "14", "55", "13"], result);
        Ok(())
    }

    #[test]
    fn test_parse_entry() -> Result<()> {
        // Arrange
        let input = "seed-to-soil map:
50 98 2
52 50 48

";

        let mut expected = HashMap::new();
        expected.insert(
            "seed-to-soil",
            vec![vec!["50", "98", "2"], vec!["52", "50", "48"]],
        );

        // Act
        let (_, result) = parse_entries(input)?;

        // Assert
        assert_eq!(expected, result);
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

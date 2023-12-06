use std::vec;

use anyhow::Result;
use nom::{
    branch::alt,
    bytes::complete::tag,
    character::complete::{self, line_ending, space1},
    multi::separated_list1,
    sequence::{preceded, separated_pair},
    IResult,
};

#[derive(Debug)]
pub struct Race {
    pub time_ms: u32,
    pub distance_mm: u32,
}

fn parse_races(input: &str) -> IResult<&str, Vec<Race>> {
    let (o, races) = separated_pair(
        preceded(
            preceded(tag("Time:"), space1),
            separated_list1(space1, complete::u32),
        ),
        line_ending,
        preceded(
            preceded(tag("Distance:"), space1),
            separated_list1(space1, complete::u32),
        ),
    )(input)?;

    dbg!(&races);
    let mut races_new: Vec<Race> = vec![];
    let times = races.0;
    let mut dists = races.1.iter();

    for time in times {
        races_new.push(Race {
            time_ms: time,
            distance_mm: *dists.next().expect("has same dist count as times"),
        });
    }

    Ok((o, races_new))
}

pub fn process(input: &str) -> Result<String> {
    let (_, races) = parse_races(input).expect("valid parse");

    dbg!(&races);
    Ok("".to_string())
}

#[cfg(test)]
mod tests {
    use super::*;
    use pretty_assertions::assert_eq;

    #[test]
    fn test_process() -> Result<()> {
        let input = "Time:      7  15   30
Distance:  9  40  200";
        assert_eq!("288", process(input)?);
        Ok(())
    }
}

use std::vec;

use anyhow::Result;
use nom::{
    bytes::complete::tag,
    character::complete::{self, line_ending, space1},
    multi::separated_list1,
    sequence::{preceded, separated_pair},
    IResult,
};

#[derive(Debug)]
pub struct Race {
    pub duration_ms: u32,
    pub record_distance_mm: u32,
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

    let mut races_new: Vec<Race> = vec![];
    let times = races.0;
    let mut dists = races.1.iter();

    for time in times {
        races_new.push(Race {
            duration_ms: time,
            record_distance_mm: *dists.next().expect("has same dist count as times"),
        });
    }

    Ok((o, races_new))
}

// 2065338 = just right
pub fn process(input: &str) -> Result<String> {
    let (_, races) = parse_races(input).expect("valid parse");

    let moe = races
        .iter()
        .map(|r| {
            (0..=r.duration_ms)
                .fold(Vec::<u32>::new(), |mut acc, ms_tick_mult| {
                    let dist = (r.duration_ms - ms_tick_mult) * ms_tick_mult;
                    if dist > r.record_distance_mm {
                        acc.push(dist);
                    }
                    acc
                })
                .len()
        })
        .collect::<Vec<_>>()
        .iter()
        .product::<usize>();

    Ok(moe.to_string())
}

#[cfg(test)]
mod tests {
    use super::*;
    use pretty_assertions::assert_eq;

    #[test]
    fn test_process() -> Result<()> {
        let input = "Time:      7  15   30
Distance:  9  40  200";
        assert_eq!(process(input)?, "288");
        Ok(())
    }
}

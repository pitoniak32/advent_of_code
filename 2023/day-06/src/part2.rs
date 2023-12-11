use anyhow::Result;
use nom::{
    bytes::complete::tag,
    character::complete::{digit1, line_ending, space1},
    multi::separated_list1,
    sequence::{preceded, separated_pair},
    IResult,
};

#[derive(Debug)]
pub struct Race {
    pub duration_ms: u64,
    pub record_distance_mm: u64,
}

fn parse_race(input: &str) -> IResult<&str, Race> {
    let (o, races) = separated_pair(
        preceded(
            preceded(tag("Time:"), space1),
            separated_list1(space1, digit1),
        ),
        line_ending,
        preceded(
            preceded(tag("Distance:"), space1),
            separated_list1(space1, digit1),
        ),
    )(input)?;

    let times = races
        .0
        .join("")
        .parse::<u64>()
        .expect("time is valid number");
    let dist = races
        .1
        .join("")
        .parse::<u64>()
        .expect("dist is valid number");

    Ok((
        o,
        Race {
            duration_ms: times,
            record_distance_mm: dist,
        },
    ))
}

pub fn process(input: &str) -> Result<String> {
    let (_, race) = parse_race(input).expect("valid parse");

    dbg!(&race);
    let moe = (0..=race.duration_ms)
        .fold(Vec::<u128>::new(), |mut acc, ms_tick_mult| {
            let dist = (race.duration_ms - ms_tick_mult) * ms_tick_mult;
            if dist > race.record_distance_mm {
                acc.push(dist as u128);
            }
            acc
        })
        .len()
        .to_string();

    Ok(moe)
}

#[cfg(test)]
mod tests {
    use super::*;
    use pretty_assertions::assert_eq;

    #[test]
    fn test_process() -> Result<()> {
        let input = "Time:      7  15   30
Distance:  9  40  200";
        assert_eq!(process(input)?, "71503");
        Ok(())
    }
}

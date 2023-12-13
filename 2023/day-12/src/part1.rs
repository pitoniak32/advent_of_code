use anyhow::Result;
use nom::{
    bytes::{complete::tag, streaming::take_till1},
    character::complete::{self, line_ending},
    multi::separated_list1,
    sequence::separated_pair,
    IResult,
};

pub fn parse_arrangements(input: &str) -> IResult<&str, Vec<(Vec<char>, Vec<u32>)>> {
    let (o, arrangements) = separated_list1(
        line_ending,
        separated_pair(
            take_till1(|c| c == ' '),
            tag(" "),
            separated_list1(tag(","), complete::u32),
        ),
    )(input)?;

    Ok((
        o,
        arrangements
            .iter()
            .map(|a| (a.0.chars().collect(), a.1.clone()))
            .collect::<Vec<(Vec<char>, Vec<u32>)>>(),
    ))
}

pub fn process(input: &str) -> Result<String> {
    let (_, arrangements) = parse_arrangements(input).expect("valid parse");

    dbg!(arrangements);

    Ok("".to_string())
}

#[cfg(test)]
mod tests {
    use super::*;
    use pretty_assertions::assert_eq;

    #[test]
    fn test_process() -> Result<()> {
        let input = "???.### 1,1,3
.??..??...?##. 1,1,3
?#?#?#?#?#?#?#? 1,3,1,6
????.#...#... 4,1,1
????.######..#####. 1,6,5
?###???????? 3,2,1";
        assert_eq!(process(input)?, "21");
        Ok(())
    }
}

use anyhow::Result;
use itertools::{repeat_n, Itertools};
use nom::{
    bytes::{complete::tag, streaming::take_till1},
    character::complete,
    multi::separated_list1,
    sequence::separated_pair,
    IResult,
};

pub fn parse_arrangement(input: &str) -> IResult<&str, (Vec<char>, Vec<u32>)> {
    let (o, arrangement) = separated_pair(
        take_till1(|c| c == ' '),
        tag(" "),
        separated_list1(tag(","), complete::u32),
    )(input)?;

    Ok((o, (arrangement.0.chars().collect(), arrangement.1)))
}

// 7221 = just right
pub fn process(input: &str) -> Result<String> {
    let opts = input.lines().map(bruteforce_line).sum::<u32>();

    Ok(opts.to_string())
}

pub fn bruteforce_line(line: &str) -> u32 {
    let (_, arrange) = parse_arrangement(line).expect("valid parse");
    let wild_cards = arrange
        .0
        .clone()
        .into_iter()
        .filter(|c| c == &'?')
        .collect::<Vec<char>>();

    let options = repeat_n([".", "#"].into_iter(), wild_cards.len())
        .multi_cartesian_product()
        .map(|p| p.join(""))
        .collect::<Vec<String>>();

    options
        .into_iter()
        .filter(|o| check_option(o, arrange.0.clone(), arrange.1.clone()))
        .count() as u32
}

pub fn check_option(option: &str, line: Vec<char>, counts: Vec<u32>) -> bool {
    let mut option_c = option.chars();

    let filled = line
        .iter()
        .map(|c| match c {
            '?' => option_c.next().expect("should have char"),
            val => *val,
        })
        .collect::<String>();

    let filled_counts = filled
        .chars()
        .group_by(|c| c == &'#')
        .into_iter()
        .filter_map(|(h, v)| h.then_some(v.into_iter().count() as u32))
        .collect::<Vec<u32>>();

    counts[..] == filled_counts[..]
}

#[cfg(test)]
mod tests {
    use super::*;
    use pretty_assertions::assert_eq;
    use rstest::rstest;

    #[rstest]
    #[case("???.### 1,1,3", 1)]
    #[case(".??..??...?##. 1,1,3", 4)]
    #[case("?#?#?#?#?#?#?#? 1,3,1,6", 1)]
    #[case("????.#...#... 4,1,1", 1)]
    #[case("????.######..#####. 1,6,5", 4)]
    #[case("?###???????? 3,2,1", 10)]
    fn test_line(#[case] line: &str, #[case] expected: u32) -> Result<()> {
        assert_eq!(bruteforce_line(line), expected);
        Ok(())
    }

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

use std::collections::BTreeMap;

use anyhow::Result;
use nom::{IResult, sequence::{separated_pair, delimited}, character::complete::{alpha1, line_ending}, multi::separated_list1, bytes::complete::tag};

#[derive(Debug)]
pub struct Map<'a> {
    instructions: Vec<char>,
    nodes: BTreeMap<&'a str, (&'a str, &'a str)>,
}

fn parse_node(input: &str) -> IResult<&str, (&str, (&str, &str))> {
    separated_pair(alpha1, tag(" = "), delimited(tag("("), separated_pair(alpha1, tag(", "), alpha1), tag(")")))(input)
}

fn parse_map(input: &str) -> IResult<&str, Map> {
    let (o, map) = separated_pair(alpha1, tag("\n\n"), separated_list1(line_ending, parse_node))(input)?;

    Ok((o, Map {
        instructions: map.0.chars().collect(),
        nodes: map.1.iter().fold(BTreeMap::new(), |mut acc, node|  {
            acc.insert(node.0, node.1);
            acc
        }),
    }))
}

pub fn process(input: &str) -> Result<String> {
    let (_, map) = parse_map(input).expect("valid parse");
    dbg!(map);
    Ok("".to_string())
}

#[cfg(test)]
mod tests {
    use super::*;
    use pretty_assertions::assert_eq;

    #[test]
    fn test_process() -> Result<()> {
        let input = "RL

AAA = (BBB, CCC)
BBB = (DDD, EEE)
CCC = (ZZZ, GGG)
DDD = (DDD, DDD)
EEE = (EEE, EEE)
GGG = (GGG, GGG)
ZZZ = (ZZZ, ZZZ)";

        assert_eq!(process(input)?, "2");
        Ok(())
    }
}

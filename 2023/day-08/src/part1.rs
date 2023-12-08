use std::collections::BTreeMap;

use anyhow::Result;
use nom::{
    bytes::complete::tag,
    character::complete::{alpha1, line_ending},
    multi::separated_list1,
    sequence::{delimited, separated_pair},
    IResult,
};

#[derive(Debug)]
pub struct Map<'a> {
    instructions: Vec<char>,
    nodes: BTreeMap<&'a str, (&'a str, &'a str)>,
}

fn parse_node(input: &str) -> IResult<&str, (&str, (&str, &str))> {
    separated_pair(
        alpha1,
        tag(" = "),
        delimited(
            tag("("),
            separated_pair(alpha1, tag(", "), alpha1),
            tag(")"),
        ),
    )(input)
}

fn parse_map(input: &str) -> IResult<&str, Map> {
    let (o, map) = separated_pair(
        alpha1,
        tag("\n\n"),
        separated_list1(line_ending, parse_node),
    )(input)?;

    Ok((
        o,
        Map {
            instructions: map.0.chars().collect(),
            nodes: map.1.iter().fold(BTreeMap::new(), |mut acc, node| {
                acc.insert(node.0, node.1);
                acc
            }),
        },
    ))
}

// 13019 = just right
pub fn process(input: &str) -> Result<String> {
    let (_, map) = parse_map(input).expect("valid parse");

    let mut instructions = map.instructions.iter();
    let first_node = map
        .nodes
        .first_key_value()
        .expect("should have at least 1 node")
        .1;
    let first_instruction = instructions
        .next()
        .expect("should have at least 1 instruction");
    let mut choice = if first_instruction == &'L' {
        first_node.0
    } else {
        first_node.1
    };
    // Start at 1 to account for getting first choice.
    let mut steps = 1;
    let mut found_end = false;

    while !found_end {
        instructions
            .map(|instruction| {
                let next_option = map
                    .nodes
                    .get(choice)
                    .expect("should always have a corresponding node");

                if instruction == &'L' {
                    choice = next_option.0
                } else {
                    choice = next_option.1
                }

                steps += 1;

                if choice == "ZZZ" {
                    found_end = true;
                }
            })
            .collect::<()>();

        instructions = map.instructions.iter();
    }

    Ok(steps.to_string())
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

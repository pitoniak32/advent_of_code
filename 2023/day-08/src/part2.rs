use std::collections::{BTreeMap, HashMap};

use anyhow::Result;
use nom::{
    bytes::complete::tag,
    character::complete::{alpha1, alphanumeric1, line_ending},
    multi::separated_list1,
    sequence::{delimited, separated_pair},
    IResult,
};
use rayon::prelude::{IntoParallelRefIterator, ParallelIterator};

#[derive(Debug)]
pub struct Map<'a> {
    instructions: Vec<char>,
    nodes: BTreeMap<&'a str, (&'a str, &'a str)>,
}

fn parse_node(input: &str) -> IResult<&str, (&str, (&str, &str))> {
    separated_pair(
        alphanumeric1,
        tag(" = "),
        delimited(
            tag("("),
            separated_pair(alphanumeric1, tag(", "), alphanumeric1),
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

// 103038 = too low
// 103044 = too low
pub fn process(input: &str) -> Result<String> {
    let (_, map) = parse_map(input).expect("valid parse");

    let starting_nodes = map.nodes.iter().filter(|n| n.0.ends_with("A"));
    let starting_nodes_found: HashMap<&str, bool> = starting_nodes
        .clone()
        .map(|n| (n.0.clone(), false))
        .collect();

    let mut steps = 1;

    let mut instructions = map.instructions.iter();
    let mut starting_nodes_f = starting_nodes_found.clone();

    while !starting_nodes_f.clone().iter().all(|n| *n.1) {
        if let Some(instruction) = instructions.next() {
            for node in starting_nodes.clone() {
                let choice = if instruction == &'L' { node.1.0 } else { node.1.1 };

                if choice.ends_with('Z') {
                    println!("Found: {:?}", choice);
                    starting_nodes_f.insert(node.0, true);
                }

                steps += 1;
            }
        } else {
            instructions = map.instructions.iter().clone()
        }
    }

    Ok(steps.to_string())
}

#[cfg(test)]
mod tests {
    use super::*;
    use pretty_assertions::assert_eq;

    #[test]
    fn test_process() -> Result<()> {
        let input = "LR

11A = (11B, XXX)
11B = (XXX, 11Z)
11Z = (11B, XXX)
22A = (22B, XXX)
22B = (22C, 22C)
22C = (22Z, 22Z)
22Z = (22B, 22B)
XXX = (XXX, XXX)";

        assert_eq!(process(input)?, "6");
        Ok(())
    }
}

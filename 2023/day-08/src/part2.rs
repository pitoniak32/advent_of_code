use std::collections::{BTreeMap};

use anyhow::Result;
use nom::{
    bytes::complete::tag,
    character::complete::{alpha1, alphanumeric1, line_ending},
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

// 13524038372771 = just right
// 131298 = too low
// 103038 = too low
// 103044 = too low
pub fn process(input: &str) -> Result<String> {
    let (_, map) = parse_map(input).expect("valid parse");

    let starting_nodes: Vec<&str> = map
        .nodes
        .keys()
        .filter(|n| n.ends_with('A'))
        .cloned()
        .collect();

    let instructions = map.instructions;

    // Needed help to get this one.
    let result = starting_nodes
        .iter()
        .map(|node| {
            let mut starting_nodes_visited = vec![*node];
            let mut current_node = *node;
            instructions
                .iter()
                .cycle()
                .enumerate()
                .find_map(|(index, instruction)| {
                    let option = map
                        .nodes
                        .get(current_node)
                        .expect("should always have a node");

                    let next_node = match instruction {
                        'L' => option.0,
                        'R' => option.1,
                        _ => unreachable!("should never have anything other than LR"),
                    };

                    if next_node.ends_with('Z') {
                        Some(index + 1)
                    } else {
                        starting_nodes_visited.push(next_node);
                        current_node = next_node;
                        None
                    }
                })
                .expect("should find cycle")
        })
        .collect::<Vec<usize>>();

    Ok(lcm(&result).to_string())
}

pub fn lcm(nums: &[usize]) -> usize {
    if nums.len() == 1 {
        return nums[0];
    }
    let a = nums[0];
    let b = lcm(&nums[1..]);
    a * b / gcd_of_two_numbers(a, b)
}

fn gcd_of_two_numbers(a: usize, b: usize) -> usize {
    if b == 0 {
        return a;
    }
    gcd_of_two_numbers(b, a % b)
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

use std::collections::HashMap;

use anyhow::Result;
use nom::{
    branch::alt,
    bytes::complete::tag,
    character::complete::{self, alpha1},
    combinator::opt,
    sequence::tuple,
    IResult,
};

#[derive(Debug)]
pub enum Op {
    Dash,
    Equal,
}

#[derive(Debug)]
pub struct Step {
    pub label: String,
    pub op: Op,
    pub focal_len: Option<u32>,
    pub hash: u64,
}

#[derive(Debug)]
pub struct Lens {
    pub focal_len: u32,
    pub label: String,
}

impl From<&Step> for Lens {
    fn from(value: &Step) -> Self {
        Lens {
            focal_len: value.focal_len.expect("must have focal len"),
            label: value.label.clone(),
        }
    }
}

pub fn parse_step(input: &str) -> IResult<&str, Step> {
    let (o, step) = tuple((alpha1, alt((tag("-"), tag("="))), opt(complete::u32)))(input)?;

    Ok((
        o,
        Step {
            label: step.0.to_string(),
            op: if step.1 == "-" { Op::Dash } else { Op::Equal },
            focal_len: step.2,
            hash: hash(step.0),
        },
    ))
}

// 210906 = just right
pub fn process(input: &str) -> Result<String> {
    let steps = input.split(',').fold(Vec::new(), |mut acc, code| {
        acc.push(parse_step(code).expect("valid parse").1);
        acc
    });

    let boxes = steps
        .iter()
        .fold(HashMap::<u64, Vec<Lens>>::new(), |mut acc, step| {
            match step.op {
                Op::Dash => {
                    if let Some(contents) = acc.get_mut(&step.hash) {
                        if let Some(found_lens) =
                            contents.iter().position(|lens| lens.label == step.label)
                        {
                            contents.remove(found_lens);
                        }
                    }
                }
                Op::Equal => {
                    if let Some(contents) = acc.get_mut(&step.hash) {
                        if let Some(found_lens) =
                            contents.iter().position(|lens| lens.label == step.label)
                        {
                            *contents
                                .get_mut(found_lens)
                                .expect("position is in contents") = Lens::from(step);
                        } else {
                            contents.push(Lens::from(step))
                        }
                    } else {
                        acc.insert(step.hash, vec![Lens::from(step)]);
                    };
                }
            };
            acc
        })
        .iter()
        .fold(0, |mut acc, (key, curr_box)| {
            acc += curr_box
                .iter()
                .enumerate()
                .map(|(i, lens)| (key + 1) * (i + 1) as u64 * lens.focal_len as u64)
                .sum::<u64>();
            acc
        });

    dbg!(&boxes);

    Ok(boxes.to_string())
}

pub fn hash(input: &str) -> u64 {
    input.trim().chars().fold(0_u64, |mut acc, c| {
        let ascii_code = c as u8;
        acc += ascii_code as u64;
        acc *= 17;
        acc %= 256;
        acc
    })
}

#[cfg(test)]
mod tests {
    use super::*;
    use pretty_assertions::assert_eq;
    use rstest::rstest;

    #[rstest]
    #[case("HASH", 52)]
    #[case("rn=1", 30)]
    #[case("cm-", 253)]
    #[case("qp=3", 97)]
    #[case("cm=2", 47)]
    #[case("qp-", 14)]
    #[case("pc=4", 180)]
    #[case("ot=9", 9)]
    #[case("ab=5", 197)]
    #[case("pc-", 48)]
    #[case("pc=6", 214)]
    #[case("ot=7", 231)]
    fn test_hash(#[case] input: &str, #[case] expected: u64) -> Result<()> {
        assert_eq!(hash(input), expected);
        Ok(())
    }

    #[test]
    fn test_process() -> Result<()> {
        let input = "rn=1,cm-,qp=3,cm=2,qp-,pc=4,ot=9,ab=5,pc-,pc=6,ot=7";
        assert_eq!(process(input)?, "145");
        Ok(())
    }
}

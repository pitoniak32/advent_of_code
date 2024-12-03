use anyhow::Result;
use nom::bytes::complete::tag;
use nom::character::complete::anychar;
use nom::multi::many1;
use nom::multi::many_till;
use nom::sequence::delimited;
use nom::sequence::separated_pair;
use nom::IResult;
use nom::Parser;

pub fn process(input: &'static str) -> Result<String> {
    let (_, instructions) = parse_instructions(input)?;
    Ok(instructions
        .iter()
        .map(|ins| match ins {
            Instruction::Mul(a, b) => a * b,
            Instruction::Do => todo!(),
            Instruction::Dont => todo!(),
        })
        .sum::<i32>()
        .to_string())
}

#[derive(Debug)]
pub enum Instruction {
    Mul(i32, i32),
    Do,
    Dont,
}

pub fn parse_instruction(input: &str) -> IResult<&str, Instruction> {
    let (input, _) = tag("mul")(input)?;
    let (input, pair) = delimited(
        tag("("),
        separated_pair(
            nom::character::complete::i32,
            tag(","),
            nom::character::complete::i32,
        ),
        tag(")"),
    )(input)?;

    Ok((input, Instruction::Mul(pair.0, pair.1)))
}

pub fn parse_instructions(input: &str) -> IResult<&str, Vec<Instruction>> {
    many1(many_till(anychar, parse_instruction).map(|(_discard, ins)| ins))(input)
}

#[cfg(test)]
mod tests {
    use super::*;
    use pretty_assertions::assert_eq;

    #[test]
    fn test_process() -> Result<()> {
        let input = "xmul(2,4)%&mul[3,7]!@^do_not_mul(5,5)+mul(32,64]then(mul(11,8)mul(8,5))";
        assert_eq!(process(input)?, "161");
        Ok(())
    }
}

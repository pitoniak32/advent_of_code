use anyhow::Result;

pub fn process(input: &str) -> Result<String> {
    let result = input.split(',').fold(0_u64, |mut acc, code| {
        acc += hash(code);
        acc
    });

    Ok(result.to_string())
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
        assert_eq!(process(input)?, "1320");
        Ok(())
    }
}

use anyhow::Result;

pub fn process(
    _input: &str,
) -> Result<String> {
    todo!("{{crate_name}} - part 1");
}

#[cfg(test)]
mod tests {
    use pretty_assertions::assert_eq;
    use rstest::rstest;
    use super::*;

    #[test]
    fn test_process() -> Result<()> {
        todo!("haven't built test yet");
        let input = "";
        assert_eq!("", process(input)?);
        Ok(())
    }
}

use anyhow::Result;

pub fn process(
    input: &str,
) -> Result<String> {
    todo!("{input}");
}

#[cfg(test)]
mod tests {
    use pretty_assertions::assert_eq;
    use super::*;

    #[test]
    fn test_process() -> Result<()> {
        todo!("haven't built test yet");
        let input = "";
        assert_eq!("", process(input)?);
        Ok(())
    }
}

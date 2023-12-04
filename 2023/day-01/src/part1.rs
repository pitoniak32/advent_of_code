use anyhow::Result;

pub fn process(input: &str) -> Result<String> {
    let mut total_code = 0;
    for line in input.lines() {
        let code = get_code(line);
        total_code += code;
    }
    Ok(format!("final code: {total_code}"))
}

fn get_code(code: &str) -> u32 {
    let mut vals = vec![];
    for l_char in code.chars() {
        if l_char.is_ascii_digit() {
            vals.push(l_char);
        }
    }

    if vals.len() == 1 {
        vals.push(*vals.first().unwrap());
    }

    format!(
        "{}{}",
        vals.first().unwrap().clone(),
        vals.last().unwrap().clone(),
    )
    .parse::<u32>()
    .expect("to be u32")
}

#[cfg(test)]
mod tests {
    use super::*;
    use pretty_assertions::assert_eq;
    use rstest::rstest;

    #[rstest]
    #[case("a1b2c3d4e5f", 15)]
    fn test_get_code_part1(#[case] input: &str, #[case] expected: u32) {
        // Arrange / Act / Assert
        assert_eq!(get_code(input), expected);
    }
}

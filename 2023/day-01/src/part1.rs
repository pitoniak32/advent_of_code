use anyhow::Result;

pub fn process(
    input: &str,
) -> Result<String> {
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
        if l_char.is_digit(10) {
            vals.push(l_char);
        }
    }

    if vals.len() == 1 {
        vals.push(vals.get(0).unwrap().clone());
    }

    format!(
        "{}{}",
        vals.get(0).unwrap().clone(),
        vals.get(vals.len() - 1).unwrap().clone(),
    )
    .parse::<u32>()
    .expect("to be u32")
}

#[cfg(test)]
mod tests {
    use super::*;
    use pretty_assertions::assert_eq;

    #[test]
    fn test_get_code_part1() {
        // Arrange
        let expected = 15;
        // Act / Assert
        assert_eq!(get_code("a1b2c3d4e5f"), expected);
    }
}

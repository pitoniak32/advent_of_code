use anyhow::Result;

fn main() -> Result<()> {
    let input_file = include_str!("../../input1.txt");
    println!("{}", input_file);
    let mut total_code = 0;
    for line in input_file.split("\n") {
        if line.trim().len() < 1 {
            continue
        }
        let code = get_code(line);
        total_code += code;
    }
    println!("final code: {total_code}");
    Ok(())
}

fn get_code(code: &str) -> u32 {
    let mut vals = vec![];
    for l_char in code.chars() {
        if l_char.is_digit(10) {
            vals.push(l_char);
        }
    }

    println!("{vals:?}");

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
    fn test_example_works() {
        // Arrange
        let expected = 15;
        // Act / Assert
        assert_eq!(get_code("a1b2c3d4e5f"), expected);
    }
}

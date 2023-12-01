use anyhow::Result;

fn main() -> Result<()> {
    let input_file = include_str!("../../input2.txt");
    println!("{}", input_file);
    let mut total_code = 0;
    for line in input_file.split("\n") {
        if line.trim().len() < 1 {
            continue;
        }
        let code = get_code(line);
        total_code += code;
    }
    println!("final code: {total_code}");
    Ok(())
}

// five1two = 52
fn get_code(code: &str) -> u32 {
    0
}

#[cfg(test)]
mod tests {
    use super::*;
    use pretty_assertions::assert_eq;

    #[test]
    fn test_it_works_29() {
        assert_eq!(get_code("two1nine"), 29);
    }

    #[test]
    fn test_it_works_83() {
        assert_eq!(get_code("eightwothree"), 83);
    }
}

use anyhow::Result;

fn main() -> Result<()> {
    let input_file = include_str!("../../input2.txt");
    println!("{}", input_file);
    Ok(())
}

#[cfg(test)]
mod tests {
    use pretty_assertions::{assert_eq, assert_ne};
    use super::*;

    #[test]
    fn test_it_works() {
        assert_eq!(1, 1);
    }
}

use day_12::part1::process;

use anyhow::{Context, Result};

fn main() -> Result<()> {
    let input_file = include_str!("../../input1.txt");
    let result = process(input_file).context("process part 1")?;
    println!("{}", result);
    Ok(())
}

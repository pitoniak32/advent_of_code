use day_11::part2::process;

use anyhow::{Context, Result};

fn main() -> Result<()> {
    let input_file = include_str!("../../input2.txt");
    let result = process(input_file, 1_000_000).context("process part 2")?;
    println!("{}", result);
    Ok(())
}

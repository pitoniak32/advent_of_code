use day_01::part2::process;

use anyhow::{Context, Result};

fn main() -> Result<()> {
    let input_file = include_str!("../../../input1.txt");
    let result = process(input_file).context("process part 2")?;
    println!("{}", result);
    Ok(())
}

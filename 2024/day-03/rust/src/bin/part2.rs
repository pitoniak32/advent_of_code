use day_03::part2::process;

use anyhow::{Result, Context};

fn main() -> Result<()> {
    let input_file = include_str!("../../../input2.txt");
    let result = process(input_file).context("process part 2")?;
    println!("{}", result);
    Ok(())
}

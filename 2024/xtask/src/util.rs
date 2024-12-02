use std::fs::{self, OpenOptions};
use std::io::Write;
use std::path::PathBuf;

pub fn get_day_string(day: u8) -> String {
    format!("day-{day:02}")
}

pub fn get_part_string(part: u8) -> String {
    format!("part{part}")
}

pub fn setup_day_folder(dir: &PathBuf) -> anyhow::Result<()> {
    fs::create_dir_all(dir)?;

    if let Ok(mut input1_file) = OpenOptions::new()
        .create_new(true)
        .append(true)
        .open(dir.join("input1.txt"))
    {
        if let Err(e) = writeln!(input1_file, "input1 file!") {
            eprintln!("Couldn't write to file: {}", e);
        }
    }

    if let Ok(mut input2_file) = OpenOptions::new()
        .create_new(true)
        .append(true)
        .open(dir.join("input2.txt"))
    {
        if let Err(e) = writeln!(input2_file, "input2 file!") {
            eprintln!("Couldn't write to file: {}", e);
        }
    }

    Ok(())
}

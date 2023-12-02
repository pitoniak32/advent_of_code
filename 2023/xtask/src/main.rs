use anyhow::Result;
use std::{
    env,
    path::{Path, PathBuf},
    process::{Command, Stdio, exit}, fs::OpenOptions, io::Write,
};

use clap::{command, Args, Parser, Subcommand};

#[derive(Parser)]
#[command(author, version, about)]
struct XTaskCli {
    #[command(subcommand)]
    command: XTaskCommands,
}

#[derive(Subcommand)]
enum XTaskCommands {
    Generate(GenerateArgs),
    Run(DayPartOptsArgs),
    Bench(DayPartArgs),
    Build(DayArgs),
    Test(DayArgs),
}

#[derive(Args)]
struct GenerateArgs {
    /// day you want to generate. ex: 1 for day-01
    day_num: u8,
}

#[derive(Args)]
struct DayPartOptsArgs {
    day: u8,
    part: u8,
    opts: Vec<String>,
}

#[derive(Args)]
struct DayPartArgs {
    day: u8,
    part: u8,
}

#[derive(Args)]
struct DayArgs {
    day: u8,
    opts: Vec<String>,
}

fn main() -> Result<()> {
    let cli = XTaskCli::parse();
    let project_root = project_root();
    let cargo = env::var("CARGO").unwrap_or_else(|_| "cargo".to_string());

    match cli.command {
        XTaskCommands::Generate(args) => {
            Command::new(cargo)
                .current_dir(project_root)
                .args([
                    "generate",
                    "--path",
                    "./template",
                    "--name",
                    &format!("day-{:02}", &args.day_num),
                ])
                .status()?;
        }
        XTaskCommands::Run(mut args) => {
            Command::new(cargo)
                .current_dir(project_root)
                .args(cargo_day_part_opts("run", &mut args))
                .status()?;
        }
        XTaskCommands::Bench(args) => {
            let output = Command::new(cargo)
                .stdout(Stdio::piped())
                .current_dir(project_root)
                .args([
                    "bench",
                    "--bench",
                    &format!("day-{day:02}", day = args.day),
                    &format!("part{}", args.part),
                ])
                .output()?;

            if output.status.success() {
                let mut file = OpenOptions::new()
                    .create(true)
                    .append(true)
                    .open(format!("day-{:02}.bench.txt", &args.day))
                    .expect("Unable to open benchmark results file");
                file.write_all(&output.stdout).expect("Unable to write benchmark results to file");
            } else {
                eprintln!("benchmark failed, results will not be written: {}", String::from_utf8(output.stderr)?);
                exit(1);
            }
        }
        XTaskCommands::Build(mut args) => {
            Command::new(cargo)
                .current_dir(project_root)
                .args(cargo_day("build", &mut args))
                .status()?;
        }
        XTaskCommands::Test(mut args) => {
            Command::new(cargo)
                .current_dir(project_root)
                .args(cargo_day("test", &mut args))
                .status()?;
        }
    }

    Ok(())
}

fn cargo_day(cmd: &str, args: &mut DayArgs) -> Vec<String> {
    let day = args.day;
    let mut opts = &mut args.opts;
    let mut args = vec![
        cmd.to_string(),
        "--package".to_string(),
        format!("day-{day:02}"),
    ];
    args.append(&mut opts);
    args
}

fn cargo_day_part_opts(cmd: &str, args: &mut DayPartOptsArgs) -> Vec<String> {
    let day = args.day;
    let part = args.part;
    let mut opts = &mut args.opts;
    let mut args = vec![
        cmd.to_string(),
        "--package".to_string(),
        format!("day-{day:02}"),
        "--bin".to_string(),
        format!("part{part}"),
    ];
    args.append(&mut opts);
    args
}

fn project_root() -> PathBuf {
    Path::new(&env!("CARGO_MANIFEST_DIR"))
        .ancestors()
        .nth(1)
        .unwrap()
        .to_path_buf()
}

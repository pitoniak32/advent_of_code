use anyhow::Result;
use std::{
    env,
    path::{Path, PathBuf},
    process::Command,
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
    Run(DayPartArgs),
    Build(DayPartArgs),
    Test(DayPartArgs),
}

#[derive(Args)]
struct GenerateArgs {
    /// day you want to generate. ex: 1 for day-01
    day_num: u8,
}

#[derive(Args)]
struct DayPartArgs {
    day: u8,
    part: u8,
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
                .args(["generate", "--path", "./template", "--name", &format!("day-{:02}", &args.day_num)])
                .status()?;
        }
        XTaskCommands::Run(args) => {
            Command::new(cargo)
                .current_dir(project_root)
                .args(cargo_day_part("run", args))
                .status()?;
        }
        XTaskCommands::Build(args) => {
            Command::new(cargo)
                .current_dir(project_root)
                .args(cargo_day_part("build", args))
                .status()?;
        }
        XTaskCommands::Test(args) => {
            Command::new(cargo)
                .current_dir(project_root)
                .args(cargo_day_part("test", args))
                .status()?;
        }
    }

    Ok(())
}

fn cargo_day_part(cmd: &str, args: DayPartArgs) -> Vec<String> {
    let day = args.day;
    let part = args.part;
    let mut opts = args.opts;
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

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
    name: String,
}

#[derive(Args)]
struct DayPartArgs {
    #[arg(short, long)]
    day: u8,
    #[arg(short, long)]
    part: u8,
}

fn main() -> Result<()> {
    let cli = XTaskCli::parse();
    let project_root = project_root();
    let cargo = env::var("CARGO").unwrap_or_else(|_| "cargo".to_string());

    match cli.command {
        XTaskCommands::Generate(args) => {
            Command::new(cargo)
                .current_dir(project_root)
                .args(["generate", "--path", "./template", "--name", &args.name])
                .status()?;
        }
        XTaskCommands::Run(args) => {
            Command::new(cargo)
                .current_dir(project_root)
                .args(cargo_day_part("run", args.day, args.part))
                .status()?;
        }
        XTaskCommands::Build(args) => {
            Command::new(cargo)
                .current_dir(project_root)
                .args(cargo_day_part("build", args.day, args.part))
                .status()?;
        }
        XTaskCommands::Test(args) => {
            Command::new(cargo)
                .current_dir(project_root)
                .args(cargo_day_part("test", args.day, args.part))
                .status()?;
        }
    }

    Ok(())
}

fn cargo_day_part(cmd: &str, day: u8, part: u8) -> Vec<String> {
    vec!(cmd.to_string(), "--package".to_string(), format!("day-{day:02}"), "--bin".to_string(), format!("part{part}"))
}

fn project_root() -> PathBuf {
    Path::new(&env!("CARGO_MANIFEST_DIR"))
        .ancestors()
        .nth(1)
        .unwrap()
        .to_path_buf()
}

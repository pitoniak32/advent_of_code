use anyhow::Result;
use std::{
    env,
    path::{Path, PathBuf},
};

use clap::{command, Args, Parser, Subcommand, ValueEnum};

use self::{go::handle_go, rust::handle_rust};

mod go;
mod rust;
mod util;

#[derive(Parser)]
#[command(author, version, about)]
struct XTaskCli {
    #[command(subcommand)]
    command: XTaskCommands,
    #[arg(short, long)]
    lang: Lang,
}

#[derive(Subcommand)]
enum XTaskCommands {
    Generate(GenerateArgs),
    Run(DayPartOptsArgs),
    Bench(DayPartOptsArgs),
    Build(DayPartOptsArgs),
    Test(DayPartOptsArgs),
}

#[derive(Clone, ValueEnum)]
enum Lang {
    Go,
    Rust,
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

fn main() -> Result<()> {
    let cli = XTaskCli::parse();
    let project_root = project_root();

    match cli.lang {
        Lang::Go => handle_go(cli, project_root),
        Lang::Rust => handle_rust(cli, project_root),
    }
}

fn project_root() -> PathBuf {
    Path::new(&env!("CARGO_MANIFEST_DIR"))
        .ancestors()
        .nth(1)
        .unwrap()
        .to_path_buf()
}

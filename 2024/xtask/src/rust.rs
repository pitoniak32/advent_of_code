use std::env;
use std::fs::OpenOptions;
use std::io::Write;
use std::path::PathBuf;
use std::process::{Command, Stdio};

use crate::util::{get_day_string, get_part_string, setup_day_folder};
use crate::{DayPartOptsArgs, XTaskCli, XTaskCommands};

pub fn handle_rust(cli: XTaskCli, project_root: PathBuf) -> anyhow::Result<()> {
    let cargo = env::var("CARGO").unwrap_or_else(|_| "cargo".to_string());
    match cli.command {
        XTaskCommands::Generate(args) => {
            let day_string = get_day_string(args.day_num);
            let dir = project_root.join(&day_string);

            setup_day_folder(&dir)?;

            Command::new(cargo)
                .current_dir(dir)
                .args([
                    "generate",
                    "--path",
                    "../template-rust",
                    "--name",
                    "rust",
                    "--define",
                    &format!("day_dash={}", day_string),
                    "--define",
                    &format!("day_under={}", day_string.replace("-", "_")),
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
                    &get_day_string(args.day),
                    &get_part_string(args.part),
                ])
                .output()?;

            if output.status.success() {
                let mut file = OpenOptions::new()
                    .create(true)
                    .append(true)
                    .open(format!("{}.bench.txt", get_day_string(args.day)))
                    .expect("Unable to open benchmark results file");
                file.write_all(&output.stdout)
                    .expect("Unable to write benchmark results to file");
            } else {
                eprintln!(
                    "benchmark failed, results will not be written: {}",
                    String::from_utf8(output.stderr)?
                );
                std::process::exit(1);
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

fn cargo_day(cmd: &str, args: &mut DayPartOptsArgs) -> Vec<String> {
    let day = args.day;
    let opts = &mut args.opts;
    let day_string = get_day_string(day);
    let mut args = vec![cmd.to_string(), "--package".to_string(), day_string];
    args.append(opts);
    args
}

fn cargo_day_part_opts(cmd: &str, args: &mut DayPartOptsArgs) -> Vec<String> {
    let day = args.day;
    let part = args.part;
    let opts = &mut args.opts;
    let mut args = vec![
        cmd.to_string(),
        "--package".to_string(),
        get_day_string(day),
        "--bin".to_string(),
        get_part_string(part),
    ];
    args.append(opts);
    args
}

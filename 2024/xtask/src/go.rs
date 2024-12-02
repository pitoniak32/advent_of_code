use std::path::PathBuf;
use std::process::Command;

use crate::util::{get_day_string, get_part_string, setup_day_folder};
use crate::{XTaskCli, XTaskCommands};

pub fn handle_go(cli: XTaskCli, project_root: PathBuf) -> anyhow::Result<()> {
    let cargo = std::env::var("CARGO").unwrap_or_else(|_| "cargo".to_string());
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
                    "../template-go",
                    "--name",
                    "go",
                    "--define",
                    &format!("day={}", day_string),
                ])
                .status()?;
        }
        XTaskCommands::Run(args) => {
            let day_string = get_day_string(args.day);
            let part_string = get_part_string(args.part);
            let path = PathBuf::new()
                .join(&day_string)
                .join("go")
                .join(&part_string)
                .join(format!("{}.go", part_string));
            Command::new("go")
                .args(vec!["run", path.to_string_lossy().as_ref()])
                .current_dir(project_root)
                .status()?;
        }
        XTaskCommands::Test(args) => {
            let day_string = get_day_string(args.day);
            let part_string = get_part_string(args.part);
            let path = project_root.join(&day_string).join("go").join(&part_string);
            Command::new("go")
                .args(vec!["test", "-v", path.to_string_lossy().as_ref()])
                .current_dir(project_root)
                .status()?;
        }
        _ => unimplemented!("this command is not usable for go"),
    }
    Ok(())
}

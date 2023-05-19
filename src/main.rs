use anyhow::{anyhow, Result};
use clap::{command, Parser};
use std::{env, fs, os::unix::fs::symlink};

#[derive(Parser, Debug)]
#[command(author, version, about, long_about = None)]
struct Args {
    #[arg(short = 'e', default_value_t = String::from("env"), help = "The path from cwd to the root of the env dir")]
    env_files_path: String,

    #[arg(
        short = 'a',
        help = "The path from root of git worktree to the root of the apps"
    )]
    apps_root_path: String,

    #[arg(
        short = 'w',
        help = "The path from cwd to the root of the git worktree"
    )]
    worktree_root_path: String,
}
fn main() -> Result<()> {
    let args = Args::parse();
    let cwd = env::current_dir()?;

    let env_dir_path = cwd.join(args.env_files_path);
    let worktree_dir_path = cwd.join(args.worktree_root_path);
    let env_dir = fs::read_dir(&env_dir_path)?;

    for env_file in env_dir {
        let env_file = env_file?;
        let env_file_name = String::from(
            env_file
                .file_name()
                .to_str()
                .ok_or(anyhow!("Could not parse env file name"))?,
        );
        let app_name = env_file_name
            .split('.')
            .next()
            .ok_or(anyhow!("No '.' in {:?}", env_file_name))?;

        let worktree_dir = fs::read_dir(&worktree_dir_path)?;
        for worktree in worktree_dir {
            let worktree_name = worktree?.file_name();
            let mut apps_dir = fs::read_dir(
                &worktree_dir_path
                    .join(worktree_name)
                    .join(&args.apps_root_path),
            )?;

            let app_dir = apps_dir
                .find(|dir| {
                    if let Ok(dir) = dir {
                        return dir.file_name() == app_name;
                    }

                    false
                })
                .ok_or(anyhow!("App with {app_name} should have been found"))??;

            // We found a matching app
            let link_path = app_dir.path().join(".env");

            // Do not need to deal with Error here, as if file doesn't exist we create it outselves
            fs::remove_file(&link_path);

            let original_path = env_dir_path.clone().join(&env_file_name);
            println!(
                "Attempting to symlink {:?} to {:?}",
                original_path, link_path
            );
            symlink(&original_path, &link_path)?;
            println!("Symlinked {:?} to {:?}", original_path, link_path);
        }
    }
    Ok(())
}

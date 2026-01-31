use anyhow::{bail, Context, Result};
use clap::{Parser, Subcommand};
use std::fs;
use std::path::PathBuf;

#[derive(Parser, Debug)]
#[command(
    name = "quies",
    version,
    about = "Return your Mac to a quiet, known state via profiles."
)]
struct Cli {
    #[command(subcommand)]
    command: Command,
}

#[derive(Subcommand, Debug)]
enum Command {
    /// Manage profiles (save/apply/show/list/delete)
    Profile {
        #[command(subcommand)]
        command: ProfileCommand,
    },
}

#[derive(Subcommand, Debug)]
enum ProfileCommand {
    /// List saved profiles
    List,

    /// Show a profile JSON
    Show { name: String },

    /// Save current state as a profile
    Save { name: String },

    /// Apply a profile
    Apply {
        name: String,
        /// Only show what would change
        #[arg(long)]
        dry_run: bool,
    },

    /// Delete a profile
    Delete { name: String },
}

fn main() -> Result<()> {
    let cli = Cli::parse();

    match cli.command {
        Command::Profile { command } => match command {
            ProfileCommand::List => {
                let dir = profiles_dir()?;
                println!("(todo) profile list: {}", dir.display());
            }
            ProfileCommand::Show { name } => {
                let path = profile_path(&name)?;
                println!("(todo) profile show: {} ({})", name, path.display());
            }
            ProfileCommand::Save { name } => println!("(todo) profile save: {name}"),
            ProfileCommand::Apply { name, dry_run } => {
                if dry_run {
                    println!("(todo) profile apply (dry-run): {name}");
                } else {
                    println!("(todo) profile apply: {name}");
                }
            }
            ProfileCommand::Delete { name } => println!("(todo) profile delete: {name}"),
        },
    }

    Ok(())
}

fn profiles_dir() -> Result<PathBuf> {
    // macOS: ~/Library/Application Support/quies/profiles
    let base = dirs::data_dir().context("failed to resolve data_dir")?;
    Ok(base.join("quies").join("profiles"))
}

fn profile_path(name: &str) -> Result<PathBuf> {
    validate_profile_name(name)?;
    Ok(profiles_dir()?.join(format!("{name}.json")))
}

fn validate_profile_name(name: &str) -> Result<()> {
    if name.is_empty() {
        bail!("profile name must not be empty");
    }
    if name.len() > 64 {
        bail!("profile name is too long (max 64)");
    }
    if !name
        .chars()
        .all(|c| c.is_ascii_alphanumeric() || c == '-' || c == '_')
    {
        bail!("profile name may contain only [A-Za-z0-9-_]");
    }
    Ok(())
}

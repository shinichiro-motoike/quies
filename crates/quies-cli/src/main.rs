use anyhow::{Context, Result};
use std::path::PathBuf;
use clap::{Parser, Subcommand};

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
            ProfileCommand::List => println!("(todo) profile list"),
            ProfileCommand::Show { name } => println!("(todo) profile show: {name}"),
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
    Ok(profiles_dir()?.join(format!("{name}.json")))
}


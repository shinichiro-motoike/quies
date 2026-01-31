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
            ProfileCommand::List => command_profile_list(),
            ProfileCommand::Show { name } => command_profile_show(&name),
            ProfileCommand::Save { name } => command_profile_save(&name),
            ProfileCommand::Apply { name, dry_run } => command_profile_apply(&name, dry_run),
            ProfileCommand::Delete { name } => command_profile_delete(&name),
        },
    }
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

fn command_profile_list() -> Result<()> {
    for name in quies_core::profile::list()? {
        println!("{name}");
    }
    Ok(())
}

fn command_profile_show(name: &str) -> Result<()> {
    let s = quies_core::profile::show_pretty_json(name)?;
    print!("{s}");
    Ok(())
}

fn command_profile_save(name: &str) -> Result<()> {
    quies_core::profile::save_placeholder(name)?;
    Ok(())
}

fn command_profile_apply(name: &str, dry_run: bool) -> Result<()> {
    if dry_run {
        println!("(todo) profile apply (dry-run): {name}");
    } else {
        println!("(todo) profile apply: {name}");
    }
    Ok(())
}

fn command_profile_delete(name: &str) -> Result<()> {
    let path = profile_path(name)?;

    if !path.exists() {
        bail!("profile not found: {name}");
    }

    fs::remove_file(&path).with_context(|| format!("failed to delete: {}", path.display()))?;

    Ok(())
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

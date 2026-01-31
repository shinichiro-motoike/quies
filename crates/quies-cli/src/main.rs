use anyhow::Result;
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
    Save {
        name: String,
        /// Overwrite if the profile already exists
        #[arg(long)]
        force: bool,
    },

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
            ProfileCommand::Save { name, force } => command_profile_save(&name, force),
            ProfileCommand::Apply { name, dry_run } => command_profile_apply(&name, dry_run),
            ProfileCommand::Delete { name } => command_profile_delete(&name),
        },
    }
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

fn command_profile_save(name: &str, force: bool) -> Result<()> {
    if force {
        quies_core::profile::save_current_state_force(name)?;
    } else {
        quies_core::profile::save_current_state(name)?;
    }
    Ok(())
}

fn command_profile_apply(name: &str, dry_run: bool) -> Result<()> {
    if dry_run {
        let s = quies_core::profile::dry_run_apply(name)?;
        print!("{s}");
        Ok(())
    } else {
        quies_core::profile::apply(name)?;
        Ok(())
    }
}

fn command_profile_delete(name: &str) -> Result<()> {
    quies_core::profile::delete(name)?;
    Ok(())
}

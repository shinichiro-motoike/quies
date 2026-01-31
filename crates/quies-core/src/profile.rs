use anyhow::{bail, Context, Result};
use serde::{Deserialize, Serialize};
use std::fs;
use std::path::PathBuf;

const PLACEHOLDER_NOTE: &str = "placeholder profile (no audio state yet)";

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Profile {
    pub version: u32,
    pub name: String,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub note: Option<String>,
}

pub fn profiles_dir() -> Result<PathBuf> {
    // macOS: ~/Library/Application Support/quies/profiles
    let base = dirs::data_dir().context("failed to resolve data_dir")?;
    Ok(base.join("quies").join("profiles"))
}

pub fn validate_profile_name(name: &str) -> Result<()> {
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

pub fn profile_path(name: &str) -> Result<PathBuf> {
    validate_profile_name(name)?;
    Ok(profiles_dir()?.join(format!("{name}.json")))
}

/// v1: create a placeholder profile JSON (no audio state yet).
/// - creates directory if missing
/// - fails if profile already exists
pub fn save_placeholder(name: &str) -> Result<Profile> {
    let path = profile_path(name)?;

    if path.exists() {
        bail!("profile already exists: {name}");
    }

    let dir = profiles_dir()?;
    fs::create_dir_all(&dir).with_context(|| format!("failed to create dir: {}", dir.display()))?;

    let profile = Profile {
        version: 1,
        name: name.to_string(),
        note: Some(PLACEHOLDER_NOTE.to_string()),
    };

    let s = serde_json::to_string_pretty(&profile)?;
    fs::write(&path, s).with_context(|| format!("failed to write profile: {}", path.display()))?;

    Ok(profile)
}

pub fn load(name: &str) -> Result<Profile> {
    let path = profile_path(name)?;
    let s = fs::read_to_string(&path)
        .with_context(|| format!("profile not found: {}", path.display()))?;
    let profile: Profile = serde_json::from_str(&s)
        .with_context(|| format!("failed to parse profile json: {}", path.display()))?;
    Ok(profile)
}

/// Convenience for CLI: pretty JSON output of the parsed Profile.
/// (We parse first so the output is stable and schema-validated.)
pub fn show_pretty_json(name: &str) -> Result<String> {
    let profile = load(name)?;
    Ok(serde_json::to_string_pretty(&profile)?)
}

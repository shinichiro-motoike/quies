use crate::coreaudio;
use anyhow::{bail, Context, Result};
use serde::{Deserialize, Serialize};
use std::fs;
use std::path::PathBuf;

const PLACEHOLDER_NOTE: &str = "placeholder profile (no audio state yet)";

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Profile {
    pub version: u32,
    pub name: String,

    #[serde(default)]
    pub state: AudioState,

    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub note: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct AudioState {
    /// default output device identifier (future: CoreAudio UID)
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub default_output: Option<String>,

    /// default input device identifier (future: CoreAudio UID)
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub default_input: Option<String>,
}

#[derive(Debug, Clone)]
pub struct ApplyPlan {
    pub profile_name: String,
    pub current: AudioState,
    pub target: AudioState,
    pub operations: Vec<String>,
    pub notes: Vec<String>,
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
        state: AudioState::default(), // unknown (None)
        note: Some(PLACEHOLDER_NOTE.to_string()),
    };

    let s = serde_json::to_string_pretty(&profile)?;
    fs::write(&path, s).with_context(|| format!("failed to write profile: {}", path.display()))?;

    Ok(profile)
}

/// v2: save current audio state (CoreAudio).
/// - creates directory if missing
/// - fails if profile already exists
pub fn save_current_state(name: &str) -> Result<Profile> {
    let path = profile_path(name)?;

    if path.exists() {
        bail!("profile already exists: {name}");
    }

    let dir = profiles_dir()?;
    fs::create_dir_all(&dir).with_context(|| format!("failed to create dir: {}", dir.display()))?;

    let (state, notes) = current_state();
    let note = if notes.is_empty() {
        None
    } else {
        Some(notes.join("; "))
    };

    let profile = Profile {
        version: 1,
        name: name.to_string(),
        state,
        note,
    };

    let s = serde_json::to_string_pretty(&profile)?;
    fs::write(&path, s).with_context(|| format!("failed to write profile: {}", path.display()))?;

    Ok(profile)
}

pub fn save_current_state_force(name: &str) -> Result<Profile> {
    let path = profile_path(name)?;

    let dir = profiles_dir()?;
    fs::create_dir_all(&dir).with_context(|| format!("failed to create dir: {}", dir.display()))?;

    let (state, notes) = current_state();
    let note = if notes.is_empty() {
        None
    } else {
        Some(notes.join("; "))
    };

    let profile = Profile {
        version: 1,
        name: name.to_string(),
        state,
        note,
    };

    let s = serde_json::to_string_pretty(&profile)?;
    fs::write(&path, s).with_context(|| format!("failed to write profile: {}", path.display()))?;

    Ok(profile)
}

pub fn delete(name: &str) -> Result<()> {
    let path = profile_path(name)?;

    if !path.exists() {
        bail!("profile not found: {name}");
    }

    std::fs::remove_file(&path).with_context(|| format!("failed to delete: {}", path.display()))?;

    Ok(())
}

pub fn list() -> Result<Vec<String>> {
    let dir = profiles_dir()?;
    if !dir.exists() {
        return Ok(vec![]);
    }

    let mut names: Vec<String> = Vec::new();
    for entry in
        std::fs::read_dir(&dir).with_context(|| format!("failed to read dir: {}", dir.display()))?
    {
        let entry = entry?;
        let path = entry.path();

        if path.extension().and_then(|s| s.to_str()) != Some("json") {
            continue;
        }

        if let Some(stem) = path.file_stem().and_then(|s| s.to_str()) {
            if validate_profile_name(stem).is_ok() {
                names.push(stem.to_string());
            }
        }
    }

    names.sort();
    Ok(names)
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

fn current_state() -> (AudioState, Vec<String>) {
    match coreaudio::current_audio_state() {
        Ok((default_output, default_input)) => (
            AudioState {
                default_output,
                default_input,
            },
            vec![],
        ),
        Err(e) => (
            AudioState::default(),
            vec![format!(
                "failed to read current audio state via CoreAudio: {e}"
            )],
        ),
    }
}

fn diff_audio_state(current: &AudioState, target: &AudioState) -> Vec<String> {
    fn fmt(v: &Option<String>) -> &str {
        v.as_deref().unwrap_or("(unknown)")
    }

    let mut ops = Vec::new();

    if current.default_output != target.default_output {
        ops.push(format!(
            "default_output: {} -> {}",
            fmt(&current.default_output),
            fmt(&target.default_output),
        ));
    }

    if current.default_input != target.default_input {
        ops.push(format!(
            "default_input: {} -> {}",
            fmt(&current.default_input),
            fmt(&target.default_input),
        ));
    }

    ops
}

pub fn apply_plan(name: &str) -> Result<ApplyPlan> {
    let profile = load(name)?;
    let (current, notes) = current_state();
    let target = profile.state.clone();

    let operations = diff_audio_state(&current, &target);

    Ok(ApplyPlan {
        profile_name: profile.name,
        current,
        target,
        operations,
        notes,
    })
}

pub fn apply(name: &str) -> Result<()> {
    let plan = apply_plan(name)?;

    // default output
    if let Some(target_uid) = plan.target.default_output.as_deref() {
        if plan.current.default_output.as_deref() != Some(target_uid) {
            coreaudio::set_default_output_uid(target_uid)
                .with_context(|| format!("failed to apply default_output to uid={target_uid}"))?;
        }
    }

    // default input
    if let Some(target_uid) = plan.target.default_input.as_deref() {
        if plan.current.default_input.as_deref() != Some(target_uid) {
            coreaudio::set_default_input_uid(target_uid)
                .with_context(|| format!("failed to apply default_input to uid={target_uid}"))?;
        }
    }

    Ok(())
}

pub fn dry_run_apply(name: &str) -> Result<String> {
    let plan = apply_plan(name)?;
    Ok(render_plan(&plan))
}

pub fn render_plan(plan: &ApplyPlan) -> String {
    fn fmt_opt(v: &Option<String>) -> &str {
        v.as_deref().unwrap_or("(unknown)")
    }

    let mut out = String::new();
    out.push_str(&format!("profile: {}\n", plan.profile_name));
    out.push_str("mode: dry-run\n");

    // schema-first: current/target を必ず表示（unknown でも）
    out.push_str("state:\n");
    out.push_str(&format!(
        "  default_output: {} -> {}\n",
        fmt_opt(&plan.current.default_output),
        fmt_opt(&plan.target.default_output)
    ));
    out.push_str(&format!(
        "  default_input:  {} -> {}\n",
        fmt_opt(&plan.current.default_input),
        fmt_opt(&plan.target.default_input)
    ));

    if plan.operations.is_empty() {
        out.push_str("changes: (none or unknown)\n");
    } else {
        out.push_str("changes:\n");
        for op in &plan.operations {
            out.push_str(&format!("  - {op}\n"));
        }
    }

    if !plan.notes.is_empty() {
        out.push_str("notes:\n");
        for n in &plan.notes {
            out.push_str(&format!("  - {n}\n"));
        }
    }

    out
}

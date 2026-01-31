# quies

**A CLI tool to return your Mac to a quiet, known state**

quies is a macOS-only CLI tool.  
Instead of simply switching audio devices, it focuses on  
**saving and reliably restoring the current audio state as profiles**.

---

## Concept

It is designed for situations such as:

- After finishing an online meeting
- After plugging or unplugging external microphones or headsets
- When Bluetooth / USB device order changes
- When macOS audio settings end up in an unexpected state

In these moments, quies helps you  
**smoothly return your Mac to a quiet, predictable state**.

Audio configurations are treated as **profiles**, representing work modes like:

- `meeting`
- `desk`
- `focus`

---

## Features

- macOS only
- Lightweight CLI written in Rust
- Direct CoreAudio integration (no external tools required)
- Profile-centric design
- No background daemon
- No GUI (CLI only)
- Safe diff preview with `--dry-run`

---

## Installation

> ⚠️ Currently under active development

For now, clone and build locally:

```bash
git clone https://github.com/yourname/quies.git
cd quies
cargo build --release
```

Planned installation methods:

- `cargo install`
- Homebrew formula

---

## Usage

### List profiles
```bash
quies profile list
```

### Show a profile
```bash
quies profile show <name>
```

### Save the current audio state
```bash
quies profile save <name>
```

Overwrite an existing profile:
```bash
quies profile save <name> --force
```

### Apply a profile (dry-run)
```bash
quies profile apply <name> --dry-run
```

### Apply a profile (execute)
```bash
quies profile apply <name>
```

### Delete a profile
```bash
quies profile delete <name>
```

---

## About `--dry-run`

With `--dry-run`, quies does not apply any changes.  
It only shows the difference between the current audio state and the target profile.

This allows you to safely verify what would change before applying it.

---

## Profiles

Profiles are stored as JSON files.

### Location
```
~/Library/Application Support/quies/profiles/
```

### Stored information
- Profile name
- Version (for future migrations)
- Default output device UID
- Default input device UID

Internally, quies uses **stable device UIDs**, so profiles remain valid even if device display names change.

---

## Non-goals

The following are **out of scope for v1**:

- Real-time monitoring
- Background services or daemons
- Menu bar applications
- GUI interfaces
- Automatic switching

---

## Project status

This project is **actively developed**.

Core functionality (save / apply / dry-run) is implemented,  
but internal APIs and output formats may still evolve.

---

## License

MIT OR Apache-2.0

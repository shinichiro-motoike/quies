# quies

**A CLI tool to return your Mac to a quiet, known state**

quies is a macOS-only CLI tool.
Instead of simply switching audio devices, it focuses on **saving and restoring the current audio device state as profiles**.

---

## Concept

quies is not just an "audio switcher".

It is designed for moments like:

- After finishing an online meeting
- After plugging or unplugging external audio devices
- When your audio configuration somehow gets messed up

In those situations, quies helps you **return your Mac to a quiet, predictable state**.

Profiles represent "work modes", such as:

- `meeting`
- `desk`
- `focus`

---

## Features

- macOS only
- Lightweight CLI written in Rust
- Direct CoreAudio integration
- Profile-centric design
- No background daemon
- No GUI (CLI only)

---

## Installation

> ⚠️ Currently under active development

Planned installation methods:

- Homebrew
- `cargo install`

---

## Usage (planned)

```bash
quies profile list
quies profile show <name>
quies profile save <name>
quies profile apply <name> [--dry-run]
quies profile delete <name>
```

---

## Profiles

Profiles are stored as JSON files.

- Planned location:
  `~/Library/Application Support/quies/profiles/`

- Each profile includes:
  - Audio device information
  - A version field for future compatibility

---

## Non-goals

The following are **out of scope for v1**:

- Real-time monitoring
- Background services or daemons
- Menu bar applications
- GUI interfaces

---

## Project status

This project is in an **early development stage**.
APIs and internal structures may change.

---

## License

MIT OR Apache-2.0

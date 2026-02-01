# Using quies with Hammerspoon
## Auto-apply a profile when AirPods are selected

`quies` can be integrated with **Hammerspoon** to automatically apply a profile
when your Mac’s audio output device changes.

A common use case:

> When AirPods become the default output device, automatically apply a known-good audio profile.

This helps keep your Mac in a *quiet, predictable state* without manual intervention.

---

## Why Hammerspoon?

`quies` is intentionally **event-agnostic**.

- It applies profiles when explicitly invoked
- It does not watch system events or perform automation on its own

Hammerspoon complements this design by:

- Observing macOS system events (audio device changes)
- Deciding **when** `quies` should run
- Keeping `quies` focused on **state management**, not automation logic

This separation keeps `quies` simple, predictable, and scriptable.

---

## Prerequisites

- macOS
- **Hammerspoon**  
  https://www.hammerspoon.org/
- `quies` installed and available in `PATH`

```bash
quies --version
```

- A profile to apply (example: `default`)

```bash
quies profile save default
```

---

## Example: Auto-apply a profile when AirPods are selected

Add the following configuration to your Hammerspoon setup:

`~/.hammerspoon/init.lua`

```lua
-- =========================
-- quies auto apply on AirPods
-- =========================

local AIRPODS_KEYWORDS = { "AirPods" }
local DEFAULT_PROFILE = "default"
local COOLDOWN_SECONDS = 3
local lastRunAt = 0

local function notify(title, text)
  hs.notify.new({ title = title, informativeText = text }):send()
end

local function resolveQuiesPath()
  local out, ok, _, rc = hs.execute("command -v quies", true)
  if ok and rc == 0 and out then
    local path = out:gsub("%s+$", "")
    if path ~= "" then return path end
  end
  return nil
end

local function isAirPods(deviceName)
  if not deviceName then return false end
  for _, keyword in ipairs(AIRPODS_KEYWORDS) do
    if string.find(deviceName, keyword, 1, true) then
      return true
    end
  end
  return false
end

local function applyProfile(quiesPath)
  hs.task.new(quiesPath, function(exitCode)
    if exitCode ~= 0 then
      notify("quies", "profile apply failed")
    end
  end, { "profile", "apply", DEFAULT_PROFILE }):start()
end

local function audioDeviceChanged(event)
  if event ~= "dOut" then return end

  local device = hs.audiodevice.defaultOutputDevice()
  if not device then return end

  local name = device:name()
  if not isAirPods(name) then return end

  local now = hs.timer.secondsSinceEpoch()
  if (now - lastRunAt) < COOLDOWN_SECONDS then return end
  lastRunAt = now

  local quiesPath = resolveQuiesPath()
  if not quiesPath then
    notify("Hammerspoon", "quies not found in PATH")
    return
  end

  applyProfile(quiesPath)
end

hs.audiodevice.watcher.setCallback(audioDeviceChanged)
hs.audiodevice.watcher.start()

notify("Hammerspoon", "quies AirPods watcher started")
```

---

## Troubleshooting

### `quies not found in PATH`

Hammerspoon runs as a GUI application and may not inherit your shell’s `PATH`.

Install `quies` into `/usr/local/bin` or `/opt/homebrew/bin`,
or explicitly define `PATH` inside `init.lua`.

---

## Design philosophy

- **quies** — declarative, explicit state restoration
- **Hammerspoon** — imperative, event-driven automation

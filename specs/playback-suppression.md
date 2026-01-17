# Playback Suppression Specification

**Status:** Implemented
**Version:** 1.0
**Last Updated:** 2025-01-17

---

## 1. Overview

### Purpose

The playback suppression module prevents sounds from playing when specific applications are running. This avoids awkward interruptions during video calls, meetings, or other focused activities.

### Goals

- **Automatic suppression** - Detect running apps and skip playback without user intervention
- **Configurable blocklist** - Users can customize which apps trigger suppression
- **Sensible defaults** - Ship with common meeting apps pre-configured
- **Override capability** - Provide a `--force` flag to bypass suppression

### Non-Goals

- **Audio ducking** - No lowering volume instead of muting (too complex)
- **Scheduled suppression** - No time-based rules (use OS focus modes instead)
- **Per-category rules** - No different blocklists per sound category

---

## 2. Architecture

### Component Structure

```
src/
├── config/
│   └── mod.rs           # Extended with blocklist config
├── suppression/
│   ├── mod.rs           # Module exports
│   └── detector.rs      # Process detection logic
```

### Dependency

This module uses `sysinfo` for cross-platform process detection:

```toml
[dependencies]
sysinfo = "0.30"
```

### Data Flow

```
CLI: cue play <category>
         │
         ▼
┌─────────────────┐
│  Load Config    │──▶ Get blocklist from config
└─────────────────┘
         │
         ▼
┌─────────────────┐
│ Check --force   │──▶ If set, skip detection
└─────────────────┘
         │
         ▼
┌─────────────────┐
│ Detect Processes│──▶ Scan running processes
└─────────────────┘
         │
         ▼
┌─────────────────┐
│ Match Blocklist │──▶ Case-insensitive substring match
└─────────────────┘
         │
    ┌────┴────┐
    ▼         ▼
 Match?     No Match
    │         │
    ▼         ▼
 Print     Continue
 Skip      to playback
 Message
```

---

## 3. Core Types

### 3.1 Config Extension

Extend the existing `Config` struct to include blocklist settings.

```rust
#[derive(Debug, Deserialize)]
pub struct Config {
    pub sounds_path: PathBuf,

    /// List of process names that suppress playback
    #[serde(default = "default_blocklist")]
    pub blocklist: Vec<String>,
}

fn default_blocklist() -> Vec<String> {
    vec![
        "zoom".to_string(),
        "teams".to_string(),
        "webex".to_string(),
        "slack".to_string(),
        "discord".to_string(),
        "facetime".to_string(),
        "meet".to_string(),
    ]
}
```

| Field | Type | Required | Description |
|-------|------|----------|-------------|
| blocklist | Vec<String> | No | Process name substrings to match (defaults provided) |

### 3.2 SuppressionResult

Result of checking for blocking applications.

```rust
pub enum SuppressionResult {
    /// No blocking apps detected, proceed with playback
    Clear,

    /// A blocking app is running
    Blocked { app_name: String },
}
```

### 3.3 ProcessDetector

Handles process enumeration and matching.

```rust
pub struct ProcessDetector {
    system: System,
}

impl ProcessDetector {
    /// Create a new detector, initializing the process list
    pub fn new() -> Self;

    /// Check if any blocklisted process is running
    /// Returns the first matching process name, if any
    pub fn check_blocklist(&mut self, blocklist: &[String]) -> SuppressionResult;
}
```

---

## 4. Behaviors

### 4.1 Process Detection

**Purpose:** Determine if any blocklisted application is currently running.

**Algorithm:**
1. Refresh the system's process list
2. For each process, get its name
3. For each blocklist entry, check if process name contains it (case-insensitive)
4. Return first match, or `Clear` if no matches

**Matching Rules:**
- Case-insensitive substring matching
- "zoom" matches "zoom.us", "Zoom", "zoom.us Helper"
- "teams" matches "Microsoft Teams", "Teams", "teams.exe"

**Example Implementation:**
```rust
pub fn check_blocklist(&mut self, blocklist: &[String]) -> SuppressionResult {
    self.system.refresh_processes();

    for process in self.system.processes().values() {
        let name = process.name().to_string_lossy().to_lowercase();

        for blocked in blocklist {
            if name.contains(&blocked.to_lowercase()) {
                return SuppressionResult::Blocked {
                    app_name: process.name().to_string_lossy().into_owned(),
                };
            }
        }
    }

    SuppressionResult::Clear
}
```

### 4.2 Play Command Integration

**Purpose:** Check suppression before playing a sound.

**Modified Flow:**
1. Parse CLI arguments
2. Load configuration
3. **If `--force` is NOT set:**
   - Create ProcessDetector
   - Check blocklist
   - If `Blocked`: print message and exit 0
4. Load archive and play sound (existing behavior)

**CLI Change:**
```rust
Command::Play {
    category: String,

    #[arg(short, long, default_value = "100")]
    volume: u32,

    /// Bypass blocklist check and play anyway
    #[arg(short, long)]
    force: bool,
}
```

### 4.3 Suppression Output

**Purpose:** Inform user when playback is skipped.

**Format:**
```
Skipped: {ProcessName} is running
```

**Examples:**
```
Skipped: zoom.us is running
Skipped: Microsoft Teams is running
Skipped: Discord is running
```

**Exit Code:** 0 (success - skipping is intentional behavior)

---

## 5. Configuration

### 5.1 Config File Format

```toml
sounds_path = "~/.cue/sounds"

# Processes that suppress sound playback (case-insensitive substring match)
blocklist = [
    "zoom",
    "teams",
    "webex",
    "slack",
    "discord",
    "facetime",
    "meet",
]
```

### 5.2 Default Blocklist

If `blocklist` is not specified in config, these defaults apply:

| Entry | Matches |
|-------|---------|
| zoom | Zoom, zoom.us, zoom.us Helper |
| teams | Microsoft Teams, Teams |
| webex | Webex, Cisco Webex, CiscoCollabHost |
| slack | Slack, Slack Helper |
| discord | Discord, Discord Helper |
| facetime | FaceTime |
| meet | Google Meet (browser process names vary) |

### 5.3 Disabling Suppression

To disable suppression entirely, set an empty blocklist:

```toml
blocklist = []
```

### 5.4 Environment Variable

No environment variable override for blocklist (config file only). The `--force` flag provides runtime override.

---

## 6. Implementation Phases

| Phase | Description | Dependencies | Complexity |
|-------|-------------|--------------|------------|
| 1 | Add sysinfo dependency and ProcessDetector | None | Low |
| 2 | Extend Config with blocklist field and defaults | Phase 1 | Low |
| 3 | Integrate suppression check into play command | Phase 2 | Low |
| 4 | Add --force flag to CLI | Phase 3 | Low |

---

## 7. Open Questions

None - design decisions resolved during specification.

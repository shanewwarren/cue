# Configuration Specification

**Status:** Planned
**Version:** 1.0
**Last Updated:** 2025-01-17

---

## 1. Overview

### Purpose

The configuration module manages the sound archive path setting, supporting both a config file and environment variable override.

### Goals

- **Config file** - Persistent configuration in `~/.config/cue/config.toml`
- **Environment override** - `CUE_SOUNDS_PATH` env var overrides config file
- **Sensible defaults** - Default to `~/.cue/sounds` if no config exists

### Non-Goals

- **Multiple archives** - Only one archive path is supported
- **Per-category config** - No category-specific settings
- **Hot reload** - Config is read once at startup

---

## 2. Architecture

### Component Structure

```
src/
├── config/
│   ├── mod.rs           # Module exports
│   └── loader.rs        # Configuration loading logic
```

### Dependencies

```toml
[dependencies]
serde = { version = "1", features = ["derive"] }
toml = "0.8"
directories = "5"  # For XDG paths
```

---

## 3. Core Types

### 3.1 Config

```rust
use serde::Deserialize;
use std::path::PathBuf;

#[derive(Debug, Deserialize)]
pub struct Config {
    /// Path to the sounds archive directory
    pub sounds_path: PathBuf,
}

impl Config {
    /// Load configuration with precedence:
    /// 1. CUE_SOUNDS_PATH environment variable
    /// 2. Config file (~/.config/cue/config.toml)
    /// 3. Default (~/.cue/sounds)
    pub fn load() -> Result<Self, ConfigError>;

    /// Get the config file path
    pub fn config_path() -> PathBuf;
}

impl Default for Config {
    fn default() -> Self {
        Self {
            sounds_path: dirs::home_dir()
                .unwrap_or_default()
                .join(".cue")
                .join("sounds"),
        }
    }
}
```

| Field | Type | Required | Description |
|-------|------|----------|-------------|
| sounds_path | PathBuf | Yes | Absolute or relative path to sounds archive |

### 3.2 ConfigError

```rust
#[derive(Debug, thiserror::Error)]
pub enum ConfigError {
    #[error("Failed to read config file: {0}")]
    ReadError(#[from] std::io::Error),

    #[error("Failed to parse config file: {0}")]
    ParseError(#[from] toml::de::Error),

    #[error("Invalid path in CUE_SOUNDS_PATH: {0}")]
    InvalidEnvPath(String),
}
```

---

## 4. Configuration File

### 4.1 Location

```
~/.config/cue/config.toml
```

On macOS, this expands to:
```
/Users/<username>/.config/cue/config.toml
```

### 4.2 Format

```toml
# Path to the sounds archive directory
sounds_path = "/Users/shanewarren/.claude/sounds"
```

### 4.3 Path Resolution

Paths in the config file can be:
- **Absolute:** `/Users/shanewarren/.claude/sounds`
- **Home-relative:** `~/.claude/sounds` (expanded at load time)
- **Relative:** `./sounds` (relative to current directory - not recommended)

---

## 5. Environment Variable

### 5.1 Variable Name

```
CUE_SOUNDS_PATH
```

### 5.2 Behavior

- If set, completely overrides the config file value
- Supports the same path formats as config file
- Empty string is treated as unset

### 5.3 Example Usage

```bash
# Temporary override
CUE_SOUNDS_PATH=~/my-sounds cue play idle

# Export for session
export CUE_SOUNDS_PATH=~/my-sounds
cue play idle
```

---

## 6. Behaviors

### 6.1 Load Configuration

**Purpose:** Load configuration with precedence rules.

**Algorithm:**
1. Check `CUE_SOUNDS_PATH` environment variable
   - If set and non-empty, use it (expand `~` if present)
   - Return Config with this path
2. Check for config file at `~/.config/cue/config.toml`
   - If exists, parse it
   - Expand `~` in sounds_path if present
   - Return Config from file
3. Return default Config (`~/.cue/sounds`)

**Path Expansion:**
- `~` at start of path is expanded to user's home directory
- `~user` syntax is NOT supported (too complex, rarely needed)

### 6.2 Config Path Discovery

**Purpose:** Return the path where config file should be located.

Uses `directories` crate to get XDG-compliant config directory:
```rust
pub fn config_path() -> PathBuf {
    directories::ProjectDirs::from("", "", "cue")
        .map(|dirs| dirs.config_dir().join("config.toml"))
        .unwrap_or_else(|| PathBuf::from("~/.config/cue/config.toml"))
}
```

---

## 7. Error Handling

### 7.1 Missing Config File

Not an error - use defaults.

### 7.2 Invalid TOML

```
Error: Failed to parse config file: expected `=` at line 2, column 5
```

### 7.3 Nonexistent Path

The config module does NOT validate that the path exists. That's the archive module's responsibility. This allows:
- Creating config before creating the sounds directory
- Helpful error messages from the archive module

---

## 8. Implementation Phases

| Phase | Description | Dependencies | Complexity |
|-------|-------------|--------------|------------|
| 1 | Define Config struct with Default | None | Low |
| 2 | Implement config file loading | Phase 1 | Low |
| 3 | Add environment variable override | Phase 2 | Low |
| 4 | Add tilde expansion | Phase 3 | Low |

---

## 9. Open Questions

None.

# CLI Interface Specification

**Status:** Planned
**Version:** 1.0
**Last Updated:** 2025-01-17

---

## 1. Overview

### Purpose

The CLI interface provides commands for interacting with the sound archive: playing sounds, listing categories, and previewing specific files.

### Goals

- **Intuitive commands** - Simple verbs: `play`, `list`, `preview`
- **Helpful output** - Clear error messages and usage hints
- **Shell-friendly** - Exit codes and minimal output for scripting

### Non-Goals

- **Interactive mode** - No REPL or interactive shell
- **GUI** - Command-line only

---

## 2. Architecture

### Dependency

This module uses `clap` with derive macros:

```toml
[dependencies]
clap = { version = "4", features = ["derive"] }
```

### Component Structure

```
src/
├── main.rs              # Entry point, CLI dispatch
├── cli/
│   ├── mod.rs           # CLI types and parsing
│   └── commands.rs      # Command implementations
```

---

## 3. CLI Structure

### 3.1 Top-Level

```
cue <COMMAND> [OPTIONS]

Commands:
  play     Play a random sound from a category
  list     List available categories or sounds
  preview  Play a specific sound file

Options:
  -h, --help     Print help
  -V, --version  Print version
```

### 3.2 Play Command

```
cue play <CATEGORY> [OPTIONS]

Arguments:
  <CATEGORY>  The category to play from (e.g., "idle", "question")

Options:
  -v, --volume <PERCENT>  Volume level 0-100+ (default: 100)
  -h, --help              Print help

Examples:
  cue play idle
  cue play question --volume 50
```

### 3.3 List Command

```
cue list [CATEGORY]

Arguments:
  [CATEGORY]  Optional category to list sounds from

Options:
  -h, --help  Print help

Examples:
  cue list              # List all categories
  cue list idle         # List sounds in "idle" category
```

**Output Format (categories):**
```
Available categories:
  idle (3 sounds)
  question (5 sounds)
  success (2 sounds)
```

**Output Format (sounds in category):**
```
Sounds in 'idle':
  chime
  ding
  notification
```

### 3.4 Preview Command

```
cue preview <CATEGORY> <SOUND> [OPTIONS]

Arguments:
  <CATEGORY>  The category containing the sound
  <SOUND>     The sound name (without extension)

Options:
  -v, --volume <PERCENT>  Volume level 0-100+ (default: 100)
  -h, --help              Print help

Examples:
  cue preview idle chime
  cue preview question alert --volume 75
```

---

## 4. Core Types

### 4.1 CLI Definition

```rust
use clap::{Parser, Subcommand};

#[derive(Parser)]
#[command(name = "cue")]
#[command(about = "Play categorized audio cues from a sound library")]
#[command(version)]
pub struct Cli {
    #[command(subcommand)]
    pub command: Command,
}

#[derive(Subcommand)]
pub enum Command {
    /// Play a random sound from a category
    Play {
        /// The category to play from
        category: String,

        /// Volume level 0-100+
        #[arg(short, long, default_value = "100")]
        volume: u32,
    },

    /// List available categories or sounds
    List {
        /// Category to list sounds from (omit for all categories)
        category: Option<String>,
    },

    /// Play a specific sound file
    Preview {
        /// The category containing the sound
        category: String,

        /// The sound name (without extension)
        sound: String,

        /// Volume level 0-100+
        #[arg(short, long, default_value = "100")]
        volume: u32,
    },
}
```

---

## 5. Behaviors

### 5.1 Command: play

**Purpose:** Play a random sound from the specified category.

**Flow:**
1. Load configuration to get archive path
2. Load sound archive from path
3. Find category by name (case-insensitive)
4. Select random sound from category
5. Play sound with specified volume
6. Exit with code 0

**Errors:**
| Condition | Exit Code | Message |
|-----------|-----------|---------|
| Category not found | 1 | `Error: Category 'foo' not found. Run 'cue list' to see available categories.` |
| Category empty | 1 | `Error: Category 'foo' has no sound files.` |
| Playback failed | 1 | `Error: Failed to play sound: {reason}` |

### 5.2 Command: list

**Purpose:** List categories or sounds within a category.

**Flow (no argument):**
1. Load configuration and archive
2. Print each category name with sound count
3. Exit with code 0

**Flow (with category):**
1. Load configuration and archive
2. Find category by name
3. Print each sound name in the category
4. Exit with code 0

**Errors:**
| Condition | Exit Code | Message |
|-----------|-----------|---------|
| Category not found | 1 | `Error: Category 'foo' not found.` |

### 5.3 Command: preview

**Purpose:** Play a specific sound file by name.

**Flow:**
1. Load configuration and archive
2. Find category by name
3. Find sound by name within category (case-insensitive, extension optional)
4. Play sound with specified volume
5. Exit with code 0

**Errors:**
| Condition | Exit Code | Message |
|-----------|-----------|---------|
| Category not found | 1 | `Error: Category 'foo' not found.` |
| Sound not found | 1 | `Error: Sound 'bar' not found in category 'foo'. Run 'cue list foo' to see available sounds.` |

---

## 6. Exit Codes

| Code | Meaning |
|------|---------|
| 0 | Success |
| 1 | User error (bad input, not found) |
| 2 | System error (no audio device, config error) |

---

## 7. Implementation Phases

| Phase | Description | Dependencies | Complexity |
|-------|-------------|--------------|------------|
| 1 | Define CLI types with clap derive | None | Low |
| 2 | Implement `list` command | sound-archive | Low |
| 3 | Implement `play` command | sound-archive, playback | Low |
| 4 | Implement `preview` command | sound-archive, playback | Low |

---

## 8. Open Questions

None.

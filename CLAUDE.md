# Cue

A CLI tool that plays categorized audio cues from a sound library.

## Project Overview

Cue allows users to organize sound files into categories (folders) and play random sounds from those categories via command line. Designed to integrate with Claude Code hooks for audio notifications.

## Tech Stack

- **Language:** Rust
- **CLI Framework:** clap (derive macros)
- **Audio Playback:** rodio
- **Config Format:** TOML

## Project Structure

```
src/
├── main.rs              # Entry point, CLI dispatch
├── cli/
│   ├── mod.rs           # CLI types (clap derive)
│   └── commands.rs      # Command implementations
├── archive/
│   ├── mod.rs           # Module exports
│   ├── discovery.rs     # Directory scanning
│   └── types.rs         # SoundArchive, Category, SoundFile
├── playback/
│   ├── mod.rs           # Module exports
│   └── player.rs        # Audio playback via rodio
└── config/
    ├── mod.rs           # Module exports
    └── loader.rs        # Config loading with env override
```

## Commands

```bash
cue play <category>              # Play random sound from category
cue play <category> -v 50        # Play at 50% volume
cue list                         # List all categories
cue list <category>              # List sounds in category
cue preview <category> <sound>   # Play specific sound
```

## Configuration

**Config file:** `~/.config/cue/config.toml`
```toml
sounds_path = "~/.claude/sounds"
```

**Environment override:** `CUE_SOUNDS_PATH`

**Default:** `~/.cue/sounds`

## Specifications

**IMPORTANT:** Before implementing any feature, consult `specs/README.md`.

- **Assume NOT implemented.** Specs describe intent; code describes reality.
- **Check the codebase first.** Search actual code before concluding.
- **Use specs as guidance.** Follow design patterns in relevant spec.
- **Spec index:** `specs/README.md` lists all specs by category.

## Development

```bash
cargo build                      # Build debug
cargo build --release            # Build release
cargo run -- play idle           # Test play command
cargo run -- list                # Test list command
```

## Dependencies

```toml
[dependencies]
clap = { version = "4", features = ["derive"] }
rodio = "0.19"
serde = { version = "1", features = ["derive"] }
toml = "0.8"
directories = "5"
thiserror = "1"
rand = "0.8"
```

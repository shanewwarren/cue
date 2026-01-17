# Cue

A CLI tool that plays categorized audio cues from a sound library. Designed to integrate with Claude Code hooks for audio notifications.

## Installation

### Quick Install (macOS/Linux)

```bash
curl -fsSL https://raw.githubusercontent.com/shanewwarren/cue/master/install.sh | sh
```

### From Source

Requires [Rust](https://rustup.rs/).

```bash
cargo install --git https://github.com/shanewwarren/cue.git
```

### Manual Build

```bash
git clone https://github.com/shanewwarren/cue.git
cd cue
cargo build --release
cp target/release/cue /usr/local/bin/
```

## Usage

```bash
# Play a random sound from a category
cue play <category>

# Play at 50% volume
cue play <category> -v 50

# List all categories
cue list

# List sounds in a category
cue list <category>

# Play a specific sound
cue preview <category> <sound>
```

## Setup

### Sound Library

Organize your sounds into category folders:

```
~/.cue/sounds/
├── success/
│   ├── chime.mp3
│   └── ding.wav
├── error/
│   └── buzz.mp3
└── idle/
    └── notification.mp3
```

### Configuration

Create `~/.config/cue/config.toml`:

```toml
sounds_path = "~/.cue/sounds"
```

Or set the environment variable:

```bash
export CUE_SOUNDS_PATH="/path/to/sounds"
```

**Defaults:** `~/.cue/sounds`

## Claude Code Integration

Add hooks to your Claude Code configuration to play audio cues on events:

```json
{
  "hooks": {
    "PostToolUse": [
      {
        "matcher": "Bash",
        "hooks": ["cue play success"]
      }
    ],
    "Stop": [
      {
        "hooks": ["cue play idle"]
      }
    ]
  }
}
```

## Supported Formats

MP3, WAV, FLAC, OGG, and other formats supported by [rodio](https://github.com/RustAudio/rodio).

## License

MIT

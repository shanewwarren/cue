# Playback Specification

**Status:** Planned
**Version:** 1.0
**Last Updated:** 2025-01-17

---

## 1. Overview

### Purpose

The playback module handles audio file playback using the `rodio` crate. It provides a simple interface to play a sound file to completion with optional volume control.

### Goals

- **Simple playback** - Fire-and-forget audio playback that blocks until complete
- **Volume control** - Adjustable volume from 0.0 (mute) to 1.0+ (amplified)
- **Format support** - Play MP3, WAV, OGG, and FLAC files

### Non-Goals

- **Async playback** - No background playback or callbacks
- **Audio mixing** - No playing multiple sounds simultaneously
- **Audio effects** - No filters, EQ, or audio processing

---

## 2. Architecture

### Component Structure

```
src/
├── playback/
│   ├── mod.rs           # Module exports
│   └── player.rs        # Playback implementation
```

### Dependency

This module depends on `rodio` for audio output:

```toml
[dependencies]
rodio = "0.19"
```

Rodio provides:
- Cross-platform audio output (CoreAudio on macOS)
- MP3 decoding via Symphonia (default feature)
- Simple Sink-based playback with volume control

---

## 3. Core Types

### 3.1 Player

Simple audio player.

```rust
pub struct Player {
    _stream: OutputStream,
    handle: OutputStreamHandle,
}

impl Player {
    /// Create a new player using the default audio output device
    pub fn new() -> Result<Self, PlaybackError>;

    /// Play a sound file, blocking until complete
    pub fn play(&self, path: &Path, volume: f32) -> Result<(), PlaybackError>;
}
```

### 3.2 PlaybackError

```rust
#[derive(Debug, thiserror::Error)]
pub enum PlaybackError {
    #[error("No audio output device available")]
    NoDevice,

    #[error("Failed to open audio file: {0}")]
    FileError(PathBuf),

    #[error("Failed to decode audio: {0}")]
    DecodeError(String),

    #[error("Playback failed: {0}")]
    StreamError(String),
}
```

---

## 4. Behaviors

### 4.1 Play Sound

**Purpose:** Play an audio file to completion.

**Parameters:**
| Parameter | Type | Description |
|-----------|------|-------------|
| path | &Path | Path to the audio file |
| volume | f32 | Volume level (0.0 = mute, 1.0 = normal, >1.0 = amplified) |

**Algorithm:**
1. Open the audio file
2. Create a decoder for the file format
3. Create a Sink attached to the output stream
4. Set the volume on the Sink
5. Append the decoded audio to the Sink
6. Wait for playback to complete (`sink.sleep_until_end()`)

**Example Implementation:**
```rust
pub fn play(&self, path: &Path, volume: f32) -> Result<(), PlaybackError> {
    let file = File::open(path)
        .map_err(|_| PlaybackError::FileError(path.to_path_buf()))?;

    let source = Decoder::new(BufReader::new(file))
        .map_err(|e| PlaybackError::DecodeError(e.to_string()))?;

    let sink = Sink::try_new(&self.handle)
        .map_err(|e| PlaybackError::StreamError(e.to_string()))?;

    sink.set_volume(volume);
    sink.append(source);
    sink.sleep_until_end();

    Ok(())
}
```

### 4.2 Volume Handling

**Volume Scale:**
| Value | Effect |
|-------|--------|
| 0.0 | Muted |
| 0.5 | Half volume |
| 1.0 | Normal volume (default) |
| 2.0 | Double volume (may distort) |

**CLI Mapping:**
The CLI accepts volume as a percentage (0-100+), which is converted to the 0.0-1.0+ scale:
- CLI `--volume 50` → Player volume `0.5`
- CLI `--volume 100` → Player volume `1.0`
- CLI `--volume 150` → Player volume `1.5`

---

## 5. Implementation Phases

| Phase | Description | Dependencies | Complexity |
|-------|-------------|--------------|------------|
| 1 | Implement Player::new() with device detection | None | Low |
| 2 | Implement Player::play() with volume | Phase 1 | Low |

---

## 6. Open Questions

None - rodio handles the complexity.

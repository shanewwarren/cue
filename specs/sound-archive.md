# Sound Archive Specification

**Status:** Implemented
**Version:** 1.0
**Last Updated:** 2025-01-17

---

## 1. Overview

### Purpose

The sound archive module discovers and indexes a directory structure containing categorized audio files. Categories are determined by folder names, and each category can contain multiple sound files.

### Goals

- **Directory-based categories** - Each subdirectory in the archive represents a playable category
- **Multi-format support** - Support MP3, WAV, OGG, and FLAC audio files
- **Fast discovery** - Index the archive quickly without caching (archives are typically small)

### Non-Goals

- **Nested categories** - Only top-level directories are categories (no `idle/morning/` nesting)
- **Metadata parsing** - No ID3 tags or audio metadata extraction
- **Watch mode** - No file system watching for changes (re-scan on each invocation)

---

## 2. Architecture

### Component Structure

```
src/
├── archive/
│   ├── mod.rs           # Module exports
│   ├── discovery.rs     # Directory scanning logic
│   └── types.rs         # Archive and Category types
```

### Data Flow

```
Archive Path (config)
        │
        ▼
┌─────────────────┐
│   Discovery     │──▶ Read directory entries
└─────────────────┘
        │
        ▼
┌─────────────────┐
│   Filtering     │──▶ Filter for directories (categories)
└─────────────────┘    and audio files
        │
        ▼
┌─────────────────┐
│   SoundArchive  │──▶ Structured data ready for use
└─────────────────┘
```

---

## 3. Core Types

### 3.1 SoundArchive

Represents the complete indexed archive.

```rust
pub struct SoundArchive {
    pub path: PathBuf,
    pub categories: Vec<Category>,
}

impl SoundArchive {
    /// Load and index an archive from the given path
    pub fn load(path: impl AsRef<Path>) -> Result<Self, ArchiveError>;

    /// Get a category by name (case-insensitive)
    pub fn category(&self, name: &str) -> Option<&Category>;

    /// List all category names
    pub fn category_names(&self) -> Vec<&str>;
}
```

| Field | Type | Required | Description |
|-------|------|----------|-------------|
| path | PathBuf | Yes | Absolute path to the archive root |
| categories | Vec<Category> | Yes | All discovered categories |

### 3.2 Category

Represents a single category (directory) of sounds.

```rust
pub struct Category {
    pub name: String,
    pub path: PathBuf,
    pub sounds: Vec<SoundFile>,
}

impl Category {
    /// Pick a random sound from this category
    pub fn random(&self) -> Option<&SoundFile>;

    /// Get a sound by filename (case-insensitive, extension optional)
    pub fn sound(&self, name: &str) -> Option<&SoundFile>;
}
```

| Field | Type | Required | Description |
|-------|------|----------|-------------|
| name | String | Yes | Category name (directory name, lowercase) |
| path | PathBuf | Yes | Absolute path to category directory |
| sounds | Vec<SoundFile> | Yes | All sound files in this category |

### 3.3 SoundFile

Represents a single audio file.

```rust
pub struct SoundFile {
    pub name: String,
    pub path: PathBuf,
    pub format: AudioFormat,
}
```

| Field | Type | Required | Description |
|-------|------|----------|-------------|
| name | String | Yes | Filename without extension |
| path | PathBuf | Yes | Absolute path to the file |
| format | AudioFormat | Yes | Detected audio format |

### 3.4 AudioFormat

Supported audio formats.

```rust
pub enum AudioFormat {
    Mp3,
    Wav,
    Ogg,
    Flac,
}

impl AudioFormat {
    /// Try to detect format from file extension
    pub fn from_extension(ext: &str) -> Option<Self>;
}
```

### 3.5 ArchiveError

```rust
#[derive(Debug, thiserror::Error)]
pub enum ArchiveError {
    #[error("Archive path does not exist: {0}")]
    NotFound(PathBuf),

    #[error("Archive path is not a directory: {0}")]
    NotDirectory(PathBuf),

    #[error("Failed to read directory: {0}")]
    ReadError(#[from] std::io::Error),

    #[error("Category not found: {0}")]
    CategoryNotFound(String),

    #[error("Category is empty: {0}")]
    EmptyCategory(String),
}
```

---

## 4. Behaviors

### 4.1 Archive Discovery

**Purpose:** Scan the archive directory and build the index.

**Algorithm:**
1. Verify archive path exists and is a directory
2. Iterate over directory entries
3. For each entry that is a directory, create a Category
4. For each Category, scan for audio files with supported extensions
5. Sort categories and sounds alphabetically for consistent output

**Supported Extensions:**
- `.mp3`
- `.wav`
- `.ogg`
- `.flac`

**Edge Cases:**
- Empty categories are included (but will error when played)
- Hidden files/directories (starting with `.`) are ignored
- Symlinks are followed

### 4.2 Category Lookup

**Purpose:** Find a category by name.

- Lookup is case-insensitive (`idle` matches `Idle` or `IDLE`)
- Returns `None` if not found

### 4.3 Random Sound Selection

**Purpose:** Pick a random sound from a category.

- Uses `rand` crate for randomness
- Returns `None` if category is empty

---

## 5. Implementation Phases

| Phase | Description | Dependencies | Complexity |
|-------|-------------|--------------|------------|
| 1 | Define types (SoundArchive, Category, SoundFile) | None | Low |
| 2 | Implement directory scanning | Phase 1 | Low |
| 3 | Implement category/sound lookup | Phase 2 | Low |

---

## 6. Open Questions

None - this is a straightforward module.

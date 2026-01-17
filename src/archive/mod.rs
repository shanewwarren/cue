use rand::seq::SliceRandom;
use std::path::{Path, PathBuf};
use std::{fs, io};
use thiserror::Error;

#[derive(Debug, Error)]
pub enum ArchiveError {
    #[error("Archive path does not exist: {0}")]
    NotFound(PathBuf),

    #[error("Archive path is not a directory: {0}")]
    NotDirectory(PathBuf),

    #[error("Failed to read directory: {0}")]
    ReadError(#[from] io::Error),

    #[error("Category not found: {0}")]
    CategoryNotFound(String),

    #[error("Category is empty: {0}")]
    EmptyCategory(String),

    #[error("Sound not found: {0}")]
    SoundNotFound(String),
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum AudioFormat {
    Mp3,
    Wav,
    Ogg,
    Flac,
}

impl AudioFormat {
    pub fn from_extension(ext: &str) -> Option<Self> {
        match ext.to_lowercase().as_str() {
            "mp3" => Some(Self::Mp3),
            "wav" => Some(Self::Wav),
            "ogg" => Some(Self::Ogg),
            "flac" => Some(Self::Flac),
            _ => None,
        }
    }
}

#[derive(Debug, Clone)]
pub struct SoundFile {
    pub name: String,
    pub path: PathBuf,
    pub format: AudioFormat,
}

#[derive(Debug, Clone)]
pub struct Category {
    pub name: String,
    pub path: PathBuf,
    pub sounds: Vec<SoundFile>,
}

impl Category {
    /// Pick a random sound from this category
    pub fn random(&self) -> Option<&SoundFile> {
        self.sounds.choose(&mut rand::thread_rng())
    }

    /// Get a sound by filename (case-insensitive, extension optional)
    pub fn sound(&self, name: &str) -> Option<&SoundFile> {
        let name_lower = name.to_lowercase();
        self.sounds.iter().find(|s| {
            s.name.to_lowercase() == name_lower
                || s.path
                    .file_name()
                    .and_then(|f| f.to_str())
                    .map(|f| f.to_lowercase() == name_lower)
                    .unwrap_or(false)
        })
    }
}

#[derive(Debug)]
pub struct SoundArchive {
    pub path: PathBuf,
    pub categories: Vec<Category>,
}

impl SoundArchive {
    /// Load and index an archive from the given path
    pub fn load(path: impl AsRef<Path>) -> Result<Self, ArchiveError> {
        let path = path.as_ref().to_path_buf();

        if !path.exists() {
            return Err(ArchiveError::NotFound(path));
        }

        if !path.is_dir() {
            return Err(ArchiveError::NotDirectory(path));
        }

        let mut categories = Vec::new();

        for entry in fs::read_dir(&path)? {
            let entry = entry?;
            let entry_path = entry.path();

            // Skip hidden directories
            if entry
                .file_name()
                .to_str()
                .map(|s| s.starts_with('.'))
                .unwrap_or(false)
            {
                continue;
            }

            // Only process directories
            if !entry_path.is_dir() {
                continue;
            }

            let category_name = entry
                .file_name()
                .to_str()
                .unwrap_or("unknown")
                .to_lowercase();

            let mut sounds = Vec::new();

            for sound_entry in fs::read_dir(&entry_path)? {
                let sound_entry = sound_entry?;
                let sound_path = sound_entry.path();

                // Skip hidden files
                if sound_entry
                    .file_name()
                    .to_str()
                    .map(|s| s.starts_with('.'))
                    .unwrap_or(false)
                {
                    continue;
                }

                // Only process files with supported extensions
                if !sound_path.is_file() {
                    continue;
                }

                let extension = sound_path
                    .extension()
                    .and_then(|e| e.to_str())
                    .unwrap_or("");

                if let Some(format) = AudioFormat::from_extension(extension) {
                    let name = sound_path
                        .file_stem()
                        .and_then(|s| s.to_str())
                        .unwrap_or("unknown")
                        .to_string();

                    sounds.push(SoundFile {
                        name,
                        path: sound_path,
                        format,
                    });
                }
            }

            // Sort sounds alphabetically
            sounds.sort_by(|a, b| a.name.to_lowercase().cmp(&b.name.to_lowercase()));

            categories.push(Category {
                name: category_name,
                path: entry_path,
                sounds,
            });
        }

        // Sort categories alphabetically
        categories.sort_by(|a, b| a.name.cmp(&b.name));

        Ok(Self { path, categories })
    }

    /// Get a category by name (case-insensitive)
    pub fn category(&self, name: &str) -> Option<&Category> {
        let name_lower = name.to_lowercase();
        self.categories.iter().find(|c| c.name == name_lower)
    }

    /// List all category names
    pub fn category_names(&self) -> Vec<&str> {
        self.categories.iter().map(|c| c.name.as_str()).collect()
    }
}

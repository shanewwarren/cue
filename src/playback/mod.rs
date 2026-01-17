use rodio::{Decoder, OutputStream, OutputStreamHandle, Sink};
use std::fs::File;
use std::io::BufReader;
use std::path::{Path, PathBuf};
use thiserror::Error;

#[derive(Debug, Error)]
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

pub struct Player {
    _stream: OutputStream,
    handle: OutputStreamHandle,
}

impl Player {
    /// Create a new player using the default audio output device
    pub fn new() -> Result<Self, PlaybackError> {
        let (stream, handle) =
            OutputStream::try_default().map_err(|_| PlaybackError::NoDevice)?;

        Ok(Self {
            _stream: stream,
            handle,
        })
    }

    /// Play a sound file, blocking until complete
    pub fn play(&self, path: &Path, volume: f32) -> Result<(), PlaybackError> {
        let file =
            File::open(path).map_err(|_| PlaybackError::FileError(path.to_path_buf()))?;

        let source = Decoder::new(BufReader::new(file))
            .map_err(|e| PlaybackError::DecodeError(e.to_string()))?;

        let sink = Sink::try_new(&self.handle)
            .map_err(|e| PlaybackError::StreamError(e.to_string()))?;

        sink.set_volume(volume);
        sink.append(source);
        sink.sleep_until_end();

        Ok(())
    }
}

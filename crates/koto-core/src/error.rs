//! Error types for Koto

use thiserror::Error;

/// Main error type for Koto operations
#[derive(Error, Debug)]
pub enum KotoError {
    #[error("Audio device error: {0}")]
    AudioDevice(String),

    #[error("Audio stream error: {0}")]
    AudioStream(String),

    #[error("MIDI device error: {0}")]
    MidiDevice(String),

    #[error("File I/O error: {0}")]
    FileIo(#[from] std::io::Error),

    #[error("Invalid sample rate: {0}")]
    InvalidSampleRate(u32),

    #[error("Invalid buffer size: {0}")]
    InvalidBufferSize(usize),

    #[error("Plugin error: {0}")]
    Plugin(String),

    #[error("Project error: {0}")]
    Project(String),
}

/// Result type for Koto operations
pub type KotoResult<T> = Result<T, KotoError>;

//! Koto Core - Core types and traits for the Koto DAW
//!
//! This crate provides fundamental types used throughout the Koto DAW:
//! - Audio buffer types
//! - Time representation (samples, beats, ticks)
//! - MIDI message types
//! - Common traits for audio processing

pub mod error;
pub mod traits;
pub mod types;

pub use error::*;
pub use traits::*;
pub use types::*;

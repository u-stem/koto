//! Koto Core - Core types and traits for the Koto DAW
//!
//! This crate provides fundamental types used throughout the Koto DAW:
//! - Audio buffer types
//! - Time representation (samples, beats, ticks)
//! - MIDI message types
//! - Common traits for audio processing

pub mod types;
pub mod traits;
pub mod error;

pub use types::*;
pub use traits::*;
pub use error::*;

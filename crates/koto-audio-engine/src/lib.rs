//! Koto Audio Engine - Real-time audio processing engine
//!
//! This crate provides the core audio engine for Koto DAW, including:
//! - Audio device management (via cpal)
//! - Real-time audio callback handling
//! - Lock-free communication with the UI thread
//! - Buffer management

mod buffer_pool;
mod callback;
mod command;
mod device;
mod engine;

pub use buffer_pool::*;
pub use callback::*;
pub use command::*;
pub use device::*;
pub use engine::*;

//! Koto Audio Engine - Real-time audio processing engine
//!
//! This crate provides the core audio engine for Koto DAW, including:
//! - Audio device management (via cpal)
//! - Real-time audio callback handling
//! - Lock-free communication with the UI thread
//! - Buffer management

mod engine;
mod device;
mod callback;
mod command;
mod buffer_pool;

pub use engine::*;
pub use device::*;
pub use callback::*;
pub use command::*;
pub use buffer_pool::*;

//! Commands and events for audio engine communication

use koto_core::{SamplePosition, Tempo, TimeSignature};

/// Commands sent from UI thread to audio thread
#[derive(Debug, Clone)]
pub enum AudioCommand {
    /// Start playback
    Play,
    /// Stop playback
    Stop,
    /// Seek to a specific position
    Seek(SamplePosition),
    /// Set tempo
    SetTempo(Tempo),
    /// Set time signature
    SetTimeSignature(TimeSignature),
    /// Start recording
    StartRecording,
    /// Stop recording
    StopRecording,
    /// Set master volume (0.0 to 1.0)
    SetMasterVolume(f32),
    /// Enable/disable metronome
    SetMetronomeEnabled(bool),
}

/// Events sent from audio thread to UI thread
#[derive(Debug, Clone)]
pub enum AudioEvent {
    /// Playhead position update
    PlayheadMoved(SamplePosition),
    /// Meter level update
    MeterUpdate {
        peak_left: f32,
        peak_right: f32,
        rms_left: f32,
        rms_right: f32,
    },
    /// Transport state changed
    TransportStateChanged {
        is_playing: bool,
        is_recording: bool,
    },
    /// Audio device error
    DeviceError(String),
    /// Buffer underrun occurred
    BufferUnderrun,
}

/// Transport state
#[derive(Debug, Clone, Copy, Default)]
pub struct TransportState {
    pub is_playing: bool,
    pub is_recording: bool,
    pub playhead: SamplePosition,
    pub tempo: Tempo,
    pub time_signature: TimeSignature,
    pub loop_enabled: bool,
    pub loop_start: SamplePosition,
    pub loop_end: SamplePosition,
}

impl TransportState {
    pub fn new() -> Self {
        Self {
            tempo: Tempo::DEFAULT,
            time_signature: TimeSignature::COMMON_TIME,
            ..Default::default()
        }
    }
}

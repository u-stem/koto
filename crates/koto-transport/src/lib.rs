//! Koto Transport - Transport control

use koto_core::{SamplePosition, SampleRate, Tempo, TimeSignature};

/// Transport controller
pub struct Transport {
    pub is_playing: bool,
    pub is_recording: bool,
    pub playhead: SamplePosition,
    pub tempo: Tempo,
    pub time_signature: TimeSignature,
    pub sample_rate: SampleRate,
    pub loop_enabled: bool,
    pub loop_start: SamplePosition,
    pub loop_end: SamplePosition,
}

impl Transport {
    pub fn new(sample_rate: SampleRate) -> Self {
        Self {
            is_playing: false,
            is_recording: false,
            playhead: SamplePosition::ZERO,
            tempo: Tempo::DEFAULT,
            time_signature: TimeSignature::COMMON_TIME,
            sample_rate,
            loop_enabled: false,
            loop_start: SamplePosition::ZERO,
            loop_end: SamplePosition::ZERO,
        }
    }

    pub fn play(&mut self) {
        self.is_playing = true;
    }

    pub fn stop(&mut self) {
        self.is_playing = false;
    }

    pub fn pause(&mut self) {
        self.is_playing = false;
    }

    pub fn seek(&mut self, position: SamplePosition) {
        self.playhead = position;
    }

    pub fn rewind(&mut self) {
        self.playhead = SamplePosition::ZERO;
    }

    pub fn set_tempo(&mut self, tempo: Tempo) {
        self.tempo = tempo;
    }

    pub fn set_time_signature(&mut self, time_signature: TimeSignature) {
        self.time_signature = time_signature;
    }

    pub fn start_recording(&mut self) {
        self.is_recording = true;
    }

    pub fn stop_recording(&mut self) {
        self.is_recording = false;
    }

    /// Get playhead position in seconds
    pub fn playhead_seconds(&self) -> f64 {
        self.playhead.to_seconds(self.sample_rate)
    }
}

impl Default for Transport {
    fn default() -> Self {
        Self::new(SampleRate::default())
    }
}

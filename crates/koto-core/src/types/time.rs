//! Time representation types

use super::SampleRate;
use serde::{Deserialize, Serialize};

/// Ticks per quarter note (PPQ) - standard MIDI resolution
pub const TICKS_PER_QUARTER_NOTE: i32 = 960;

/// Position in samples (absolute)
#[derive(
    Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Default, Serialize, Deserialize,
)]
pub struct SamplePosition(pub i64);

impl SamplePosition {
    pub const ZERO: Self = Self(0);

    pub fn from_seconds(seconds: f64, sample_rate: SampleRate) -> Self {
        Self((seconds * sample_rate.as_f64()) as i64)
    }

    pub fn to_seconds(&self, sample_rate: SampleRate) -> f64 {
        self.0 as f64 / sample_rate.as_f64()
    }

    pub fn advance(&mut self, frames: usize) {
        self.0 += frames as i64;
    }
}

impl std::ops::Add for SamplePosition {
    type Output = Self;

    fn add(self, rhs: Self) -> Self::Output {
        Self(self.0 + rhs.0)
    }
}

impl std::ops::Sub for SamplePosition {
    type Output = Self;

    fn sub(self, rhs: Self) -> Self::Output {
        Self(self.0 - rhs.0)
    }
}

/// Musical time position (bars, beats, ticks)
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Default, Serialize, Deserialize)]
pub struct MusicalTime {
    /// Bar number (1-based)
    pub bar: i32,
    /// Beat within bar (1-based)
    pub beat: i32,
    /// Tick within beat (0-based)
    pub tick: i32,
}

impl MusicalTime {
    pub fn new(bar: i32, beat: i32, tick: i32) -> Self {
        Self { bar, beat, tick }
    }

    /// Convert to total ticks from start
    pub fn to_ticks(&self, beats_per_bar: i32) -> i64 {
        let bars = (self.bar - 1) as i64;
        let beats = (self.beat - 1) as i64;
        let ticks = self.tick as i64;

        bars * (beats_per_bar as i64) * (TICKS_PER_QUARTER_NOTE as i64)
            + beats * (TICKS_PER_QUARTER_NOTE as i64)
            + ticks
    }

    /// Create from total ticks
    pub fn from_ticks(total_ticks: i64, beats_per_bar: i32) -> Self {
        let ticks_per_bar = beats_per_bar as i64 * TICKS_PER_QUARTER_NOTE as i64;
        let bar = (total_ticks / ticks_per_bar) as i32 + 1;
        let remaining = total_ticks % ticks_per_bar;
        let beat = (remaining / TICKS_PER_QUARTER_NOTE as i64) as i32 + 1;
        let tick = (remaining % TICKS_PER_QUARTER_NOTE as i64) as i32;

        Self { bar, beat, tick }
    }
}

/// Time signature
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub struct TimeSignature {
    /// Numerator (beats per bar)
    pub numerator: u8,
    /// Denominator (beat unit, e.g., 4 = quarter note)
    pub denominator: u8,
}

impl TimeSignature {
    pub const COMMON_TIME: Self = Self {
        numerator: 4,
        denominator: 4,
    };

    pub const WALTZ_TIME: Self = Self {
        numerator: 3,
        denominator: 4,
    };

    pub fn new(numerator: u8, denominator: u8) -> Self {
        Self {
            numerator,
            denominator,
        }
    }

    pub fn beats_per_bar(&self) -> i32 {
        self.numerator as i32
    }
}

impl Default for TimeSignature {
    fn default() -> Self {
        Self::COMMON_TIME
    }
}

/// Tempo in beats per minute
#[derive(Debug, Clone, Copy, PartialEq, Serialize, Deserialize)]
pub struct Tempo(pub f64);

impl Tempo {
    pub const DEFAULT: Self = Self(120.0);

    pub fn new(bpm: f64) -> Self {
        Self(bpm.clamp(20.0, 999.0))
    }

    pub fn bpm(&self) -> f64 {
        self.0
    }

    /// Get samples per beat at the given sample rate
    pub fn samples_per_beat(&self, sample_rate: SampleRate) -> f64 {
        sample_rate.as_f64() * 60.0 / self.0
    }

    /// Get samples per tick at the given sample rate
    pub fn samples_per_tick(&self, sample_rate: SampleRate) -> f64 {
        self.samples_per_beat(sample_rate) / TICKS_PER_QUARTER_NOTE as f64
    }
}

impl Default for Tempo {
    fn default() -> Self {
        Self::DEFAULT
    }
}

/// Time converter for converting between different time representations
pub struct TimeConverter {
    sample_rate: SampleRate,
    tempo: Tempo,
    time_signature: TimeSignature,
}

impl TimeConverter {
    pub fn new(sample_rate: SampleRate, tempo: Tempo, time_signature: TimeSignature) -> Self {
        Self {
            sample_rate,
            tempo,
            time_signature,
        }
    }

    pub fn samples_to_seconds(&self, samples: SamplePosition) -> f64 {
        samples.to_seconds(self.sample_rate)
    }

    pub fn seconds_to_samples(&self, seconds: f64) -> SamplePosition {
        SamplePosition::from_seconds(seconds, self.sample_rate)
    }

    pub fn samples_to_musical(&self, samples: SamplePosition) -> MusicalTime {
        let seconds = self.samples_to_seconds(samples);
        let beats = seconds * self.tempo.bpm() / 60.0;
        let total_ticks = (beats * TICKS_PER_QUARTER_NOTE as f64) as i64;
        MusicalTime::from_ticks(total_ticks, self.time_signature.beats_per_bar())
    }

    pub fn musical_to_samples(&self, time: MusicalTime) -> SamplePosition {
        let total_ticks = time.to_ticks(self.time_signature.beats_per_bar());
        let beats = total_ticks as f64 / TICKS_PER_QUARTER_NOTE as f64;
        let seconds = beats * 60.0 / self.tempo.bpm();
        self.seconds_to_samples(seconds)
    }
}

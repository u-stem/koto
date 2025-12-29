//! Audio buffer types and sample handling

use serde::{Deserialize, Serialize};

/// A single audio sample (32-bit float, range -1.0 to 1.0)
pub type Sample = f32;

/// Number of audio channels
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub struct ChannelCount(pub u16);

impl ChannelCount {
    pub const MONO: Self = Self(1);
    pub const STEREO: Self = Self(2);

    pub fn as_usize(&self) -> usize {
        self.0 as usize
    }
}

impl Default for ChannelCount {
    fn default() -> Self {
        Self::STEREO
    }
}

/// Sample rate in Hz
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub struct SampleRate(pub u32);

impl SampleRate {
    pub const CD_QUALITY: Self = Self(44100);
    pub const DVD_QUALITY: Self = Self(48000);
    pub const HIGH_RES: Self = Self(96000);

    pub fn as_f64(&self) -> f64 {
        self.0 as f64
    }
}

impl Default for SampleRate {
    fn default() -> Self {
        Self::DVD_QUALITY
    }
}

/// Buffer size in samples
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub struct BufferSize(pub usize);

impl BufferSize {
    pub const SMALL: Self = Self(64);
    pub const MEDIUM: Self = Self(256);
    pub const LARGE: Self = Self(512);
    pub const EXTRA_LARGE: Self = Self(1024);
}

impl Default for BufferSize {
    fn default() -> Self {
        Self::MEDIUM
    }
}

/// An audio buffer containing interleaved samples for multiple channels
#[derive(Debug, Clone)]
pub struct AudioBuffer {
    /// Interleaved sample data
    samples: Vec<Sample>,
    /// Number of channels
    channels: ChannelCount,
    /// Number of frames (samples per channel)
    frames: usize,
}

impl AudioBuffer {
    /// Create a new audio buffer filled with silence
    pub fn new(channels: ChannelCount, frames: usize) -> Self {
        let sample_count = channels.as_usize() * frames;
        Self {
            samples: vec![0.0; sample_count],
            channels,
            frames,
        }
    }

    /// Create a buffer from existing samples
    pub fn from_samples(samples: Vec<Sample>, channels: ChannelCount) -> Self {
        let frames = samples.len() / channels.as_usize();
        Self {
            samples,
            channels,
            frames,
        }
    }

    /// Get the number of channels
    pub fn channels(&self) -> ChannelCount {
        self.channels
    }

    /// Get the number of frames
    pub fn frames(&self) -> usize {
        self.frames
    }

    /// Get a reference to the raw sample data
    pub fn samples(&self) -> &[Sample] {
        &self.samples
    }

    /// Get a mutable reference to the raw sample data
    pub fn samples_mut(&mut self) -> &mut [Sample] {
        &mut self.samples
    }

    /// Get a sample at a specific frame and channel
    pub fn get(&self, frame: usize, channel: usize) -> Option<Sample> {
        if frame < self.frames && channel < self.channels.as_usize() {
            Some(self.samples[frame * self.channels.as_usize() + channel])
        } else {
            None
        }
    }

    /// Set a sample at a specific frame and channel
    pub fn set(&mut self, frame: usize, channel: usize, value: Sample) {
        if frame < self.frames && channel < self.channels.as_usize() {
            self.samples[frame * self.channels.as_usize() + channel] = value;
        }
    }

    /// Fill the buffer with silence
    pub fn clear(&mut self) {
        self.samples.fill(0.0);
    }

    /// Copy samples from another buffer
    pub fn copy_from(&mut self, other: &AudioBuffer) {
        let len = self.samples.len().min(other.samples.len());
        self.samples[..len].copy_from_slice(&other.samples[..len]);
    }

    /// Apply gain to all samples
    pub fn apply_gain(&mut self, gain: f32) {
        for sample in &mut self.samples {
            *sample *= gain;
        }
    }

    /// Mix another buffer into this one
    pub fn mix(&mut self, other: &AudioBuffer) {
        let len = self.samples.len().min(other.samples.len());
        for i in 0..len {
            self.samples[i] += other.samples[i];
        }
    }

    /// Get the peak level (maximum absolute value)
    pub fn peak(&self) -> Sample {
        self.samples.iter().map(|s| s.abs()).fold(0.0, f32::max)
    }
}

/// A stereo frame (left and right samples)
#[derive(Debug, Clone, Copy, Default)]
pub struct StereoFrame {
    pub left: Sample,
    pub right: Sample,
}

impl StereoFrame {
    pub fn new(left: Sample, right: Sample) -> Self {
        Self { left, right }
    }

    pub fn silence() -> Self {
        Self::default()
    }

    pub fn mono(value: Sample) -> Self {
        Self {
            left: value,
            right: value,
        }
    }
}

/// Audio format specification
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub struct AudioFormat {
    pub sample_rate: SampleRate,
    pub channels: ChannelCount,
    pub buffer_size: BufferSize,
}

impl Default for AudioFormat {
    fn default() -> Self {
        Self {
            sample_rate: SampleRate::default(),
            channels: ChannelCount::default(),
            buffer_size: BufferSize::default(),
        }
    }
}

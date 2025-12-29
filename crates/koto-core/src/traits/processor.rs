//! Audio processor traits

use crate::types::{AudioBuffer, MidiEvent, SamplePosition, SampleRate, Tempo, TimeSignature};

/// Context passed to audio processors during processing
pub struct ProcessContext<'a> {
    /// Sample rate
    pub sample_rate: SampleRate,
    /// Current tempo
    pub tempo: Tempo,
    /// Current time signature
    pub time_signature: TimeSignature,
    /// Current playhead position
    pub playhead: SamplePosition,
    /// Number of samples to process
    pub frames: usize,
    /// MIDI events for this buffer
    pub midi_events: &'a [MidiEvent],
    /// Is the transport playing?
    pub is_playing: bool,
    /// Is recording enabled?
    pub is_recording: bool,
}

/// Trait for audio processing nodes
pub trait AudioProcessor: Send + 'static {
    /// Process audio through this processor
    ///
    /// # Arguments
    /// * `inputs` - Input audio buffers
    /// * `outputs` - Output audio buffers (pre-allocated)
    /// * `context` - Processing context
    fn process(
        &mut self,
        inputs: &[AudioBuffer],
        outputs: &mut [AudioBuffer],
        context: &ProcessContext,
    );

    /// Get the number of audio input channels
    fn input_channels(&self) -> usize;

    /// Get the number of audio output channels
    fn output_channels(&self) -> usize;

    /// Called when sample rate changes
    fn set_sample_rate(&mut self, _sample_rate: SampleRate) {}

    /// Reset processor state (e.g., clear delay lines)
    fn reset(&mut self) {}

    /// Get the latency introduced by this processor in samples
    fn latency(&self) -> usize {
        0
    }
}

/// Trait for parameter handling
pub trait ParameterHandler {
    /// Get parameter value by ID
    fn get_parameter(&self, id: u32) -> Option<f32>;

    /// Set parameter value by ID
    fn set_parameter(&mut self, id: u32, value: f32);

    /// Get parameter count
    fn parameter_count(&self) -> usize;
}

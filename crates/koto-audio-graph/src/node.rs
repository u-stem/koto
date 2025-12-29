//! Audio node implementations

use crate::AudioNode;

/// A simple pass-through node
pub struct PassthroughNode {
    inputs: usize,
    outputs: usize,
}

impl PassthroughNode {
    pub fn new(channels: usize) -> Self {
        Self {
            inputs: channels,
            outputs: channels,
        }
    }
}

impl AudioNode for PassthroughNode {
    fn input_count(&self) -> usize {
        self.inputs
    }

    fn output_count(&self) -> usize {
        self.outputs
    }

    fn name(&self) -> &str {
        "Passthrough"
    }
}

/// A gain node that adjusts volume
pub struct GainNode {
    gain: f32,
}

impl GainNode {
    pub fn new(gain: f32) -> Self {
        Self { gain }
    }

    pub fn set_gain(&mut self, gain: f32) {
        self.gain = gain;
    }

    pub fn gain(&self) -> f32 {
        self.gain
    }
}

impl AudioNode for GainNode {
    fn input_count(&self) -> usize {
        2 // Stereo
    }

    fn output_count(&self) -> usize {
        2 // Stereo
    }

    fn name(&self) -> &str {
        "Gain"
    }
}

/// Master output node
pub struct MasterNode;

impl AudioNode for MasterNode {
    fn input_count(&self) -> usize {
        2 // Stereo
    }

    fn output_count(&self) -> usize {
        0 // No output (goes to audio interface)
    }

    fn name(&self) -> &str {
        "Master"
    }
}

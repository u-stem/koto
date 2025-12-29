//! MIDI device management

use koto_core::{KotoError, KotoResult};
use midir::{MidiInput, MidiOutput};

/// MIDI device info
#[derive(Debug, Clone)]
pub struct MidiDeviceInfo {
    pub name: String,
    pub port_number: usize,
}

/// MIDI device manager
pub struct MidiDeviceManager {
    midi_in: Option<MidiInput>,
    midi_out: Option<MidiOutput>,
}

impl MidiDeviceManager {
    pub fn new() -> KotoResult<Self> {
        let midi_in =
            MidiInput::new("Koto MIDI Input").map_err(|e| KotoError::MidiDevice(e.to_string()))?;
        let midi_out = MidiOutput::new("Koto MIDI Output")
            .map_err(|e| KotoError::MidiDevice(e.to_string()))?;

        Ok(Self {
            midi_in: Some(midi_in),
            midi_out: Some(midi_out),
        })
    }

    /// List available input devices
    pub fn list_input_devices(&self) -> Vec<MidiDeviceInfo> {
        self.midi_in
            .as_ref()
            .map(|midi_in| {
                midi_in
                    .ports()
                    .iter()
                    .enumerate()
                    .filter_map(|(i, port)| {
                        midi_in.port_name(port).ok().map(|name| MidiDeviceInfo {
                            name,
                            port_number: i,
                        })
                    })
                    .collect()
            })
            .unwrap_or_default()
    }

    /// List available output devices
    pub fn list_output_devices(&self) -> Vec<MidiDeviceInfo> {
        self.midi_out
            .as_ref()
            .map(|midi_out| {
                midi_out
                    .ports()
                    .iter()
                    .enumerate()
                    .filter_map(|(i, port)| {
                        midi_out.port_name(port).ok().map(|name| MidiDeviceInfo {
                            name,
                            port_number: i,
                        })
                    })
                    .collect()
            })
            .unwrap_or_default()
    }
}

impl Default for MidiDeviceManager {
    fn default() -> Self {
        Self::new().expect("Failed to create MIDI device manager")
    }
}

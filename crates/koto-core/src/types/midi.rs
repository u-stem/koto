//! MIDI message types

use serde::{Deserialize, Serialize};

/// MIDI channel (0-15)
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Default, Serialize, Deserialize)]
pub struct MidiChannel(pub u8);

impl MidiChannel {
    pub fn new(channel: u8) -> Self {
        Self(channel.min(15))
    }
}

/// MIDI note number (0-127)
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub struct NoteNumber(pub u8);

impl NoteNumber {
    /// Middle C (C4)
    pub const MIDDLE_C: Self = Self(60);

    pub fn new(note: u8) -> Self {
        Self(note.min(127))
    }

    /// Get the note name (e.g., "C4", "A#3")
    pub fn name(&self) -> String {
        const NOTE_NAMES: [&str; 12] = [
            "C", "C#", "D", "D#", "E", "F", "F#", "G", "G#", "A", "A#", "B",
        ];
        let octave = (self.0 / 12) as i32 - 1;
        let note = NOTE_NAMES[(self.0 % 12) as usize];
        format!("{}{}", note, octave)
    }

    /// Get frequency in Hz (A4 = 440Hz)
    pub fn frequency(&self) -> f64 {
        440.0 * 2.0_f64.powf((self.0 as f64 - 69.0) / 12.0)
    }
}

/// MIDI velocity (0-127)
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub struct Velocity(pub u8);

impl Velocity {
    pub const OFF: Self = Self(0);
    pub const PIANISSIMO: Self = Self(32);
    pub const PIANO: Self = Self(48);
    pub const MEZZO_PIANO: Self = Self(64);
    pub const MEZZO_FORTE: Self = Self(80);
    pub const FORTE: Self = Self(96);
    pub const FORTISSIMO: Self = Self(112);
    pub const MAX: Self = Self(127);

    pub fn new(velocity: u8) -> Self {
        Self(velocity.min(127))
    }

    /// Convert to normalized value (0.0 to 1.0)
    pub fn normalized(&self) -> f32 {
        self.0 as f32 / 127.0
    }
}

impl Default for Velocity {
    fn default() -> Self {
        Self::MEZZO_FORTE
    }
}

/// MIDI control change number
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub struct ControlNumber(pub u8);

impl ControlNumber {
    pub const MODULATION: Self = Self(1);
    pub const BREATH: Self = Self(2);
    pub const VOLUME: Self = Self(7);
    pub const PAN: Self = Self(10);
    pub const EXPRESSION: Self = Self(11);
    pub const SUSTAIN: Self = Self(64);
    pub const ALL_NOTES_OFF: Self = Self(123);

    pub fn new(cc: u8) -> Self {
        Self(cc.min(127))
    }
}

/// MIDI message types
#[derive(Debug, Clone, Copy, PartialEq, Serialize, Deserialize)]
pub enum MidiMessage {
    /// Note On event
    NoteOn {
        channel: MidiChannel,
        note: NoteNumber,
        velocity: Velocity,
    },
    /// Note Off event
    NoteOff {
        channel: MidiChannel,
        note: NoteNumber,
        velocity: Velocity,
    },
    /// Control Change (CC) event
    ControlChange {
        channel: MidiChannel,
        control: ControlNumber,
        value: u8,
    },
    /// Program Change event
    ProgramChange { channel: MidiChannel, program: u8 },
    /// Pitch Bend event
    PitchBend {
        channel: MidiChannel,
        /// Pitch bend value (-8192 to 8191, 0 = center)
        value: i16,
    },
    /// Channel Aftertouch
    ChannelPressure { channel: MidiChannel, pressure: u8 },
    /// Polyphonic Key Pressure
    PolyPressure {
        channel: MidiChannel,
        note: NoteNumber,
        pressure: u8,
    },
}

impl MidiMessage {
    /// Get the channel for this message
    pub fn channel(&self) -> MidiChannel {
        match self {
            MidiMessage::NoteOn { channel, .. }
            | MidiMessage::NoteOff { channel, .. }
            | MidiMessage::ControlChange { channel, .. }
            | MidiMessage::ProgramChange { channel, .. }
            | MidiMessage::PitchBend { channel, .. }
            | MidiMessage::ChannelPressure { channel, .. }
            | MidiMessage::PolyPressure { channel, .. } => *channel,
        }
    }

    /// Parse raw MIDI bytes into a message
    pub fn from_bytes(data: &[u8]) -> Option<Self> {
        if data.is_empty() {
            return None;
        }

        let status = data[0];
        let channel = MidiChannel::new(status & 0x0F);
        let message_type = status & 0xF0;

        match message_type {
            0x90 if data.len() >= 3 => {
                let note = NoteNumber::new(data[1]);
                let velocity = Velocity::new(data[2]);
                if velocity.0 == 0 {
                    Some(MidiMessage::NoteOff {
                        channel,
                        note,
                        velocity,
                    })
                } else {
                    Some(MidiMessage::NoteOn {
                        channel,
                        note,
                        velocity,
                    })
                }
            }
            0x80 if data.len() >= 3 => Some(MidiMessage::NoteOff {
                channel,
                note: NoteNumber::new(data[1]),
                velocity: Velocity::new(data[2]),
            }),
            0xB0 if data.len() >= 3 => Some(MidiMessage::ControlChange {
                channel,
                control: ControlNumber::new(data[1]),
                value: data[2],
            }),
            0xC0 if data.len() >= 2 => Some(MidiMessage::ProgramChange {
                channel,
                program: data[1],
            }),
            0xE0 if data.len() >= 3 => {
                let lsb = data[1] as i16;
                let msb = data[2] as i16;
                let value = ((msb << 7) | lsb) - 8192;
                Some(MidiMessage::PitchBend { channel, value })
            }
            0xD0 if data.len() >= 2 => Some(MidiMessage::ChannelPressure {
                channel,
                pressure: data[1],
            }),
            0xA0 if data.len() >= 3 => Some(MidiMessage::PolyPressure {
                channel,
                note: NoteNumber::new(data[1]),
                pressure: data[2],
            }),
            _ => None,
        }
    }
}

/// A MIDI event with timing information
#[derive(Debug, Clone, Copy, PartialEq, Serialize, Deserialize)]
pub struct MidiEvent {
    /// Sample offset within the current buffer
    pub sample_offset: usize,
    /// The MIDI message
    pub message: MidiMessage,
}

impl MidiEvent {
    pub fn new(sample_offset: usize, message: MidiMessage) -> Self {
        Self {
            sample_offset,
            message,
        }
    }
}

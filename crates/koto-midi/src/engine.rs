//! MIDI engine

use koto_core::MidiEvent;
use std::collections::VecDeque;

/// MIDI engine for processing and routing MIDI events
pub struct MidiEngine {
    /// Pending events to be processed
    pending_events: VecDeque<MidiEvent>,
    /// Recording buffer
    recording: Vec<MidiEvent>,
    /// Is recording enabled
    is_recording: bool,
}

impl MidiEngine {
    pub fn new() -> Self {
        Self {
            pending_events: VecDeque::new(),
            recording: Vec::new(),
            is_recording: false,
        }
    }

    /// Add an event to be processed
    pub fn push_event(&mut self, event: MidiEvent) {
        if self.is_recording {
            self.recording.push(event);
        }
        self.pending_events.push_back(event);
    }

    /// Get pending events for a buffer
    pub fn drain_events(&mut self) -> Vec<MidiEvent> {
        self.pending_events.drain(..).collect()
    }

    /// Start recording
    pub fn start_recording(&mut self) {
        self.is_recording = true;
        self.recording.clear();
    }

    /// Stop recording and return recorded events
    pub fn stop_recording(&mut self) -> Vec<MidiEvent> {
        self.is_recording = false;
        std::mem::take(&mut self.recording)
    }

    /// Check if recording
    pub fn is_recording(&self) -> bool {
        self.is_recording
    }
}

impl Default for MidiEngine {
    fn default() -> Self {
        Self::new()
    }
}

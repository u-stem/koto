//! Audio callback handler for real-time processing

use crate::{AudioCommand, AudioEvent, TransportState};
use koto_core::SampleRate;
use rtrb::{Consumer, Producer};
use std::sync::Arc;
use parking_lot::Mutex;

/// Audio callback processor
pub struct AudioCallback {
    /// Commands from UI thread
    command_rx: Consumer<AudioCommand>,
    /// Events to UI thread
    event_tx: Producer<AudioEvent>,
    /// Transport state
    transport: TransportState,
    /// Sample rate
    sample_rate: SampleRate,
    /// Master volume (0.0 to 1.0)
    master_volume: f32,
    /// Metronome enabled
    metronome_enabled: bool,
    /// Frame counter for meter updates
    meter_frame_counter: usize,
    /// Frames between meter updates
    meter_update_interval: usize,
    /// Recording buffer (shared with file writer thread)
    recording_buffer: Option<Arc<Mutex<Vec<f32>>>>,
}

impl AudioCallback {
    /// Create a new audio callback
    pub fn new(
        command_rx: Consumer<AudioCommand>,
        event_tx: Producer<AudioEvent>,
        sample_rate: SampleRate,
        buffer_size: usize,
    ) -> Self {
        // Calculate meter update interval (~30 Hz)
        let meter_update_interval = (sample_rate.0 as usize / 30).max(buffer_size);

        Self {
            command_rx,
            event_tx,
            transport: TransportState::new(),
            sample_rate,
            master_volume: 1.0,
            metronome_enabled: false,
            meter_frame_counter: 0,
            meter_update_interval,
            recording_buffer: None,
        }
    }

    /// Process commands from UI thread (non-blocking)
    fn process_commands(&mut self) {
        while let Ok(command) = self.command_rx.pop() {
            match command {
                AudioCommand::Play => {
                    self.transport.is_playing = true;
                    self.send_transport_state();
                }
                AudioCommand::Stop => {
                    self.transport.is_playing = false;
                    self.send_transport_state();
                }
                AudioCommand::Seek(position) => {
                    self.transport.playhead = position;
                }
                AudioCommand::SetTempo(tempo) => {
                    self.transport.tempo = tempo;
                }
                AudioCommand::SetTimeSignature(time_sig) => {
                    self.transport.time_signature = time_sig;
                }
                AudioCommand::StartRecording => {
                    self.transport.is_recording = true;
                    self.recording_buffer = Some(Arc::new(Mutex::new(Vec::with_capacity(
                        self.sample_rate.0 as usize * 60 * 2, // 1 minute stereo
                    ))));
                    self.send_transport_state();
                }
                AudioCommand::StopRecording => {
                    self.transport.is_recording = false;
                    self.recording_buffer = None;
                    self.send_transport_state();
                }
                AudioCommand::SetMasterVolume(volume) => {
                    self.master_volume = volume.clamp(0.0, 1.0);
                }
                AudioCommand::SetMetronomeEnabled(enabled) => {
                    self.metronome_enabled = enabled;
                }
            }
        }
    }

    /// Send transport state to UI thread
    fn send_transport_state(&mut self) {
        let _ = self.event_tx.push(AudioEvent::TransportStateChanged {
            is_playing: self.transport.is_playing,
            is_recording: self.transport.is_recording,
        });
    }

    /// Process audio callback
    ///
    /// This is called from the audio thread and must be real-time safe
    pub fn process(&mut self, output: &mut [f32], input: Option<&[f32]>) {
        // Process any pending commands (non-blocking)
        self.process_commands();

        let channels = 2; // Stereo
        let frames = output.len() / channels;

        // Clear output buffer
        output.fill(0.0);

        // If recording, capture input
        if self.transport.is_recording {
            if let (Some(input_data), Some(buffer)) = (input, &self.recording_buffer) {
                if let Some(mut guard) = buffer.try_lock() {
                    guard.extend_from_slice(input_data);
                }
            }
        }

        // If playing, generate audio
        if self.transport.is_playing {
            // TODO: Process audio graph here
            // For now, generate silence

            // Generate metronome click if enabled
            if self.metronome_enabled {
                self.generate_metronome(output, frames);
            }

            // Advance playhead
            self.transport.playhead.advance(frames);
        }

        // Apply master volume
        for sample in output.iter_mut() {
            *sample *= self.master_volume;
        }

        // Calculate and send meter levels
        self.meter_frame_counter += frames;
        if self.meter_frame_counter >= self.meter_update_interval {
            self.meter_frame_counter = 0;
            self.send_meter_update(output);
        }

        // Send playhead update (~10 Hz)
        if self.transport.is_playing && self.meter_frame_counter == 0 {
            let _ = self
                .event_tx
                .push(AudioEvent::PlayheadMoved(self.transport.playhead));
        }
    }

    /// Generate metronome click
    fn generate_metronome(&mut self, output: &mut [f32], frames: usize) {
        let samples_per_beat = self.transport.tempo.samples_per_beat(self.sample_rate);
        let playhead = self.transport.playhead.0 as f64;

        for frame in 0..frames {
            let sample_pos = playhead + frame as f64;
            let beat_pos = sample_pos / samples_per_beat;
            let beat_phase = beat_pos.fract();

            // Generate a short click at the start of each beat
            if beat_phase < 0.01 {
                let click_amplitude = 0.3;
                let click_freq = if (beat_pos as i32) % self.transport.time_signature.numerator as i32 == 0 {
                    880.0 // A5 for downbeat
                } else {
                    440.0 // A4 for other beats
                };

                let t = beat_phase * 100.0; // Normalize to 0-1 within click
                let envelope = (1.0 - t).max(0.0) as f32;
                let click = (click_freq * std::f64::consts::TAU * sample_pos / self.sample_rate.0 as f64).sin() as f32;
                let sample = click * envelope * click_amplitude;

                output[frame * 2] += sample;
                output[frame * 2 + 1] += sample;
            }
        }
    }

    /// Calculate and send meter levels
    fn send_meter_update(&mut self, output: &[f32]) {
        let mut peak_left = 0.0f32;
        let mut peak_right = 0.0f32;
        let mut sum_left = 0.0f64;
        let mut sum_right = 0.0f64;

        let frames = output.len() / 2;
        for frame in 0..frames {
            let left = output[frame * 2].abs();
            let right = output[frame * 2 + 1].abs();

            peak_left = peak_left.max(left);
            peak_right = peak_right.max(right);
            sum_left += (left * left) as f64;
            sum_right += (right * right) as f64;
        }

        let rms_left = (sum_left / frames as f64).sqrt() as f32;
        let rms_right = (sum_right / frames as f64).sqrt() as f32;

        let _ = self.event_tx.push(AudioEvent::MeterUpdate {
            peak_left,
            peak_right,
            rms_left,
            rms_right,
        });
    }

    /// Get the current transport state
    pub fn transport(&self) -> &TransportState {
        &self.transport
    }

    /// Get the current sample rate
    pub fn sample_rate(&self) -> SampleRate {
        self.sample_rate
    }
}

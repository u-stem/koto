//! Main audio engine

use crate::{AudioCallback, AudioCommand, AudioDeviceManager, AudioEvent};
use cpal::traits::{DeviceTrait, StreamTrait};
use cpal::{Stream, StreamConfig};
use koto_core::{KotoError, KotoResult, SamplePosition, SampleRate, Tempo, TimeSignature};
use parking_lot::Mutex;
use rtrb::RingBuffer;
use std::sync::Arc;
use tracing::{error, info};

/// Ring buffer capacity for commands and events
const COMMAND_BUFFER_SIZE: usize = 256;
const EVENT_BUFFER_SIZE: usize = 1024;

/// The main audio engine
pub struct AudioEngine {
    /// Command sender to audio thread
    command_tx: rtrb::Producer<AudioCommand>,
    /// Event receiver from audio thread
    event_rx: rtrb::Consumer<AudioEvent>,
    /// Output stream
    _output_stream: Option<Stream>,
    /// Input stream
    _input_stream: Option<Stream>,
    /// Device manager
    device_manager: AudioDeviceManager,
    /// Sample rate
    sample_rate: SampleRate,
    /// Is engine running
    is_running: bool,
}

impl AudioEngine {
    /// Create a new audio engine
    pub fn new() -> KotoResult<Self> {
        let device_manager = AudioDeviceManager::new()?;

        // Create command and event channels
        let (command_tx, _command_rx) = RingBuffer::new(COMMAND_BUFFER_SIZE);
        let (_event_tx, event_rx) = RingBuffer::new(EVENT_BUFFER_SIZE);

        Ok(Self {
            command_tx,
            event_rx,
            _output_stream: None,
            _input_stream: None,
            device_manager,
            sample_rate: SampleRate::default(),
            is_running: false,
        })
    }

    /// Start the audio engine with default devices
    pub fn start(&mut self) -> KotoResult<()> {
        if self.is_running {
            return Ok(());
        }

        // Get default output device
        let output_device = self
            .device_manager
            .default_output_device()
            .ok_or(KotoError::AudioDevice("No output device".to_string()))?;

        let output_name = output_device.name().unwrap_or_default();
        info!("Using output device: {}", output_name);

        // Get default config
        let output_config = output_device
            .default_output_config()
            .map_err(|e| KotoError::AudioDevice(e.to_string()))?;

        self.sample_rate = SampleRate(output_config.sample_rate().0);
        let channels = output_config.channels() as usize;
        let buffer_size = 512; // Default buffer size

        info!(
            "Output config: {}Hz, {} channels",
            self.sample_rate.0, channels
        );

        // Create new command/event channels
        let (command_tx, command_rx) = RingBuffer::new(COMMAND_BUFFER_SIZE);
        let (event_tx, event_rx) = RingBuffer::new(EVENT_BUFFER_SIZE);

        self.command_tx = command_tx;
        self.event_rx = event_rx;

        // Create audio callback
        let callback = Arc::new(Mutex::new(AudioCallback::new(
            command_rx,
            event_tx,
            self.sample_rate,
            buffer_size,
        )));

        // Create output stream
        let callback_clone = callback.clone();
        let stream_config = StreamConfig {
            channels: output_config.channels(),
            sample_rate: output_config.sample_rate(),
            buffer_size: cpal::BufferSize::Default,
        };

        let output_stream = output_device
            .build_output_stream(
                &stream_config,
                move |data: &mut [f32], _: &cpal::OutputCallbackInfo| {
                    if let Some(mut cb) = callback_clone.try_lock() {
                        cb.process(data, None);
                    } else {
                        // If we can't get the lock, output silence
                        data.fill(0.0);
                    }
                },
                move |err| {
                    error!("Output stream error: {}", err);
                },
                None,
            )
            .map_err(|e| KotoError::AudioStream(e.to_string()))?;

        output_stream
            .play()
            .map_err(|e| KotoError::AudioStream(e.to_string()))?;

        self._output_stream = Some(output_stream);
        self.is_running = true;

        info!("Audio engine started");
        Ok(())
    }

    /// Stop the audio engine
    pub fn stop(&mut self) {
        self._output_stream = None;
        self._input_stream = None;
        self.is_running = false;
        info!("Audio engine stopped");
    }

    /// Send a command to the audio thread
    pub fn send_command(&mut self, command: AudioCommand) -> bool {
        self.command_tx.push(command).is_ok()
    }

    /// Receive events from the audio thread
    pub fn receive_events(&mut self) -> Vec<AudioEvent> {
        let mut events = Vec::new();
        while let Ok(event) = self.event_rx.pop() {
            events.push(event);
        }
        events
    }

    /// Start playback
    pub fn play(&mut self) {
        self.send_command(AudioCommand::Play);
    }

    /// Stop playback
    pub fn stop_playback(&mut self) {
        self.send_command(AudioCommand::Stop);
    }

    /// Seek to position
    pub fn seek(&mut self, position: SamplePosition) {
        self.send_command(AudioCommand::Seek(position));
    }

    /// Set tempo
    pub fn set_tempo(&mut self, tempo: Tempo) {
        self.send_command(AudioCommand::SetTempo(tempo));
    }

    /// Set time signature
    pub fn set_time_signature(&mut self, time_signature: TimeSignature) {
        self.send_command(AudioCommand::SetTimeSignature(time_signature));
    }

    /// Start recording
    pub fn start_recording(&mut self) {
        self.send_command(AudioCommand::StartRecording);
    }

    /// Stop recording
    pub fn stop_recording(&mut self) {
        self.send_command(AudioCommand::StopRecording);
    }

    /// Set master volume
    pub fn set_master_volume(&mut self, volume: f32) {
        self.send_command(AudioCommand::SetMasterVolume(volume));
    }

    /// Enable/disable metronome
    pub fn set_metronome_enabled(&mut self, enabled: bool) {
        self.send_command(AudioCommand::SetMetronomeEnabled(enabled));
    }

    /// Get the sample rate
    pub fn sample_rate(&self) -> SampleRate {
        self.sample_rate
    }

    /// Check if engine is running
    pub fn is_running(&self) -> bool {
        self.is_running
    }

    /// Get device manager
    pub fn device_manager(&self) -> &AudioDeviceManager {
        &self.device_manager
    }
}

impl Drop for AudioEngine {
    fn drop(&mut self) {
        self.stop();
    }
}

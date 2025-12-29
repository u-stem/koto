//! Main application state and UI

use crate::theme::KotoTheme;
use egui::{CentralPanel, Context, TopBottomPanel};
use koto_audio_engine::{AudioEngine, AudioEvent};
use koto_core::{SamplePosition, Tempo};

/// Main application state
pub struct KotoApp {
    /// Audio engine
    pub audio_engine: AudioEngine,
    /// UI theme
    pub theme: KotoTheme,
    /// Current playhead position
    pub playhead: SamplePosition,
    /// Current tempo
    pub tempo: Tempo,
    /// Is playing
    pub is_playing: bool,
    /// Is recording
    pub is_recording: bool,
    /// Peak meters (left, right)
    pub peak_meters: (f32, f32),
    /// Master volume
    pub master_volume: f32,
    /// Metronome enabled
    pub metronome_enabled: bool,
}

impl KotoApp {
    /// Create a new application
    pub fn new(_cc: &eframe::CreationContext<'_>) -> Self {
        let mut audio_engine = AudioEngine::new().expect("Failed to create audio engine");

        // Start audio engine
        if let Err(e) = audio_engine.start() {
            tracing::error!("Failed to start audio engine: {}", e);
        }

        Self {
            audio_engine,
            theme: KotoTheme::dark(),
            playhead: SamplePosition::ZERO,
            tempo: Tempo::DEFAULT,
            is_playing: false,
            is_recording: false,
            peak_meters: (0.0, 0.0),
            master_volume: 1.0,
            metronome_enabled: false,
        }
    }

    /// Process events from audio engine
    fn process_audio_events(&mut self) {
        for event in self.audio_engine.receive_events() {
            match event {
                AudioEvent::PlayheadMoved(pos) => {
                    self.playhead = pos;
                }
                AudioEvent::MeterUpdate {
                    peak_left,
                    peak_right,
                    ..
                } => {
                    self.peak_meters = (peak_left, peak_right);
                }
                AudioEvent::TransportStateChanged {
                    is_playing,
                    is_recording,
                } => {
                    self.is_playing = is_playing;
                    self.is_recording = is_recording;
                }
                AudioEvent::DeviceError(err) => {
                    tracing::error!("Audio device error: {}", err);
                }
                AudioEvent::BufferUnderrun => {
                    tracing::warn!("Audio buffer underrun");
                }
            }
        }
    }
}

impl eframe::App for KotoApp {
    fn update(&mut self, ctx: &Context, _frame: &mut eframe::Frame) {
        // Apply theme
        self.theme.apply(ctx);

        // Process audio events
        self.process_audio_events();

        // Top toolbar
        TopBottomPanel::top("toolbar").show(ctx, |ui| {
            ui.horizontal(|ui| {
                ui.heading("Koto");
                ui.separator();

                // Transport controls
                if ui.button(if self.is_playing { "⏸" } else { "▶" }).clicked() {
                    if self.is_playing {
                        self.audio_engine.stop_playback();
                    } else {
                        self.audio_engine.play();
                    }
                }

                if ui.button("⏹").clicked() {
                    self.audio_engine.stop_playback();
                    self.audio_engine.seek(SamplePosition::ZERO);
                }

                let rec_button = ui.button(if self.is_recording { "⏺ REC" } else { "⏺" });
                if rec_button.clicked() {
                    if self.is_recording {
                        self.audio_engine.stop_recording();
                    } else {
                        self.audio_engine.start_recording();
                    }
                }

                ui.separator();

                // Tempo
                ui.label("BPM:");
                let mut bpm = self.tempo.bpm();
                if ui
                    .add(
                        egui::DragValue::new(&mut bpm)
                            .speed(0.1)
                            .range(20.0..=999.0),
                    )
                    .changed()
                {
                    self.tempo = Tempo::new(bpm);
                    self.audio_engine.set_tempo(self.tempo);
                }

                ui.separator();

                // Metronome
                if ui.checkbox(&mut self.metronome_enabled, "🔔").changed() {
                    self.audio_engine
                        .set_metronome_enabled(self.metronome_enabled);
                }

                ui.separator();

                // Time display
                let seconds = self.playhead.to_seconds(self.audio_engine.sample_rate());
                let minutes = (seconds / 60.0) as i32;
                let secs = seconds % 60.0;
                ui.label(format!("{:02}:{:05.2}", minutes, secs));
            });
        });

        // Bottom status bar
        TopBottomPanel::bottom("status_bar").show(ctx, |ui| {
            ui.horizontal(|ui| {
                // Peak meters
                let meter_width = 100.0;
                let meter_height = 12.0;

                ui.label("L:");
                let (rect, _) = ui.allocate_exact_size(
                    egui::vec2(meter_width, meter_height),
                    egui::Sense::hover(),
                );
                let meter_level = (self.peak_meters.0 * meter_width) as f32;
                ui.painter().rect_filled(rect, 2.0, self.theme.surface);
                let mut filled_rect = rect;
                filled_rect.set_width(meter_level);
                let color = if self.peak_meters.0 > 0.9 {
                    self.theme.error
                } else if self.peak_meters.0 > 0.7 {
                    self.theme.warning
                } else {
                    self.theme.success
                };
                ui.painter().rect_filled(filled_rect, 2.0, color);

                ui.label("R:");
                let (rect, _) = ui.allocate_exact_size(
                    egui::vec2(meter_width, meter_height),
                    egui::Sense::hover(),
                );
                let meter_level = (self.peak_meters.1 * meter_width) as f32;
                ui.painter().rect_filled(rect, 2.0, self.theme.surface);
                let mut filled_rect = rect;
                filled_rect.set_width(meter_level);
                let color = if self.peak_meters.1 > 0.9 {
                    self.theme.error
                } else if self.peak_meters.1 > 0.7 {
                    self.theme.warning
                } else {
                    self.theme.success
                };
                ui.painter().rect_filled(filled_rect, 2.0, color);

                ui.separator();

                // Master volume
                ui.label("Master:");
                if ui
                    .add(egui::Slider::new(&mut self.master_volume, 0.0..=1.0).show_value(false))
                    .changed()
                {
                    self.audio_engine.set_master_volume(self.master_volume);
                }

                ui.with_layout(egui::Layout::right_to_left(egui::Align::Center), |ui| {
                    ui.label(format!("{}Hz", self.audio_engine.sample_rate().0));
                });
            });
        });

        // Main content area
        CentralPanel::default().show(ctx, |ui| {
            ui.centered_and_justified(|ui| {
                ui.heading("Welcome to Koto DAW");
            });
        });

        // Request repaint for smooth animation
        ctx.request_repaint();
    }
}

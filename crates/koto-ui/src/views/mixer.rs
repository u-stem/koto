//! Mixer view

use egui::Ui;

/// Mixer console view
pub struct MixerView {
    /// Show mixer
    pub visible: bool,
}

impl Default for MixerView {
    fn default() -> Self {
        Self { visible: true }
    }
}

impl MixerView {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn ui(&mut self, ui: &mut Ui) {
        ui.horizontal(|ui| {
            ui.label("Mixer - Coming soon");
        });
    }
}

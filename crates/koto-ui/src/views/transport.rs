//! Transport bar view

use egui::Ui;

/// Transport control bar
pub struct TransportView;

impl TransportView {
    pub fn new() -> Self {
        Self
    }

    pub fn ui(&mut self, ui: &mut Ui) {
        ui.horizontal(|ui| {
            ui.label("Transport");
        });
    }
}

impl Default for TransportView {
    fn default() -> Self {
        Self::new()
    }
}

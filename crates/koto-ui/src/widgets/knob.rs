//! Rotary knob widget

use egui::{Color32, Pos2, Response, Sense, Ui, Vec2};
use std::f32::consts::PI;

/// Rotary knob widget
pub struct KnobWidget {
    /// Current value (0.0 to 1.0)
    value: f32,
    /// Label
    label: String,
    /// Size
    size: f32,
}

impl KnobWidget {
    pub fn new(value: f32, label: impl Into<String>) -> Self {
        Self {
            value: value.clamp(0.0, 1.0),
            label: label.into(),
            size: 40.0,
        }
    }

    pub fn size(mut self, size: f32) -> Self {
        self.size = size;
        self
    }

    pub fn ui(mut self, ui: &mut Ui) -> (Response, f32) {
        let desired_size = Vec2::splat(self.size);
        let (rect, response) = ui.allocate_exact_size(desired_size, Sense::click_and_drag());

        // Handle drag
        if response.dragged() {
            let delta = response.drag_delta();
            self.value = (self.value - delta.y * 0.01).clamp(0.0, 1.0);
        }

        // Draw knob
        let painter = ui.painter();
        let center = rect.center();
        let radius = rect.width() / 2.0 - 2.0;

        // Background circle
        painter.circle_filled(center, radius, Color32::from_rgb(50, 50, 55));

        // Arc showing value
        let start_angle = PI * 0.75;
        let end_angle = PI * 2.25;
        let value_angle = start_angle + (end_angle - start_angle) * self.value;

        // Draw arc segments
        let segments = 32;
        for i in 0..segments {
            let t = i as f32 / segments as f32;
            let angle = start_angle + t * (value_angle - start_angle);
            if angle > value_angle {
                break;
            }

            let x = center.x + angle.cos() * (radius - 4.0);
            let y = center.y + angle.sin() * (radius - 4.0);
            painter.circle_filled(Pos2::new(x, y), 2.0, Color32::from_rgb(74, 144, 226));
        }

        // Indicator line
        let indicator_len = radius * 0.6;
        let indicator_end = Pos2::new(
            center.x + value_angle.cos() * indicator_len,
            center.y + value_angle.sin() * indicator_len,
        );
        painter.line_segment([center, indicator_end], (2.0, Color32::WHITE));

        // Center dot
        painter.circle_filled(center, 3.0, Color32::from_rgb(80, 80, 90));

        // Label
        if !self.label.is_empty() {
            painter.text(
                Pos2::new(center.x, rect.bottom() + 4.0),
                egui::Align2::CENTER_TOP,
                &self.label,
                egui::FontId::proportional(10.0),
                Color32::from_rgb(150, 150, 160),
            );
        }

        (response, self.value)
    }
}

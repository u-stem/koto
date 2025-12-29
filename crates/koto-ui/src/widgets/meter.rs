//! Level meter widget

use egui::{Color32, Rect, Ui, Vec2};

/// VU/Peak level meter
pub struct MeterWidget {
    /// Current level (0.0 to 1.0)
    pub level: f32,
    /// Peak level
    pub peak: f32,
    /// Peak hold time
    peak_hold: f32,
}

impl Default for MeterWidget {
    fn default() -> Self {
        Self {
            level: 0.0,
            peak: 0.0,
            peak_hold: 0.0,
        }
    }
}

impl MeterWidget {
    pub fn new() -> Self {
        Self::default()
    }

    /// Update the meter level
    pub fn set_level(&mut self, level: f32) {
        self.level = level.clamp(0.0, 1.0);

        // Update peak
        if level > self.peak {
            self.peak = level;
            self.peak_hold = 1.0;
        }
    }

    /// Decay the peak indicator
    pub fn decay(&mut self, dt: f32) {
        // Decay peak hold
        self.peak_hold -= dt;
        if self.peak_hold <= 0.0 {
            self.peak *= 0.95;
        }

        // Decay level
        self.level *= 0.9;
    }

    /// Render the meter
    pub fn ui(&mut self, ui: &mut Ui, size: Vec2) {
        let (response, painter) = ui.allocate_painter(size, egui::Sense::hover());
        let rect = response.rect;

        // Background
        painter.rect_filled(rect, 2.0, Color32::from_rgb(30, 30, 35));

        // Level gradient
        let level_height = rect.height() * self.level;
        let level_rect = Rect::from_min_max(
            egui::pos2(rect.left(), rect.bottom() - level_height),
            rect.max,
        );

        // Color based on level
        let color = if self.level > 0.9 {
            Color32::from_rgb(231, 76, 60) // Red
        } else if self.level > 0.7 {
            Color32::from_rgb(241, 196, 15) // Yellow
        } else {
            Color32::from_rgb(46, 204, 113) // Green
        };

        painter.rect_filled(level_rect, 2.0, color);

        // Peak indicator
        if self.peak > 0.01 {
            let peak_y = rect.bottom() - rect.height() * self.peak;
            painter.line_segment(
                [
                    egui::pos2(rect.left(), peak_y),
                    egui::pos2(rect.right(), peak_y),
                ],
                (2.0, Color32::WHITE),
            );
        }

        // Scale marks
        for i in 0..=10 {
            let y = rect.top() + rect.height() * (i as f32 / 10.0);
            painter.line_segment(
                [
                    egui::pos2(rect.right() - 3.0, y),
                    egui::pos2(rect.right(), y),
                ],
                (1.0, Color32::from_rgb(80, 80, 90)),
            );
        }
    }
}

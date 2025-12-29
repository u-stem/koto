//! Waveform display widget

use egui::{Color32, Pos2, Ui, Vec2};

/// Waveform display widget
pub struct WaveformWidget {
    /// Waveform data (min/max pairs per pixel)
    pub data: Vec<(f32, f32)>,
    /// Waveform color
    pub color: Color32,
}

impl Default for WaveformWidget {
    fn default() -> Self {
        Self {
            data: Vec::new(),
            color: Color32::from_rgb(74, 144, 226),
        }
    }
}

impl WaveformWidget {
    pub fn new() -> Self {
        Self::default()
    }

    /// Set waveform data from audio samples
    pub fn set_data(&mut self, samples: &[f32], samples_per_pixel: usize) {
        self.data.clear();

        for chunk in samples.chunks(samples_per_pixel) {
            let mut min = f32::MAX;
            let mut max = f32::MIN;

            for &sample in chunk {
                min = min.min(sample);
                max = max.max(sample);
            }

            if min != f32::MAX && max != f32::MIN {
                self.data.push((min, max));
            }
        }
    }

    /// Render the waveform
    pub fn ui(&self, ui: &mut Ui, size: Vec2) {
        let (response, painter) = ui.allocate_painter(size, egui::Sense::hover());
        let rect = response.rect;

        // Background
        painter.rect_filled(rect, 2.0, Color32::from_rgb(25, 25, 30));

        if self.data.is_empty() {
            return;
        }

        let center_y = rect.center().y;
        let half_height = rect.height() / 2.0;

        // Draw waveform
        for (i, &(min, max)) in self.data.iter().enumerate() {
            let x = rect.left() + i as f32;
            if x > rect.right() {
                break;
            }

            let y_min = center_y - min * half_height;
            let y_max = center_y - max * half_height;

            painter.line_segment(
                [Pos2::new(x, y_min), Pos2::new(x, y_max)],
                (1.0, self.color),
            );
        }

        // Center line
        painter.line_segment(
            [
                Pos2::new(rect.left(), center_y),
                Pos2::new(rect.right(), center_y),
            ],
            (1.0, Color32::from_rgb(60, 60, 70)),
        );
    }
}

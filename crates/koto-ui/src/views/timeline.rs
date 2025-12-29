//! Timeline view

use egui::{Color32, Pos2, Rect, Ui, Vec2};

/// Timeline view for arranging audio and MIDI regions
pub struct TimelineView {
    /// Horizontal zoom level (pixels per second)
    pub zoom: f32,
    /// Horizontal scroll position in seconds
    pub scroll: f32,
    /// Track height in pixels
    pub track_height: f32,
}

impl Default for TimelineView {
    fn default() -> Self {
        Self {
            zoom: 50.0,
            scroll: 0.0,
            track_height: 80.0,
        }
    }
}

impl TimelineView {
    pub fn new() -> Self {
        Self::default()
    }

    /// Convert time (seconds) to x position
    pub fn time_to_x(&self, time: f64, offset: f32) -> f32 {
        offset + (time as f32 - self.scroll) * self.zoom
    }

    /// Convert x position to time (seconds)
    pub fn x_to_time(&self, x: f32, offset: f32) -> f64 {
        ((x - offset) / self.zoom + self.scroll) as f64
    }

    /// Render the timeline
    pub fn ui(&mut self, ui: &mut Ui) {
        let available_size = ui.available_size();
        let (response, painter) =
            ui.allocate_painter(available_size, egui::Sense::click_and_drag());
        let rect = response.rect;

        // Background
        painter.rect_filled(rect, 0.0, Color32::from_rgb(30, 30, 34));

        // Draw time ruler
        self.draw_ruler(&painter, rect);

        // Draw grid lines
        self.draw_grid(&painter, rect);

        // Handle scroll
        if response.dragged() {
            let delta = response.drag_delta();
            self.scroll -= delta.x / self.zoom;
            self.scroll = self.scroll.max(0.0);
        }

        // Handle zoom
        if let Some(hover_pos) = response.hover_pos() {
            let scroll_delta = ui.input(|i| i.raw_scroll_delta);
            if scroll_delta.y != 0.0 {
                let old_time = self.x_to_time(hover_pos.x, rect.left());
                self.zoom *= 1.0 + scroll_delta.y * 0.001;
                self.zoom = self.zoom.clamp(10.0, 500.0);
                let new_time = self.x_to_time(hover_pos.x, rect.left());
                self.scroll += (old_time - new_time) as f32;
                self.scroll = self.scroll.max(0.0);
            }
        }
    }

    fn draw_ruler(&self, painter: &egui::Painter, rect: Rect) {
        let ruler_height = 24.0;
        let ruler_rect = Rect::from_min_size(rect.min, Vec2::new(rect.width(), ruler_height));
        painter.rect_filled(ruler_rect, 0.0, Color32::from_rgb(40, 40, 45));

        // Draw time markers
        let start_time = self.scroll.floor() as i32;
        let end_time = ((self.scroll + rect.width() / self.zoom).ceil() as i32).max(start_time + 1);

        for t in start_time..=end_time {
            let x = self.time_to_x(t as f64, rect.left());
            if x >= rect.left() && x <= rect.right() {
                // Draw tick
                painter.line_segment(
                    [
                        Pos2::new(x, ruler_rect.bottom() - 8.0),
                        Pos2::new(x, ruler_rect.bottom()),
                    ],
                    (1.0, Color32::from_rgb(100, 100, 110)),
                );

                // Draw time label
                let minutes = t / 60;
                let seconds = t % 60;
                let text = format!("{}:{:02}", minutes, seconds);
                painter.text(
                    Pos2::new(x + 4.0, ruler_rect.top() + 4.0),
                    egui::Align2::LEFT_TOP,
                    text,
                    egui::FontId::proportional(10.0),
                    Color32::from_rgb(150, 150, 160),
                );
            }
        }
    }

    fn draw_grid(&self, painter: &egui::Painter, rect: Rect) {
        let start_time = self.scroll.floor() as i32;
        let end_time = ((self.scroll + rect.width() / self.zoom).ceil() as i32).max(start_time + 1);

        for t in start_time..=end_time {
            let x = self.time_to_x(t as f64, rect.left());
            if x >= rect.left() && x <= rect.right() {
                painter.line_segment(
                    [Pos2::new(x, rect.top() + 24.0), Pos2::new(x, rect.bottom())],
                    (1.0, Color32::from_rgb(45, 45, 50)),
                );
            }
        }
    }
}

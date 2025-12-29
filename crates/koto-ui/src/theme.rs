//! UI theming

use egui::{Color32, Visuals, Style, Rounding, Stroke};

/// Koto dark theme colors
pub struct KotoTheme {
    pub background: Color32,
    pub surface: Color32,
    pub primary: Color32,
    pub secondary: Color32,
    pub accent: Color32,
    pub text: Color32,
    pub text_dim: Color32,
    pub success: Color32,
    pub warning: Color32,
    pub error: Color32,
    pub track_colors: Vec<Color32>,
}

impl Default for KotoTheme {
    fn default() -> Self {
        Self::dark()
    }
}

impl KotoTheme {
    /// Create the dark theme
    pub fn dark() -> Self {
        Self {
            background: Color32::from_rgb(24, 24, 28),
            surface: Color32::from_rgb(32, 32, 36),
            primary: Color32::from_rgb(74, 144, 226),
            secondary: Color32::from_rgb(128, 128, 140),
            accent: Color32::from_rgb(255, 159, 67),
            text: Color32::from_rgb(230, 230, 235),
            text_dim: Color32::from_rgb(140, 140, 150),
            success: Color32::from_rgb(46, 204, 113),
            warning: Color32::from_rgb(241, 196, 15),
            error: Color32::from_rgb(231, 76, 60),
            track_colors: vec![
                Color32::from_rgb(74, 144, 226),   // Blue
                Color32::from_rgb(46, 204, 113),   // Green
                Color32::from_rgb(155, 89, 182),   // Purple
                Color32::from_rgb(241, 196, 15),   // Yellow
                Color32::from_rgb(231, 76, 60),    // Red
                Color32::from_rgb(26, 188, 156),   // Teal
                Color32::from_rgb(230, 126, 34),   // Orange
                Color32::from_rgb(52, 73, 94),     // Dark blue
            ],
        }
    }

    /// Apply theme to egui context
    pub fn apply(&self, ctx: &egui::Context) {
        let mut style = Style::default();

        // Set dark visuals
        let mut visuals = Visuals::dark();

        visuals.window_fill = self.surface;
        visuals.panel_fill = self.background;
        visuals.faint_bg_color = self.surface;
        visuals.extreme_bg_color = Color32::from_rgb(16, 16, 20);

        visuals.widgets.noninteractive.bg_fill = self.surface;
        visuals.widgets.noninteractive.fg_stroke = Stroke::new(1.0, self.text_dim);

        visuals.widgets.inactive.bg_fill = self.surface;
        visuals.widgets.inactive.fg_stroke = Stroke::new(1.0, self.text);

        visuals.widgets.hovered.bg_fill = Color32::from_rgb(50, 50, 56);
        visuals.widgets.hovered.fg_stroke = Stroke::new(1.0, self.text);

        visuals.widgets.active.bg_fill = self.primary;
        visuals.widgets.active.fg_stroke = Stroke::new(1.0, Color32::WHITE);

        visuals.selection.bg_fill = self.primary.linear_multiply(0.5);
        visuals.selection.stroke = Stroke::new(1.0, self.primary);

        visuals.window_rounding = Rounding::same(8.0);
        visuals.menu_rounding = Rounding::same(4.0);

        style.visuals = visuals;
        ctx.set_style(style);
    }
}

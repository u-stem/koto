//! Koto DAW - Main application entry point

use anyhow::Result;
use koto_ui::KotoApp;
use tracing::{info, Level};
use tracing_subscriber::FmtSubscriber;

fn main() -> Result<()> {
    // Initialize logging
    let subscriber = FmtSubscriber::builder()
        .with_max_level(Level::INFO)
        .finish();
    tracing::subscriber::set_global_default(subscriber)?;

    info!("Starting Koto DAW");

    // Create native options
    let native_options = eframe::NativeOptions {
        viewport: egui::ViewportBuilder::default()
            .with_title("Koto DAW")
            .with_inner_size([1280.0, 800.0])
            .with_min_inner_size([800.0, 600.0]),
        renderer: eframe::Renderer::Wgpu,
        ..Default::default()
    };

    // Run the application
    eframe::run_native(
        "Koto DAW",
        native_options,
        Box::new(|cc| {
            // Apply custom fonts/style here if needed
            Ok(Box::new(KotoApp::new(cc)))
        }),
    )
    .map_err(|e| anyhow::anyhow!("Failed to run eframe: {}", e))?;

    Ok(())
}

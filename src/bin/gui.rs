//! GUI Entrypoint for the Selfish Gene Evolutionary Simulator.

use clap::Parser;
use eframe::NativeOptions;
use selfish_gene::config::Config;
use selfish_gene::gui::SelfishGeneApp;

fn main() -> eframe::Result<()> {
    // Optionally parse initial configuration from command line arguments
    let config = Config::parse();

    let native_options = NativeOptions {
        viewport: egui::ViewportBuilder::default()
            .with_title("Selfish Gene - Evolutionary Simulator")
            .with_inner_size([1200.0, 800.0])
            .with_min_inner_size([850.0, 550.0]),
        ..Default::default()
    };

    eframe::run_native(
        "Selfish Gene Simulator",
        native_options,
        Box::new(|_cc| Ok(Box::new(SelfishGeneApp::new(config)))),
    )
}

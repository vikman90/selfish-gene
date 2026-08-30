//! Graphical User Interface (GUI) module powered by egui and eframe.

pub mod app;
pub mod panels;
pub mod state;

pub use app::SelfishGeneApp;
pub use state::{RunnerState, SimulationSpeed};

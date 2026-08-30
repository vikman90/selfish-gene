//! Selfish Gene Simulation Library
//!
//! A high-performance evolutionary simulation engine modeling the principles of
//! Richard Dawkins' *The Selfish Gene* (1976).
//!
//! This crate provides the biological domain models, simulation engine,
//! population telemetry, and visualization utilities.

pub mod config;
pub mod replicator;
pub mod simulation;
pub mod stats;
pub mod visualization;

pub use config::Config;
pub use replicator::Replicator;
pub use simulation::Simulation;
pub use stats::{ConvergenceDetector, PopulationStats, SimulationHistory};
pub use visualization::LiveVisualizer;

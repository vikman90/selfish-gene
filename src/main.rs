use clap::Parser;
use selfish_gene::{Config, Simulation};
use signal_hook::consts::{SIGINT, SIGTERM};
use signal_hook::flag;
use std::sync::atomic::AtomicBool;
use std::sync::Arc;

fn main() {
    let config = Config::parse();

    // Setup signal handlers at the very beginning
    // This catches SIGINT (Ctrl+C) and SIGTERM (kill/timeout)
    let interrupted = Arc::new(AtomicBool::new(false));

    // Register both SIGINT and SIGTERM
    flag::register(SIGINT, Arc::clone(&interrupted)).expect("Failed to register SIGINT handler");
    flag::register(SIGTERM, Arc::clone(&interrupted)).expect("Failed to register SIGTERM handler");

    let mut sim = Simulation::new(config, interrupted);
    sim.run();
}

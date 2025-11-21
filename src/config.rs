use clap::Parser;
use serde::{Deserialize, Serialize};

/// Configuration for the selfish gene simulation
#[derive(Debug, Clone, Parser, Serialize, Deserialize)]
#[command(name = "selfish-gene")]
#[command(about = "Simulate evolution of replicators based on the selfish gene theory")]
pub struct Config {
    /// Maximum capacity (C) - maximum number of replicators
    #[arg(short = 'C', long, default_value = "10000")]
    pub capacity: usize,

    /// Appearance rate (lambda for Poisson distribution)
    #[arg(short = 'a', long, default_value = "100.0")]
    pub appearance_rate: f64,

    /// Initial mean survival rate
    #[arg(long, default_value = "0.7")]
    pub init_survival_mean: f64,

    /// Initial std dev for survival rate
    #[arg(long, default_value = "0.1")]
    pub init_survival_std: f64,

    /// Initial mean replication rate
    #[arg(long, default_value = "1.2")]
    pub init_replication_mean: f64,

    /// Initial std dev for replication rate
    #[arg(long, default_value = "0.2")]
    pub init_replication_std: f64,

    /// Initial mean mutation rate
    #[arg(long, default_value = "0.01")]
    pub init_mutation_mean: f64,

    /// Initial std dev for mutation rate
    #[arg(long, default_value = "0.005")]
    pub init_mutation_std: f64,

    /// Sigma (std dev) for Gaussian mutations
    #[arg(long, default_value = "0.02")]
    pub mutation_sigma: f64,

    /// Maximum number of timesteps (0 = unlimited)
    #[arg(short = 't', long, default_value = "1000")]
    pub max_timesteps: usize,

    /// Convergence threshold for trait variance (when all traits below this, converged)
    #[arg(long, default_value = "0.001")]
    pub convergence_threshold: f64,

    /// Number of timesteps to check for convergence stability
    #[arg(long, default_value = "50")]
    pub convergence_window: usize,

    /// Display statistics every N timesteps
    #[arg(long, default_value = "10")]
    pub display_interval: usize,

    /// Disable live bar chart visualization (use text log instead)
    #[arg(long)]
    pub no_live_display: bool,

    /// Number of top profiles to show in bar chart
    #[arg(long, default_value = "15")]
    pub top_profiles: usize,

    /// Export results to JSON file
    #[arg(short = 'o', long)]
    pub output_file: Option<String>,

    /// Random seed for reproducibility
    #[arg(long)]
    pub seed: Option<u64>,
}

impl Default for Config {
    fn default() -> Self {
        Config {
            capacity: 10000,
            appearance_rate: 100.0,
            init_survival_mean: 0.7,
            init_survival_std: 0.1,
            init_replication_mean: 1.2,
            init_replication_std: 0.2,
            init_mutation_mean: 0.01,
            init_mutation_std: 0.005,
            mutation_sigma: 0.02,
            max_timesteps: 1000,
            convergence_threshold: 0.001,
            convergence_window: 50,
            display_interval: 10,
            no_live_display: false,
            top_profiles: 15,
            output_file: None,
            seed: None,
        }
    }
}

impl Config {
    /// Check if live display is enabled
    pub fn live_display(&self) -> bool {
        !self.no_live_display
    }
}

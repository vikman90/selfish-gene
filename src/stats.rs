use crate::replicator::Replicator;
use serde::{Deserialize, Serialize};
use std::collections::VecDeque;

/// Statistics for a population of replicators at a single timestep
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PopulationStats {
    pub timestep: usize,
    pub population_size: usize,
    pub survival_mean: f64,
    pub survival_std: f64,
    pub replication_mean: f64,
    pub replication_std: f64,
    pub mutation_mean: f64,
    pub mutation_std: f64,
    pub age_mean: f64,
    pub age_std: f64,
}

impl PopulationStats {
    /// Calculate statistics from a population of replicators
    pub fn from_population(timestep: usize, population: &[Replicator]) -> Self {
        if population.is_empty() {
            return Self::empty(timestep);
        }

        let n = population.len() as f64;

        // Calculate means
        let survival_mean = population.iter().map(|r| r.survival_rate).sum::<f64>() / n;
        let replication_mean = population.iter().map(|r| r.replication_rate).sum::<f64>() / n;
        let mutation_mean = population.iter().map(|r| r.mutation_rate).sum::<f64>() / n;

        // Calculate standard deviations
        let survival_variance = population
            .iter()
            .map(|r| (r.survival_rate - survival_mean).powi(2))
            .sum::<f64>()
            / n;
        let replication_variance = population
            .iter()
            .map(|r| (r.replication_rate - replication_mean).powi(2))
            .sum::<f64>()
            / n;
        let mutation_variance = population
            .iter()
            .map(|r| (r.mutation_rate - mutation_mean).powi(2))
            .sum::<f64>()
            / n;

        let age_mean = population.iter().map(|r| r.age as f64).sum::<f64>() / n;
        let age_variance = population
            .iter()
            .map(|r| (r.age as f64 - age_mean).powi(2))
            .sum::<f64>()
            / n;

        Self {
            timestep,
            population_size: population.len(),
            survival_mean,
            survival_std: survival_variance.sqrt(),
            replication_mean,
            replication_std: replication_variance.sqrt(),
            mutation_mean,
            mutation_std: mutation_variance.sqrt(),
            age_mean,
            age_std: age_variance.sqrt(),
        }
    }

    fn empty(timestep: usize) -> Self {
        Self {
            timestep,
            population_size: 0,
            survival_mean: 0.0,
            survival_std: 0.0,
            replication_mean: 0.0,
            replication_std: 0.0,
            mutation_mean: 0.0,
            mutation_std: 0.0,
            age_mean: 0.0,
            age_std: 0.0,
        }
    }

    /// Get the maximum variance across all traits
    pub fn max_variance(&self) -> f64 {
        self.survival_std
            .max(self.replication_std)
            .max(self.mutation_std)
    }

    /// Pretty print for console output
    pub fn display(&self) {
        println!(
            "Timestep {}: N={}, C=?",
            self.timestep, self.population_size
        );
        println!(
            "  survival:    μ={:.4} σ={:.4}",
            self.survival_mean, self.survival_std
        );
        println!(
            "  replication: μ={:.4} σ={:.4}",
            self.replication_mean, self.replication_std
        );
        println!(
            "  mutation:    μ={:.4} σ={:.4}",
            self.mutation_mean, self.mutation_std
        );
        println!(
            "  age:         μ={:.4} σ={:.4}",
            self.age_mean, self.age_std
        );
    }
}

/// Convergence detector using a sliding window of statistics
pub struct ConvergenceDetector {
    window: VecDeque<PopulationStats>,
    window_size: usize,
    threshold: f64,
}

impl ConvergenceDetector {
    pub fn new(window_size: usize, threshold: f64) -> Self {
        Self {
            window: VecDeque::with_capacity(window_size),
            window_size,
            threshold,
        }
    }

    /// Add a new statistic to the window
    pub fn add(&mut self, stats: PopulationStats) {
        if self.window.len() >= self.window_size {
            self.window.pop_front();
        }
        self.window.push_back(stats);
    }

    /// Check if the population has converged
    /// Convergence is defined as all traits having variance below threshold
    /// for the entire window
    pub fn has_converged(&self) -> bool {
        if self.window.len() < self.window_size {
            return false;
        }

        self.window
            .iter()
            .all(|stats| stats.max_variance() < self.threshold)
    }

    /// Get the most recent statistics
    pub fn latest(&self) -> Option<&PopulationStats> {
        self.window.back()
    }
}

/// History of all statistics for export
#[derive(Debug, Serialize, Deserialize)]
pub struct SimulationHistory {
    pub stats: Vec<PopulationStats>,
    pub converged: bool,
    pub final_timestep: usize,
}

impl SimulationHistory {
    pub fn new() -> Self {
        Self {
            stats: Vec::new(),
            converged: false,
            final_timestep: 0,
        }
    }

    pub fn add(&mut self, stats: PopulationStats) {
        self.stats.push(stats);
    }

    pub fn finalize(&mut self, converged: bool, final_timestep: usize) {
        self.converged = converged;
        self.final_timestep = final_timestep;
    }

    /// Export to JSON file
    pub fn export_json(&self, filename: &str) -> std::io::Result<()> {
        let json = serde_json::to_string_pretty(self)?;
        std::fs::write(filename, json)?;
        Ok(())
    }
}

impl Default for SimulationHistory {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_population_stats_empty() {
        let stats = PopulationStats::from_population(0, &[]);
        assert_eq!(stats.population_size, 0);
        assert_eq!(stats.survival_mean, 0.0);
        assert_eq!(stats.replication_mean, 0.0);
        assert_eq!(stats.mutation_mean, 0.0);
        assert_eq!(stats.age_mean, 0.0);
    }

    #[test]
    fn test_population_stats_calculation() {
        let reps = vec![
            Replicator::new(0.6, 1.0, 0.01),
            Replicator::new(0.8, 2.0, 0.03),
        ];
        let stats = PopulationStats::from_population(1, &reps);
        assert_eq!(stats.population_size, 2);
        assert!((stats.survival_mean - 0.7).abs() < 1e-6);
        assert!((stats.replication_mean - 1.5).abs() < 1e-6);
        assert!((stats.mutation_mean - 0.02).abs() < 1e-6);
        assert!(stats.max_variance() > 0.0);
    }

    #[test]
    fn test_convergence_detector() {
        let mut detector = ConvergenceDetector::new(3, 0.05);
        assert!(!detector.has_converged());

        // Create population with low variance
        let rep1 = Replicator::new(0.8, 1.5, 0.01);
        let rep2 = Replicator::new(0.801, 1.501, 0.0101);
        let low_var_stats = PopulationStats::from_population(1, &[rep1, rep2]);

        detector.add(low_var_stats.clone());
        assert!(!detector.has_converged()); // only 1 in window of 3

        detector.add(low_var_stats.clone());
        assert!(!detector.has_converged()); // only 2 in window of 3

        detector.add(low_var_stats);
        assert!(detector.has_converged()); // window full and all below threshold
    }

    #[test]
    fn test_simulation_history_serialization() {
        let mut history = SimulationHistory::new();
        let rep = Replicator::new(0.8, 1.5, 0.01);
        history.add(PopulationStats::from_population(0, &[rep]));
        history.finalize(true, 10);

        let json = serde_json::to_string(&history).expect("Serialize failed");
        let deserialized: SimulationHistory =
            serde_json::from_str(&json).expect("Deserialize failed");

        assert_eq!(deserialized.stats.len(), 1);
        assert!(deserialized.converged);
        assert_eq!(deserialized.final_timestep, 10);
    }
}

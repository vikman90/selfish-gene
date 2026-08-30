use crate::config::Config;
use crate::replicator::Replicator;
use crate::stats::{ConvergenceDetector, PopulationStats, SimulationHistory};
use crate::visualization::LiveVisualizer;
use crossterm::event::{self, Event, KeyCode, KeyEvent, KeyModifiers};
use rand::rngs::StdRng;
use rand::{Rng, SeedableRng};
use rand_distr::{Distribution, Poisson};
use std::sync::atomic::{AtomicBool, Ordering};
use std::sync::Arc;
use std::thread;
use std::time::Duration;

/// The main simulation engine
pub struct Simulation {
    config: Config,
    population: Vec<Replicator>,
    rng: StdRng,
    timestep: usize,
    convergence_detector: ConvergenceDetector,
    history: SimulationHistory,
    interrupted: Arc<AtomicBool>,
}

impl Simulation {
    /// Create a new simulation with the given configuration
    pub fn new(config: Config, interrupted: Arc<AtomicBool>) -> Self {
        let rng = if let Some(seed) = config.seed {
            StdRng::seed_from_u64(seed)
        } else {
            StdRng::from_entropy()
        };

        let convergence_detector =
            ConvergenceDetector::new(config.convergence_window, config.convergence_threshold);

        Self {
            config,
            population: Vec::new(),
            rng,
            timestep: 0,
            convergence_detector,
            history: SimulationHistory::new(),
            interrupted,
        }
    }

    /// Create a new standalone simulation with default interrupt handler (useful for GUI/tests)
    pub fn with_config(config: Config) -> Self {
        let mut sim = Self::new(config, Arc::new(AtomicBool::new(false)));
        sim.initialize_population();
        let stats = sim.current_stats();
        sim.convergence_detector.add(stats.clone());
        sim.history.add(stats);
        sim
    }

    /// Reset the simulation with a new or updated configuration
    pub fn reset(&mut self, config: Config) {
        let rng = if let Some(seed) = config.seed {
            StdRng::seed_from_u64(seed)
        } else {
            StdRng::from_entropy()
        };

        self.convergence_detector =
            ConvergenceDetector::new(config.convergence_window, config.convergence_threshold);
        self.config = config;
        self.population.clear();
        self.rng = rng;
        self.timestep = 0;
        self.history = SimulationHistory::new();
        self.interrupted.store(false, Ordering::SeqCst);
        self.initialize_population();
        let stats = self.current_stats();
        self.convergence_detector.add(stats.clone());
        self.history.add(stats);
    }

    /// Initialize the population with new replicators from appearance rate
    pub fn initialize_population(&mut self) {
        if self.config.appearance_rate <= 0.0 {
            self.population.clear();
            return;
        }

        let poisson = Poisson::new(self.config.appearance_rate).unwrap();
        let initial_count = poisson.sample(&mut self.rng) as usize;

        self.population = (0..initial_count)
            .map(|_| {
                Replicator::from_distributions(
                    &mut self.rng,
                    self.config.init_survival_mean,
                    self.config.init_survival_std,
                    self.config.init_replication_mean,
                    self.config.init_replication_std,
                    self.config.init_mutation_mean,
                    self.config.init_mutation_std,
                )
            })
            .collect();
    }

    /// Calculate the resource factor based on current population
    pub fn resource_factor(&self) -> f64 {
        let n = self.population.len();
        let c = self.config.capacity;
        (1.0 - (n as f64 / c as f64)).max(0.0)
    }

    /// Run one timestep of the simulation
    pub fn step(&mut self) {
        if self.is_finished() {
            return;
        }

        // Step 1: New appearances (only if population is not at capacity)
        if self.population.len() < self.config.capacity && self.config.appearance_rate > 0.0 {
            let poisson = Poisson::new(self.config.appearance_rate).unwrap();
            let new_count = poisson.sample(&mut self.rng) as usize;

            for _ in 0..new_count {
                if self.population.len() >= self.config.capacity {
                    break;
                }
                self.population.push(Replicator::from_distributions(
                    &mut self.rng,
                    self.config.init_survival_mean,
                    self.config.init_survival_std,
                    self.config.init_replication_mean,
                    self.config.init_replication_std,
                    self.config.init_mutation_mean,
                    self.config.init_mutation_std,
                ));
            }
        }

        // Step 2: Survival - filter out those who don't survive (age-dependent)
        // We generate random numbers once to preserve RNG sequence behavior
        let sen = self.config.senescence_rate;
        let survival_rolls: Vec<f64> = (0..self.population.len())
            .map(|_| self.rng.gen::<f64>())
            .collect();

        self.population = self
            .population
            .iter()
            .zip(survival_rolls.iter())
            .filter(|(rep, roll)| {
                let age_f = rep.age as f64;
                // Exponential decay with age: exp(-sen * age)
                let decay = (-sen * age_f).exp();
                let effective = (rep.survival_rate * decay).clamp(0.0, 1.0);
                **roll < effective
            })
            .map(|(rep, _)| rep.clone())
            .collect();

        if !self.population.is_empty() {
            // Step 3: Replication
            let resource_factor = self.resource_factor();
            let mut offspring = Vec::new();

            for replicator in &self.population {
                let offspring_count = {
                    let effective_rate = replicator.replication_rate * resource_factor;
                    let base = effective_rate.floor() as usize;
                    let fractional = effective_rate - effective_rate.floor();
                    if self.rng.gen::<f64>() < fractional {
                        base + 1
                    } else {
                        base
                    }
                };

                for _ in 0..offspring_count {
                    if self.population.len() + offspring.len() >= self.config.capacity {
                        break;
                    }

                    let child = if self.rng.gen::<f64>() < replicator.mutation_rate {
                        replicator.create_offspring(&mut self.rng, self.config.mutation_sigma)
                    } else {
                        replicator.clone()
                    };
                    offspring.push(child);
                }

                if self.population.len() + offspring.len() >= self.config.capacity {
                    break;
                }
            }

            self.population.extend(offspring);

            // Step 4: Age increment - all individuals age by 1 each timestep
            for rep in &mut self.population {
                rep.age = rep.age.saturating_add(1);
            }
        }

        self.timestep += 1;
        let stats = self.current_stats();
        self.convergence_detector.add(stats.clone());
        self.history.add(stats);

        if self.convergence_detector.has_converged() {
            self.history.finalize(true, self.timestep);
        }
    }

    /// Run the full simulation
    pub fn run(&mut self) {
        let visualizer = LiveVisualizer::new(self.config.top_profiles, self.config.live_display());

        if !self.config.live_display() {
            println!("Starting simulation with parameters:");
            println!("  Capacity: {}", self.config.capacity);
            println!("  Appearance rate: {}", self.config.appearance_rate);
            println!("  Max timesteps: {}", self.config.max_timesteps);
            println!(
                "  Convergence threshold: {}",
                self.config.convergence_threshold
            );
            println!();
        }

        // Initialize if empty
        if self.timestep == 0 && self.history.stats.is_empty() {
            self.initialize_population();
            let stats = self.current_stats();
            self.convergence_detector.add(stats.clone());
            self.history.add(stats);
        }

        // Main simulation loop
        while !self.is_finished() {
            // Display
            if self.timestep.is_multiple_of(self.config.display_interval) {
                if self.config.live_display() {
                    visualizer.display(self.timestep, &self.population, self.config.capacity);
                    // Small delay to make visualization visible
                    thread::sleep(Duration::from_millis(50));
                } else if let Some(stats) = self.history.stats.last() {
                    stats.display();
                    println!();
                }
            }

            // Check for keyboard input (Ctrl+C) when in live display mode
            if self.config.live_display()
                && event::poll(Duration::from_millis(0)).unwrap_or(false)
            {
                if let Ok(Event::Key(KeyEvent {
                    code: KeyCode::Char('c'),
                    modifiers: KeyModifiers::CONTROL,
                    ..
                })) = event::read()
                {
                    self.interrupted.store(true, Ordering::SeqCst);
                }
            }

            self.step();
        }

        // Final status message
        if !self.config.live_display() {
            if self.is_interrupted() {
                println!("\nInterrupted by user at timestep {}", self.timestep);
            } else if self.has_converged() {
                println!("Convergence detected at timestep {}", self.timestep);
            } else if self.config.max_timesteps > 0 && self.timestep >= self.config.max_timesteps {
                println!("Reached maximum timesteps: {}", self.config.max_timesteps);
            }
        }

        // Final statistics
        if self.config.live_display() {
            visualizer.display_final(&self.population);
        } else {
            println!("\n=== Final Statistics ===");
            if let Some(final_stats) = self.convergence_detector.latest() {
                final_stats.display();

                if let Some(winner) = self.winner_profile() {
                    println!("\n=== Winner Profile ===");
                    println!("  survival_rate:    {:.6}", winner.survival_rate);
                    println!("  replication_rate: {:.6}", winner.replication_rate);
                    println!("  mutation_rate:    {:.6}", winner.mutation_rate);
                }
            }
        }

        // Export if requested
        if let Some(ref output_file) = self.config.output_file {
            match self.history.export_json(output_file) {
                Ok(_) => println!("\nResults exported to: {}", output_file),
                Err(e) => eprintln!("\nError exporting results: {}", e),
            }
        }
    }

    /// Get a reference to the active population
    pub fn population(&self) -> &[Replicator] {
        &self.population
    }

    /// Get current configuration
    pub fn config(&self) -> &Config {
        &self.config
    }

    /// Get current timestep
    pub fn timestep(&self) -> usize {
        self.timestep
    }

    /// Calculate and return statistics for the current population state
    pub fn current_stats(&self) -> PopulationStats {
        PopulationStats::from_population(self.timestep, &self.population)
    }

    /// Get simulation history
    pub fn history(&self) -> &SimulationHistory {
        &self.history
    }

    /// Get convergence detector
    pub fn convergence_detector(&self) -> &ConvergenceDetector {
        &self.convergence_detector
    }

    /// Check if convergence criteria have been met
    pub fn has_converged(&self) -> bool {
        self.convergence_detector.has_converged()
    }

    /// Check if simulation has reached termination conditions (max steps, convergence, or interruption)
    pub fn is_finished(&self) -> bool {
        (self.config.max_timesteps > 0 && self.timestep >= self.config.max_timesteps)
            || self.has_converged()
            || self.interrupted.load(Ordering::SeqCst)
    }

    /// Get winner replicator profile (closest to mean traits) if population is non-empty
    pub fn winner_profile(&self) -> Option<Replicator> {
        let stats = self
            .convergence_detector
            .latest()
            .cloned()
            .unwrap_or_else(|| self.current_stats());

        if self.population.is_empty() {
            return None;
        }

        self.population
            .iter()
            .min_by(|a, b| {
                let a_dist = (a.survival_rate - stats.survival_mean).abs()
                    + (a.replication_rate - stats.replication_mean).abs()
                    + (a.mutation_rate - stats.mutation_mean).abs();
                let b_dist = (b.survival_rate - stats.survival_mean).abs()
                    + (b.replication_rate - stats.replication_mean).abs()
                    + (b.mutation_rate - stats.mutation_mean).abs();
                a_dist.partial_cmp(&b_dist).unwrap()
            })
            .cloned()
    }

    /// Request interruption of the simulation
    pub fn interrupt(&self) {
        self.interrupted.store(true, Ordering::SeqCst);
    }

    /// Check if simulation was interrupted
    pub fn is_interrupted(&self) -> bool {
        self.interrupted.load(Ordering::SeqCst)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_simulation_initialization() {
        let config = Config {
            appearance_rate: 50.0,
            seed: Some(12345),
            ..Default::default()
        };

        let sim = Simulation::with_config(config);
        assert!(!sim.population().is_empty());
        assert_eq!(sim.timestep(), 0);
        assert!(!sim.is_finished());
    }

    #[test]
    fn test_resource_factor() {
        let config = Config {
            capacity: 100,
            ..Default::default()
        };
        let mut sim = Simulation::with_config(config);

        // Clear population and test factor
        sim.population.clear();
        assert_eq!(sim.resource_factor(), 1.0);

        // Fill population to capacity
        for _ in 0..100 {
            sim.population.push(Replicator::new(0.8, 1.2, 0.01));
        }
        assert_eq!(sim.resource_factor(), 0.0);
    }

    #[test]
    fn test_simulation_stepping_and_aging() {
        let config = Config {
            capacity: 200,
            appearance_rate: 20.0,
            seed: Some(42),
            ..Default::default()
        };

        let mut sim = Simulation::with_config(config);
        let initial_pop = sim.population().len();
        assert!(initial_pop > 0);

        sim.step();
        assert_eq!(sim.timestep(), 1);

        // Surviving individuals from previous generation should have aged
        let has_aged = sim.population().iter().any(|r| r.age >= 1);
        assert!(has_aged);
    }

    #[test]
    fn test_simulation_determinism() {
        let config1 = Config {
            seed: Some(987654),
            max_timesteps: 15,
            capacity: 500,
            ..Default::default()
        };

        let config2 = config1.clone();

        let mut sim1 = Simulation::with_config(config1);
        let mut sim2 = Simulation::with_config(config2);

        assert_eq!(sim1.population().len(), sim2.population().len());

        for _ in 0..15 {
            sim1.step();
            sim2.step();
        }

        assert_eq!(sim1.population().len(), sim2.population().len());
        let stats1 = sim1.current_stats();
        let stats2 = sim2.current_stats();

        assert_eq!(stats1.survival_mean, stats2.survival_mean);
        assert_eq!(stats1.replication_mean, stats2.replication_mean);
        assert_eq!(stats1.mutation_mean, stats2.mutation_mean);
    }

    #[test]
    fn test_senescence_reduces_survival() {
        // Run with high senescence vs zero senescence
        let config_no_sen = Config {
            seed: Some(42),
            senescence_rate: 0.0,
            appearance_rate: 0.0, // no new arrivals
            capacity: 100,
            ..Default::default()
        };

        let mut config_high_sen = config_no_sen.clone();
        config_high_sen.senescence_rate = 0.5; // very rapid aging death

        let mut sim_no_sen = Simulation::with_config(config_no_sen);
        let mut sim_high_sen = Simulation::with_config(config_high_sen);

        // Seed with identical aged individuals
        let base_reps: Vec<Replicator> = (0..50)
            .map(|_| {
                let mut r = Replicator::new(0.9, 0.0, 0.0); // no reproduction, high base survival
                r.age = 10;
                r
            })
            .collect();

        sim_no_sen.population = base_reps.clone();
        sim_high_sen.population = base_reps;

        sim_no_sen.step();
        sim_high_sen.step();

        // High senescence should have fewer survivors among old individuals
        assert!(sim_high_sen.population().len() < sim_no_sen.population().len());
    }

    #[test]
    fn test_simulation_reset() {
        let config = Config {
            seed: Some(111),
            capacity: 50,
            ..Default::default()
        };

        let mut sim = Simulation::with_config(config.clone());
        for _ in 0..5 {
            sim.step();
        }

        sim.reset(config);
        assert_eq!(sim.timestep(), 0);
        assert_eq!(sim.history().stats.len(), 1);
        assert!(!sim.population().is_empty());
    }

    #[test]
    fn test_winner_profile() {
        let config = Config {
            seed: Some(777),
            ..Default::default()
        };
        let sim = Simulation::with_config(config);
        let winner = sim.winner_profile();
        assert!(winner.is_some());
    }
}


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

    /// Initialize the population with new replicators from appearance rate
    fn initialize_population(&mut self) {
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
    fn resource_factor(&self) -> f64 {
        let n = self.population.len();
        let c = self.config.capacity;
        (1.0 - (n as f64 / c as f64)).max(0.0)
    }

    /// Run one timestep of the simulation
    fn step(&mut self) {
        // Step 1: New appearances (only if population is not at capacity)
        if self.population.len() < self.config.capacity {
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

        // Step 2: Survival - filter out those who don't survive
        // We need to use a single RNG, so we'll generate random numbers first
        let survival_rolls: Vec<f64> = (0..self.population.len())
            .map(|_| self.rng.gen::<f64>())
            .collect();

        self.population = self
            .population
            .iter()
            .zip(survival_rolls.iter())
            .filter(|(rep, roll)| **roll < rep.survival_rate)
            .map(|(rep, _)| rep.clone())
            .collect();

        if self.population.is_empty() {
            return;
        }

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

        // Initialize
        self.initialize_population();

        // Main simulation loop
        loop {
            // Calculate and record statistics
            let stats = PopulationStats::from_population(self.timestep, &self.population);

            // Display
            if self.timestep % self.config.display_interval == 0 {
                if self.config.live_display() {
                    visualizer.display(self.timestep, &self.population, self.config.capacity);
                    // Small delay to make visualization visible
                    thread::sleep(Duration::from_millis(50));
                } else {
                    stats.display();
                    println!();
                }
            }

            // Check for keyboard input (Ctrl+C) when in live display mode
            if self.config.live_display() {
                if event::poll(Duration::from_millis(0)).unwrap_or(false) {
                    if let Ok(Event::Key(KeyEvent {
                        code: KeyCode::Char('c'),
                        modifiers: KeyModifiers::CONTROL,
                        ..
                    })) = event::read()
                    {
                        self.interrupted.store(true, Ordering::SeqCst);
                    }
                }
            }

            // Record statistics
            self.convergence_detector.add(stats.clone());
            self.history.add(stats);

            // Check for interruption (Ctrl+C)
            if self.interrupted.load(Ordering::SeqCst) {
                if !self.config.live_display() {
                    println!("\nInterrupted by user at timestep {}", self.timestep);
                }
                break;
            }

            // Check termination conditions
            if self.population.is_empty() {
                if !self.config.live_display() {
                    println!("Population extinct at timestep {}", self.timestep);
                }
                break;
            }

            if self.config.max_timesteps > 0 && self.timestep >= self.config.max_timesteps {
                if !self.config.live_display() {
                    println!("Reached maximum timesteps: {}", self.config.max_timesteps);
                }
                break;
            }

            if self.convergence_detector.has_converged() {
                if !self.config.live_display() {
                    println!("Convergence detected at timestep {}", self.timestep);
                }
                self.history.finalize(true, self.timestep);
                break;
            }

            // Next timestep
            self.timestep += 1;
            self.step();
        }

        // Final statistics
        if self.config.live_display() {
            visualizer.display_final(&self.population);
        } else {
            println!("\n=== Final Statistics ===");
            if let Some(final_stats) = self.convergence_detector.latest() {
                final_stats.display();

                if !self.population.is_empty() {
                    println!("\n=== Winner Profile ===");
                    // Find the "winner" as the one closest to the mean
                    let winner = self
                        .population
                        .iter()
                        .min_by(|a, b| {
                            let a_dist = (a.survival_rate - final_stats.survival_mean).abs()
                                + (a.replication_rate - final_stats.replication_mean).abs()
                                + (a.mutation_rate - final_stats.mutation_mean).abs();
                            let b_dist = (b.survival_rate - final_stats.survival_mean).abs()
                                + (b.replication_rate - final_stats.replication_mean).abs()
                                + (b.mutation_rate - final_stats.mutation_mean).abs();
                            a_dist.partial_cmp(&b_dist).unwrap()
                        })
                        .unwrap();

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
}

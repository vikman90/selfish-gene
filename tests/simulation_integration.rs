use selfish_gene::{Config, Simulation};

#[test]
fn test_integration_full_simulation_convergence_or_limit() {
    let config = Config {
        seed: Some(42),
        max_timesteps: 50,
        capacity: 1000,
        appearance_rate: 50.0,
        convergence_threshold: 0.05,
        convergence_window: 10,
        no_live_display: true,
        ..Default::default()
    };

    let mut sim = Simulation::with_config(config);
    assert_eq!(sim.timestep(), 0);

    while !sim.is_finished() {
        sim.step();
    }

    assert!(sim.timestep() <= 50);
    assert!(!sim.population().is_empty());
    let stats = sim.current_stats();
    assert!(stats.survival_mean >= 0.0 && stats.survival_mean <= 1.0);
    assert!(stats.replication_mean >= 0.0);
    assert!(stats.mutation_mean >= 0.0 && stats.mutation_mean <= 1.0);
    assert!(stats.aggression_mean >= 0.0 && stats.aggression_mean <= 1.0);
    assert!(sim.winner_profile().is_some());
}

#[test]
fn test_integration_hawk_dove_ess_evolution() {
    // V = 2.0, C = 10.0 => Theoretical ESS p* = V/C = 0.20
    let config = Config {
        seed: Some(42),
        max_timesteps: 150,
        capacity: 1000,
        appearance_rate: 20.0,
        enable_game_theory: true,
        game_resource_value: 2.0,
        game_injury_cost: 10.0,
        game_interaction_rate: 2.0,
        init_aggression_mean: 0.8, // Start with high aggression
        init_aggression_std: 0.1,
        no_live_display: true,
        ..Default::default()
    };

    let mut sim = Simulation::with_config(config);

    while !sim.is_finished() {
        sim.step();
    }

    let stats = sim.current_stats();
    // High initial aggression (0.8) should evolve downward toward the ESS ratio (0.20)
    assert!(stats.aggression_mean < 0.65);
}

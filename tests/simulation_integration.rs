use selfish_gene::{Config, Simulation};

#[test]
fn test_integration_full_simulation_convergence_or_limit() {
    let mut config = Config::default();
    config.seed = Some(42);
    config.max_timesteps = 50;
    config.capacity = 1000;
    config.appearance_rate = 50.0;
    config.convergence_threshold = 0.05;
    config.convergence_window = 10;
    config.no_live_display = true;

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
    assert!(sim.winner_profile().is_some());
}

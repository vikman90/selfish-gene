# Developer & AI Extension Guide (`extending.md`)

This guide provides practical protocols, patterns, and checklists for developers and AI assistants extending the **Selfish Gene Simulator**.

---

## 1. Extension Recipes

### Recipe 1: Adding a New Heritable Trait

To introduce a new continuous trait (e.g., `altruism_tendency`, `metabolic_cost`, or `maturation_age`):

#### Step 1: Update `src/replicator.rs`
1. Add the field to struct `Replicator`:
   ```rust
   pub struct Replicator {
       pub survival_rate: f64,
       pub replication_rate: f64,
       pub mutation_rate: f64,
       pub altruism: f64, // NEW trait [0.0, 1.0]
       pub age: usize,
   }
   ```
2. Update `Replicator::new(...)` and apply bounds clamping (`.max(0.0).min(1.0)`).
3. Update `Replicator::from_distributions(...)` to sample `altruism` from a `Normal` distribution.
4. Update `Replicator::create_offspring(...)` to apply Gaussian perturbation $\mathcal{N}(0, \sigma_m)$ to the new trait.

#### Step 2: Update `src/config.rs`
1. Add initial distribution arguments to `Config`:
   ```rust
   /// Initial mean altruism tendency
   #[arg(long, default_value = "0.1")]
   pub init_altruism_mean: f64,

   /// Initial std dev for altruism tendency
   #[arg(long, default_value = "0.05")]
   pub init_altruism_std: f64,
   ```
2. Update `Default for Config`.

#### Step 3: Update `src/stats.rs`
1. Add `altruism_mean` and `altruism_std` to `PopulationStats`.
2. Compute the mean and standard deviation in `PopulationStats::from_population()`.
3. Include `self.altruism_std` in `ConvergenceDetector`'s `max_variance()` calculation.

#### Step 4: Update `src/visualization.rs`
1. Extend `ProfileBin` with a discretized bin for the new trait if desired.
2. Update `display_label()` to format the new trait in the live bar chart.

#### Step 5: Update Documentation
1. Add parameter descriptions to [`docs/parameters.md`](file:///home/vikman/Projects/selfish-gene/docs/parameters.md).
2. Document the biological rationale in [`docs/theory.md`](file:///home/vikman/Projects/selfish-gene/docs/theory.md).

---

### Recipe 2: Adding Social Interactions & Game Theory (Dawkins Ch. 5)

To implement pairwise interactions (e.g., Hawk-Dove, Iterated Prisoner's Dilemma):

1. **Insert Interaction Phase:** In `Simulation::step()` in [`src/simulation.rs`](file:///home/vikman/Projects/selfish-gene/src/simulation.rs), insert a step between *Survival* and *Replication*:
   ```rust
   // Step 2.5: Social & Evolutionary Game Interactions
   self.resolve_interactions();
   ```
2. **Pairing Mechanics:**
   - Shuffle or sample pairs of individuals uniformly at random.
   - Calculate payoffs according to a payoff matrix (e.g., Hawk-Dove with resource reward $V$ and injury cost $C$).
   - Convert payoffs into individual reproductive bonuses or survival modifiers.

---

### Recipe 3: Dynamic & Seasonal Carrying Capacity

To model seasonal cycles, climate oscillations, or environmental catastrophes:

1. **Parameterize Time-Dependent Capacity:**
   In [`src/simulation.rs`](file:///home/vikman/Projects/selfish-gene/src/simulation.rs), generalize `resource_factor()`:
   ```rust
   fn dynamic_capacity(&self) -> f64 {
       let c0 = self.config.capacity as f64;
       // Example seasonal cycle: C(t) = C_0 * (1.0 + 0.3 * sin(2 * PI * t / period))
       c0
   }
   
   fn resource_factor(&self) -> f64 {
       let n = self.population.len() as f64;
       let c = self.dynamic_capacity();
       (1.0 - (n / c)).max(0.0)
   }
   ```

---

### Recipe 4: Loading Scenarios from TOML / YAML

Because `Config` derives `serde::Deserialize`, scenario files can be supported by adding a helper in [`src/config.rs`](file:///home/vikman/Projects/selfish-gene/src/config.rs):

```rust
impl Config {
    pub fn load_from_file(path: &str) -> Result<Self, Box<dyn std::error::Error>> {
        let content = std::fs::read_to_string(path)?;
        let config: Config = toml::from_str(&content)?;
        Ok(config)
    }
}
```

---

### Recipe 5: Exposing New Traits and Visualizations in the GUI

When adding new biological traits (e.g. altruism $A$, dispersal $D$) or environmental mechanisms:

1. **Add parameters to `Config`** in [`src/config.rs`](file:///home/vikman/Projects/selfish-gene/src/config.rs).
2. **Add interactive UI widget** in [`src/gui/panels/params.rs`](file:///home/vikman/Projects/selfish-gene/src/gui/panels/params.rs) within the corresponding collapsible header:
   ```rust
   ui.label("Tasa de Altruismo inicial (A):");
   ui.add(Slider::new(&mut config.init_altruism, 0.0..=1.0).step_by(0.01))
       .on_hover_text("Inversión de aptitud para beneficiar a receptores (Hamilton's Rule).");
   ```
3. **Expose time-series plot line** in [`src/gui/panels/time_series.rs`](file:///home/vikman/Projects/selfish-gene/src/gui/panels/time_series.rs):
   ```rust
   let altruism_points: PlotPoints = history
       .stats
       .iter()
       .map(|s| [s.timestep as f64, s.altruism_mean])
       .collect();
   plot_ui.line(Line::new(altruism_points).name("Altruismo μ(A)"));
   ```
4. **Update documentation** in [`docs/parameters.md`](file:///home/vikman/Projects/selfish-gene/docs/parameters.md) and [`docs/theory.md`](file:///home/vikman/Projects/selfish-gene/docs/theory.md).

---

## 2. Quality & Verification Checklist

Before submitting changes, always execute the following verification steps:

- [ ] **Compile & Unit Tests:** `cargo test` passes with zero errors.
- [ ] **Formatting:** `cargo fmt --check` passes.
- [ ] **Lints:** `cargo clippy --all-targets -- -D warnings` runs cleanly.
- [ ] **Binary Builds:** Both `cargo build --bin selfish-gene` and `cargo build --bin selfish-gene-gui` compile without warnings.
- [ ] **Documentation Sync:** All modified parameters are reflected in [`docs/parameters.md`](file:///home/vikman/Projects/selfish-gene/docs/parameters.md).
- [ ] **Theoretical Alignment:** Concepts adhere to the principles in [`docs/theory.md`](file:///home/vikman/Projects/selfish-gene/docs/theory.md).


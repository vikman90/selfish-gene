# Selfish Gene Simulator 🧬

![CI](https://github.com/vikman90/selfish-gene/actions/workflows/ci.yml/badge.svg)

A high-performance evolution simulator based on Richard Dawkins' **Selfish Gene** theory, implemented in Rust with both a terminal CLI and a native cross-platform Graphical User Interface (GUI).

## Features

- **Evolutionary simulation engine**: Core biological models (survival, replication, mutation, age senescence).
- **Interactive GUI Application (`selfish-gene-gui`)**:
  - Real-time time series plots (`egui_plot`) for population $N(t)$ vs $C$, trait means $(\bar{S}, \bar{R}, \bar{M})$, and age.
  - Interactive parameter controls: sliders for capacity, appearance rate, initial trait distributions, mutation noise, and convergence.
  - Genotype / phenotype distribution histogram with abundance percentages.
  - Playback controls: Play, Pause, Step (+1 generation), Reset, and simulation speed throttle (steps per frame).
  - Ecosystem telemetry dashboard and one-click JSON export.
- **Terminal CLI Application (`selfish-gene`)**:
  - Real-time ASCII bar chart TUI or clean text log mode.
  - Signal handling: graceful termination with Ctrl+C, SIGINT, and SIGTERM.
- **Strict Reproducibility**: Exact determinism via PRNG `--seed`.
- **Convergence Detection**: Automatic equilibrium / ESS detection via sliding variance windows.

## Installation & Running

### Requirements

- Rust 1.75+ (with Cargo)

### Running the Graphical User Interface (GUI)

```bash
# Launch interactive GUI
cargo run --bin selfish-gene-gui --release
```

### Running the Command Line Interface (CLI)

```bash
# Run with default parameters and live terminal TUI
cargo run --bin selfish-gene --release

# Run for 500 timesteps in text log mode
cargo run --bin selfish-gene -- --max-timesteps 500 --no-live-display

# Export results to JSON
cargo run --bin selfish-gene -- -o results.json
```

### Configuration options

```bash
# Simulation parameters
-C, --capacity <N>                    Maximum population capacity [default: 10000]
-a, --appearance-rate <RATE>          New replicators per timestep (Poisson λ) [default: 100]
-t, --max-timesteps <N>               Maximum timesteps (0 = unlimited) [default: 1000]
--senescence-rate <RATE>              Senescence rate (age-dependent mortality factor, default 0 = disabled)

# Initial trait distributions (mean and std dev)
--init-survival-mean <MEAN>           [default: 0.7]
--init-survival-std <STD>             [default: 0.1]
--init-replication-mean <MEAN>        [default: 1.2]
--init-replication-std <STD>          [default: 0.2]
--init-mutation-mean <MEAN>           [default: 0.01]
--init-mutation-std <STD>             [default: 0.005]

# Evolution parameters
--mutation-sigma <SIGMA>              Gaussian noise for mutations [default: 0.02]

# Convergence settings
--convergence-threshold <THRESHOLD>   Trait variance threshold [default: 0.001]
--convergence-window <WINDOW>         Timesteps to check for stability [default: 50]

# Display options
--display-interval <N>                Show stats every N timesteps [default: 10]
--no-live-display                     Use text log instead of live visualization
--top-profiles <N>                    Number of profiles in bar chart [default: 15]

# Output
-o, --output-file <FILE>              Export results to JSON

# Reproducibility
```

## Documentation

Comprehensive project documentation is available in the [`docs/`](docs/) directory:

- 📖 **[Theoretical Foundations (`docs/theory.md`)](docs/theory.md)**: Richard Dawkins' selfish gene principles, population genetics, and mathematical formulations.
- ⚙️ **[Parameter Reference (`docs/parameters.md`)](docs/parameters.md)**: Full CLI reference, mathematical formulas, parameter domains, and scenario presets.
- 🏗️ **[System Architecture (`docs/architecture.md`)](docs/architecture.md)**: Codebase modules, execution lifecycle flow, and data pipelines.
- 🚀 **[Extension Guide (`docs/extending.md`)](docs/extending.md)**: Protocols for adding new traits, environmental dynamics, and game theory interactions.
- 🤖 **[AI Guidelines (`AGENTS.md`)](AGENTS.md)**: Project compass and consistency rules for AI programming assistants.

## How it works

### The Model

Each **replicator** has three heritable traits:

1. **Survival rate** (0-1): Probability of surviving each timestep
2. **Replication rate** (≥0): Expected number of offspring per timestep
3. **Mutation rate** (0-1): Probability of trait mutation during replication

### Simulation cycle

Each timestep:

1. **Appearance**: New replicators appear (Poisson distribution)
2. **Survival**: Each replicator survives with probability = survival_rate
   - *Age (senescence):* The survival probability is reduced with age following an exponential decay: `effective_survival = survival_rate * exp(-senescence_rate * age)`.
3. **Replication**: Survivors produce offspring
   - Actual rate = replication_rate × resource_factor
   - Resource factor = `1 - (N / C)` (density-dependent)
4. **Mutation**: Offspring traits may mutate (Gaussian noise)

### Evolutionary pressure

The resource factor creates selection pressure:
- When population is low (N << C), replication is near maximum
- When approaching capacity (N → C), replication slows down
- Traits that balance survival and efficient replication tend to dominate

Senescence introduces an additional evolutionary pressure: higher senescence rates penalize older individuals, favoring traits that reproduce earlier or that maintain higher baseline survival at younger ages.

## Example output

### Live visualization mode (default)

```
╔════════════════════════════════════════════════════════════════════════════╗
║ Timestep:    100  |  Population: 10000 / 10000                             ║
╠════════════════════════════════════════════════════════════════════════════╣
║ Profile Distribution (Top 15 profiles)                                     ║
╠════════════════════════════════════════════════════════════════════════════╣
║ S:1.0 R:1.1 M:0.01 │ ███████████████████████████████████████████████  9904 ║
║ S:1.0 R:1.3 M:0.01 │ ██                                                 48 ║
║ S:0.9 R:1.4 M:0.01 │                                                    29 ║
║ S:1.0 R:1.0 M:0.01 │                                                     6 ║
╚════════════════════════════════════════════════════════════════════════════╝

Legend: S=Survival rate, R=Replication rate, M=Mutation rate
```

### Final statistics

```
════════════════════════════════════════════════════════════════
                    FINAL POPULATION
════════════════════════════════════════════════════════════════
Final population size:    10000
Average survival rate:    1.000000
Average replication rate: 1.119440
Average mutation rate:    0.007221
Average age:              12.0334
════════════════════════════════════════════════════════════════
```

## Observations

Typical evolutionary patterns observed:

- **Survival rate → 1.0**: Strong selection for maximum survival
- **Replication rate → 1.1-1.4**: Optimal balance with resource constraints
- **Mutation rate → ~0.01**: Maintains variability without destabilizing
- Initial diversity converges to 1-3 dominant profiles

## Testing

Run unit tests:

```bash
cargo test
```

## Performance

Built with Rust for maximum performance:
- Handles 10,000+ replicators efficiently
- Real-time visualization updates
- Minimal memory footprint

## License

MIT License - see [LICENSE](LICENSE) file for details.

## Contributing

Contributions are welcome! Please feel free to submit a Pull Request.

## References

- Dawkins, R. (1976). *The Selfish Gene*. Oxford University Press.

## Author

Vikman Fernandez-Castro

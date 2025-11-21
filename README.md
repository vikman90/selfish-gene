# Selfish Gene Simulator 🧬

A high-performance evolution simulator based on Richard Dawkins' **Selfish Gene** theory, implemented in Rust with real-time ASCII visualization.

## Features

- **Evolutionary simulation** with configurable traits (survival, replication, mutation)
- **Resource pressure model**: reproduction scales with population density `1 - (N/C)`
- **Real-time ASCII visualization**: live bar chart showing population distribution
- **Convergence detection**: automatic stopping when trait variance stabilizes
- **Signal handling**: graceful termination with Ctrl+C, SIGINT, and SIGTERM
- **Flexible output**: live display or text log mode
- **Data export**: JSON format for further analysis
- **Fully configurable**: all parameters adjustable via CLI

## Installation

### Requirements

- Rust 1.70+ (with Cargo)

### Build from source

```bash
git clone https://github.com/yourusername/selfish-gene.git
cd selfish-gene
cargo build --release
```

The binary will be available at `./target/release/selfish-gene`

## Usage

### Quick start

```bash
# Run with default parameters and live visualization
./target/release/selfish-gene

# Run for 500 timesteps
./target/release/selfish-gene --max-timesteps 500

# Use text log mode instead of live display
./target/release/selfish-gene --no-live-display

# Export results to JSON
./target/release/selfish-gene -o results.json
```

### Configuration options

```bash
# Simulation parameters
-C, --capacity <N>                    Maximum population capacity [default: 10000]
-a, --appearance-rate <RATE>          New replicators per timestep (Poisson λ) [default: 100]
-t, --max-timesteps <N>               Maximum timesteps (0 = unlimited) [default: 1000]

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
--seed <SEED>                         Random seed for reproducible runs
```

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
3. **Replication**: Survivors produce offspring
   - Actual rate = replication_rate × resource_factor
   - Resource factor = `1 - (N / C)` (density-dependent)
4. **Mutation**: Offspring traits may mutate (Gaussian noise)

### Evolutionary pressure

The resource factor creates selection pressure:
- When population is low (N << C), replication is near maximum
- When approaching capacity (N → C), replication slows down
- Traits that balance survival and efficient replication tend to dominate

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

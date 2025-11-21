# Changelog

All notable changes to this project will be documented in this file.

The format is based on [Keep a Changelog](https://keepachangelog.com/en/1.0.0/),
and this project adheres to [Semantic Versioning](https://semver.org/spec/v2.0.0.html).

## [0.1.0] - 2025-01-18

### Added
- Core simulation engine with replicator evolution
- Three heritable traits: survival rate, replication rate, and mutation rate
- Resource pressure model with density-dependent reproduction
- Poisson distribution for new replicator appearance
- Gaussian noise for trait mutations
- Live ASCII bar chart visualization with real-time updates
- Text log mode for non-interactive environments
- Convergence detection based on trait variance
- Signal handling: graceful termination with Ctrl+C, SIGINT, and SIGTERM
- Full CLI with clap for parameter configuration
- JSON export for simulation history and analysis
- Unit tests for core replicator logic
- Comprehensive README with usage examples
- MIT License

### Features
- Configurable simulation parameters (capacity, appearance rate, initial distributions)
- Adjustable mutation parameters (sigma for Gaussian noise)
- Convergence detection with configurable threshold and window
- Display customization (interval, top N profiles, live/text mode)
- Reproducible runs with random seed option

[0.1.0]: https://github.com/yourusername/selfish-gene/releases/tag/v0.1.0

# AI Assistant Guidelines & Project Compass (`AGENTS.md`)

This repository contains a high-performance evolutionary simulation written in Rust, modeling the core principles of **Richard Dawkins' *The Selfish Gene* (1976)**.

This document serves as the project compass and operational rulebook for AI assistants (such as Antigravity).

---

## 🧭 Project Mental Model & Architecture

The simulator models evolution from the gene's-eye view, where **replicators** with heritable traits compete for carrying capacity in a resource-constrained environment.

### Codebase Navigation Map

| Domain Concept | Theoretical Meaning | Source File | Key Structs / Functions |
| :--- | :--- | :--- | :--- |
| **Core Library** | Re-exports engine, models, and telemetry | [`src/lib.rs`](file:///home/vikman/Projects/selfish-gene/src/lib.rs) | `selfish_gene::*` |
| **Replicator (Gene)** | Unit of selection with heritable traits | [`src/replicator.rs`](file:///home/vikman/Projects/selfish-gene/src/replicator.rs) | `Replicator`, `create_offspring()`, `from_distributions()` |
| **Simulation Loop** | Generational cycle & selective pressures | [`src/simulation.rs`](file:///home/vikman/Projects/selfish-gene/src/simulation.rs) | `Simulation`, `step()`, `resource_factor()`, `reset()` |
| **Configuration** | Simulation parameters & CLI/GUI config | [`src/config.rs`](file:///home/vikman/Projects/selfish-gene/src/config.rs) | `Config` |
| **Statistics & History** | Population telemetry & convergence | [`src/stats.rs`](file:///home/vikman/Projects/selfish-gene/src/stats.rs) | `PopulationStats`, `ConvergenceDetector`, `SimulationHistory` |
| **Live CLI TUI** | Real-time TUI bar chart & distributions | [`src/visualization.rs`](file:///home/vikman/Projects/selfish-gene/src/visualization.rs) | `LiveVisualizer`, `ProfileBin` |
| **GUI Application** | Interactive dashboard (`egui` / `eframe`) | [`src/gui/app.rs`](file:///home/vikman/Projects/selfish-gene/src/gui/app.rs) | `SelfishGeneApp`, `RunnerState`, `render_params_panel` |
| **GUI Panels** | Parameter sliders, plots, profiles | [`src/gui/panels/`](file:///home/vikman/Projects/selfish-gene/src/gui/panels/) | `time_series`, `profiles`, `telemetry`, `controls`, `params` |
| **CLI Entrypoint** | Signal handling & CLI runner (`selfish-gene`) | [`src/main.rs`](file:///home/vikman/Projects/selfish-gene/src/main.rs) | `main()` |
| **GUI Entrypoint** | Window init & launcher (`selfish-gene-gui`) | [`src/bin/gui.rs`](file:///home/vikman/Projects/selfish-gene/src/bin/gui.rs) | `main()` |

---

## 📚 Documentation Index

Always refer to and maintain the documentation in `docs/`:

- [`docs/theory.md`](file:///home/vikman/Projects/selfish-gene/docs/theory.md): Biological principles (Dawkins), population genetics, and mathematical formulations.
- [`docs/parameters.md`](file:///home/vikman/Projects/selfish-gene/docs/parameters.md): Complete reference for all CLI parameters, defaults, and mathematical dynamics.
- [`docs/architecture.md`](file:///home/vikman/Projects/selfish-gene/docs/architecture.md): Internal system architecture, execution lifecycle, and module design.
- [`docs/extending.md`](file:///home/vikman/Projects/selfish-gene/docs/extending.md): Step-by-step developer protocols for introducing new traits, environmental dynamics, game theory, and scenario configs.

---

## ⚖️ Rules of Theoretical & Implementation Consistency

When proposing or implementing changes, you MUST adhere to the following rules:

1. **Gene-Centered Perspective (Dawkinsian Alignment):**
   - The fundamental unit of selection is the **replicator** (not the species or the group).
   - Replicators succeed or fail based on their longevity (survival), fecundity (replication rate), and copying fidelity (mutation rate).
   - Any cooperative or social behavior added in future extensions must be grounded in gene-level mechanisms (e.g., Kin Selection / Hamilton's Rule $rB > C$, Reciprocal Altruism / Iterated Prisoner's Dilemma, or ESS).

2. **Atomic Synchronization between Code and Docs:**
   - Any addition or modification of CLI arguments in [`src/config.rs`](file:///home/vikman/Projects/selfish-gene/src/config.rs) MUST be accompanied by an update in [`docs/parameters.md`](file:///home/vikman/Projects/selfish-gene/docs/parameters.md).
   - Any modification to trait dynamics or life cycle in [`src/replicator.rs`](file:///home/vikman/Projects/selfish-gene/src/replicator.rs) or [`src/simulation.rs`](file:///home/vikman/Projects/selfish-gene/src/simulation.rs) MUST be documented in [`docs/theory.md`](file:///home/vikman/Projects/selfish-gene/docs/theory.md) and [`docs/architecture.md`](file:///home/vikman/Projects/selfish-gene/docs/architecture.md).

3. **Rust Quality & Performance Standards:**
   - Keep inner simulation loops allocation-free where possible to support large populations ($N \ge 10^5$).
   - Preserve CLI determinism when `--seed` is provided by using `StdRng`.
   - Maintain full test coverage with `cargo test`.

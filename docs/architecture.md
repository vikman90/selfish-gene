# System Architecture & Code Map (`architecture.md`)

This document details the software architecture, module responsibilities, execution lifecycle, and data flow of the **Selfish Gene Simulator**.

---

## 1. High-Level Architecture Diagram

```mermaid
graph TD
    Main["src/main.rs<br><b>CLI & Signal Handling</b>"] --> Config["src/config.rs<br><b>Config (Clap + Serde)</b>"]
    Main --> Sim["src/simulation.rs<br><b>Simulation Engine</b>"]
    Sim --> Rep["src/replicator.rs<br><b>Replicator Entity</b>"]
    Sim --> Stats["src/stats.rs<br><b>PopulationStats & Convergence</b>"]
    Sim --> Vis["src/visualization.rs<br><b>LiveVisualizer (Crossterm)</b>"]
```

---

## 2. Module Responsibilities

### 2.1 `src/main.rs` — Application Entrypoint
- **Signal Handling:** Uses `signal_hook` to catch `SIGINT` (Ctrl+C) and `SIGTERM`, setting an `Arc<AtomicBool>` flag.
- **Initialization:** Parses CLI arguments into `Config` and instantiates the `Simulation` runner.

### 2.2 `src/config.rs` — Configuration & CLI Schema
- **`Config` Struct:** Derives `clap::Parser`, `serde::Serialize`, and `serde::Deserialize`.
- Acts as the single source of truth for all simulation settings and defaults.

### 2.3 `src/replicator.rs` — Biological Entity Model
- **`Replicator`:** Represents an individual with traits $(S, R, M)$ and state $a$ (`age`).
- **`from_distributions()`:** Samples initial traits from Gaussian distributions.
- **`create_offspring()`:** Clones traits and applies Gaussian mutation noise $\mathcal{N}(0, \sigma_m)$ if a mutation roll passes.

### 2.4 `src/simulation.rs` — Simulation Engine & Lifecycle
- Maintains the active population `Vec<Replicator>`, the pseudo-random generator `StdRng`, and the timeline counter `timestep`.
- Controls the discrete generational loop and evaluates termination conditions.

### 2.5 `src/stats.rs` — Statistical Engine & Convergence Detection
- **`PopulationStats`:** Computes population size, means ($\mu$), standard deviations ($\sigma$), and age metrics for each timestep.
- **`ConvergenceDetector`:** Maintains a sliding window (`VecDeque<PopulationStats>`) of size $W$. Detects convergence when $\max(\sigma_S, \sigma_R, \sigma_M) < \epsilon$ for all entries in the window.
- **`SimulationHistory`:** Serializes the complete timeline to formatted JSON for offline scientific analysis.

### 2.6 `src/visualization.rs` — Real-Time Terminal User Interface
- **`ProfileBin`:** Discretizes the continuous 3D trait space into discrete histogram bins:
  - Survival: 10 bins ($0.0 - 1.0$)
  - Replication: 20 bins ($0.0 - 2.0$)
  - Mutation: 5 bins ($0.00 - 0.05$)
- **`LiveVisualizer`:** Utilizes `crossterm` in raw mode with ANSI escape sequences to render non-flickering ASCII bar charts.

---

## 3. Generational Step Lifecycle

The `Simulation::step()` method executes four deterministic phases sequentially:

```mermaid
sequenceDiagram
    participant Sim as Simulation Engine
    participant Pop as Population Vec
    participant Rep as Replicator
    participant RNG as StdRng

    Note over Sim: Step 1: Inflow (Poisson Arrival)
    Sim->>RNG: Sample Poisson(appearance_rate)
    Sim->>Pop: Push new random replicators (if N < capacity)

    Note over Sim: Step 2: Survival & Senescence
    Sim->>RNG: Generate survival rolls [0, 1)
    Sim->>Pop: Retain if roll < S * exp(-senescence * age)

    Note over Sim: Step 3: Replication & Mutation
    Sim->>Sim: Compute resource_factor = max(0, 1 - N/C)
    loop For each living Replicator
        Sim->>Rep: Calculate offspring_count(R * resource_factor)
        loop For each child
            Sim->>Rep: create_offspring(mutation_sigma)
            Sim->>Pop: Collect offspring (up to capacity)
        end
    end

    Note over Sim: Step 4: Age Increment
    Sim->>Pop: Increment age by 1 for all individuals
```

---

## 4. Performance & Memory Design

- **Contiguous Allocation:** The population is stored in a flat `Vec<Replicator>`.
- **RNG Determinism:** When `--seed` is provided, `StdRng::seed_from_u64` ensures reproducible experiments across platforms.
- **Zero-Allocation Stats:** Trait means and standard deviations are computed in single-pass iterators without intermediate allocations.
- **Safe Signal Trapping:** Terminal raw mode is cleanly restored via the `Drop` implementation on `LiveVisualizer`, even upon interruption.

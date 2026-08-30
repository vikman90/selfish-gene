# Simulation Parameter Reference (`parameters.md`)

This document provides a comprehensive technical reference for all configuration parameters available in the **Selfish Gene Simulator** ([`src/config.rs`](file:///home/vikman/Projects/selfish-gene/src/config.rs)).

---

## 1. Quick Reference Table

| Flag / Option | Rust Type | Default | Domain / Range | Category | Description |
| :--- | :--- | :--- | :--- | :--- | :--- |
| `-C`, `--capacity` | `usize` | `10000` | $[1, \infty)$ | Population | Maximum carrying capacity ($C$) |
| `-a`, `--appearance-rate` | `f64` | `100.0` | $[0.0, \infty)$ | Inflow | Poisson rate parameter ($\lambda$) for spontaneous arrivals |
| `--init-survival-mean` | `f64` | `0.7` | $[0.0, 1.0]$ | Initial Traits | Mean base survival probability ($\mu_S$) |
| `--init-survival-std` | `f64` | `0.1` | $[0.0, \infty)$ | Initial Traits | Std dev of base survival probability ($\sigma_S$) |
| `--init-replication-mean` | `f64` | `1.2` | $[0.0, \infty)$ | Initial Traits | Mean base replication rate ($\mu_R$) |
| `--init-replication-std` | `f64` | `0.2` | $[0.0, \infty)$ | Initial Traits | Std dev of base replication rate ($\sigma_R$) |
| `--init-mutation-mean` | `f64` | `0.01` | $[0.0, 1.0]$ | Initial Traits | Mean mutation probability ($\mu_M$) |
| `--init-mutation-std` | `f64` | `0.005` | $[0.0, \infty)$ | Initial Traits | Std dev of mutation probability ($\sigma_M$) |
| `--mutation-sigma` | `f64` | `0.02` | $[0.0, \infty)$ | Evolution | Std dev of Gaussian mutation noise ($\sigma_m$) |
| `--senescence-rate` | `f64` | `0.0` | $[0.0, \infty)$ | Evolution | Age-dependent mortality factor ($\gamma$) |
| `-t`, `--max-timesteps` | `usize` | `1000` | $[0, \infty)$ | Execution | Max iterations ($0 = \text{unlimited}$) |
| `--convergence-threshold`| `f64` | `0.001` | $(0.0, \infty)$ | Convergence | Max variance threshold for stability ($\epsilon$) |
| `--convergence-window` | `usize` | `50` | $[1, \infty)$ | Convergence | Consecutive timesteps required below threshold ($W$) |
| `--display-interval` | `usize` | `10` | $[1, \infty)$ | Telemetry | TUI refresh / log frequency |
| `--no-live-display` | `bool` | `false` | `bool` | Telemetry | Disable TUI and print plain text logs |
| `--top-profiles` | `usize` | `15` | $[1, \infty)$ | Telemetry | Number of dominant bins in the histogram |
| `-o`, `--output-file` | `Option<String>` | `None` | File path | Export | JSON export path for full time-series history |
| `--seed` | `Option<u64>` | `None` | $\mathbb{N}_0$ | Determinism | RNG seed for exact experiment reproducibility |

---

## 2. Detailed Parameter Mechanics

### 2.1 Environmental Carrying Capacity & Inflow

#### `--capacity` (`-C <N>`)
- **Default:** `10000`
- **Formula:** 
  $$\Phi(N) = \max\left(0.0,\ 1.0 - \frac{N}{C}\right)$$
- **Role:** Sets the ecological limit $C$. When population $N$ is small, resource factor $\Phi(N) \approx 1.0$, allowing maximum reproduction. As $N \to C$, $\Phi(N) \to 0.0$, halting reproduction.

#### `--appearance-rate` (`-a <RATE>`)
- **Default:** `100.0`
- **Formula:**
  $$K_t \sim \text{Poisson}(\lambda)$$
- **Role:** Number of primordial individuals injected into the environment per timestep if $N < C$. Guarantees that the simulation can recover even after a catastrophic extinction event.

---

### 2.2 Initial Trait Distributions

When a new replicator is generated from the environment (either at initialization or through appearance), its traits $(S, R, M)$ are independently sampled:

```text
Survival Rate:    S ~ Normal(init_survival_mean,    init_survival_std),    clamped to [0.0, 1.0]
Replication Rate: R ~ Normal(init_replication_mean, init_replication_std), clamped to [0.0, +inf)
Mutation Rate:    M ~ Normal(init_mutation_mean,    init_mutation_std),    clamped to [0.0, 1.0]
```

- **Survival ($S$):** Probability of surviving a single timestep without aging effects.
- **Replication ($R$):** Expected number of offspring before environmental scaling $\Phi(N)$.
- **Mutation ($M$):** Probability that a replication event perturbs traits in the offspring.

---

### 2.3 Evolutionary Mechanics & Senescence

#### `--mutation-sigma <SIGMA>`
- **Default:** `0.02`
- **Formula:** 
  $$\text{Trait}_{\text{child}} = \text{clamp}\left(\text{Trait}_{\text{parent}} + \mathcal{N}(0, \sigma_m)\right)$$
- **Role:** Controls the step size of evolutionary divergence. Small values ($\sigma_m \le 0.01$) lead to smooth hill climbing; large values ($\sigma_m \ge 0.1$) introduce strong genetic drift and risk breaking optimized genotypes.

#### `--senescence-rate <RATE>`
- **Default:** `0.0`
- **Formula:**
  $$S_{\text{effective}}(a) = S \cdot \exp(-\gamma \cdot a)$$
- **Role:** Simulates physiological aging. When $\gamma > 0$, older individuals experience higher mortality rates, selecting for strategies that reproduce earlier in their lifecycle.

---

### 2.4 Convergence Detection & Stop Conditions

The simulation terminates when any of the following occur:
1. **User Interruption:** `Ctrl+C` / `SIGINT` / `SIGTERM` (graceful shutdown).
2. **Timestep Limit:** $t \ge \text{max\_timesteps}$ (unless set to `0`).
3. **Trait Convergence:** When the population achieves phenotypic homogeneity.

#### Convergence Formula:
For a sliding window of length $W$ (`--convergence-window`):
$$\max\left(\sigma_S(t), \sigma_R(t), \sigma_M(t)\right) < \epsilon \quad \forall t \in [T - W + 1, T]$$
where $\epsilon$ is `--convergence-threshold`.

---

## 3. Practical CLI Experiment Presets

### Experiment 1: High Selective Pressure with Senescence
```bash
selfish-gene \
  --capacity 20000 \
  --senescence-rate 0.05 \
  --max-timesteps 2000 \
  --mutation-sigma 0.01
```

### Experiment 2: Deterministic Baseline with JSON Telemetry
```bash
selfish-gene \
  --seed 42 \
  --max-timesteps 500 \
  --output-file experiment_seed42.json \
  --no-live-display
```

### Experiment 3: High Mutation / Rapid Exploration
```bash
selfish-gene \
  --init-mutation-mean 0.05 \
  --mutation-sigma 0.05 \
  --convergence-threshold 0.0005 \
  --convergence-window 100
```

# Theoretical Foundations: The Selfish Gene Model

This document outlines the evolutionary theory and mathematical models underlying the simulator, rooted in **Richard Dawkins' *The Selfish Gene* (1976)** and mathematical population biology.

---

## 1. The Gene's-Eye View of Evolution

In classical evolutionary biology, natural selection was often loosely described in terms of "the good of the species" or "the individual organism". Dawkins formalized and popularized the **gene-centered view of evolution**:

1. **Replicators vs. Vehicles:**
   - **Replicators (Genes):** The fundamental units of selection. Entities capable of producing copies of themselves that persist through generations.
   - **Vehicles (Survival Machines):** Temporary biological constructs (cells, organisms, groups) built by replicators to facilitate their own replication and survival in a hostile environment.

2. **The Three Core Properties of Successful Replicators:**
   Dawkins identified three primary attributes that determine evolutionary fitness:
   - **Longevity ($S$):** How long an individual replicator survives.
   - **Fecundity ($R$):** The rate at which the replicator produces copies of itself.
   - **Copy-Fidelity ($M$):** The precision with which copies are made without error.

---

## 2. Mathematical Formalization in the Simulator

### 2.1 Replicator State & Traits

Each individual replicator $i$ is defined by a trait vector $\mathbf{x}_i$ and an age counter $a_i \in \mathbb{N}_0$:

$$\mathbf{x}_i = \begin{pmatrix} S_i \\ R_i \\ M_i \\ A_i \end{pmatrix}$$

Where:
- $S_i \in [0.0, 1.0]$: Base probability of surviving a discrete timestep.
- $R_i \in [0.0, \infty)$: Baseline expected offspring produced per timestep.
- $M_i \in [0.0, 1.0]$: Probability of mutation occurring during replication.
- $A_i \in [0.0, 1.0]$: Heritable aggression propensity ($1.0 = \text{Pure Hawk}$, $0.0 = \text{Pure Dove}$).
- $a_i \in \mathbb{N}_0$: Age in elapsed timesteps since creation ($a_i = 0$ for newborns).

---

### 2.2 Primordial Soup & Appearance (Poisson Arrival)

In an open evolutionary system, new novel replicators emerge spontaneously from the primordial environment.

The number of spontaneous arrivals $K_t$ at timestep $t$ follows a Poisson distribution:

$$K_t \sim \text{Poisson}(\lambda_{\text{appearance}})$$

Each new replicator's traits are sampled from truncated normal distributions:

$$S_{\text{init}} \sim \text{clamp}_{[0, 1]}\left(\mathcal{N}(\mu_S, \sigma_S)\right)$$

$$R_{\text{init}} \sim \text{clamp}_{[0, \infty)}\left(\mathcal{N}(\mu_R, \sigma_R)\right)$$

$$M_{\text{init}} \sim \text{clamp}_{[0, 1]}\left(\mathcal{N}(\mu_M, \sigma_M)\right)$$

$$A_{\text{init}} \sim \text{clamp}_{[0, 1]}\left(\mathcal{N}(\mu_A, \sigma_A)\right)$$

---

### 2.3 Environmental Carrying Capacity & Resource Pressure

The environment has a finite carrying capacity $C$. As the population size $N_t = |\mathcal{P}_t|$ approaches $C$, resource scarcity decreases reproductive success (density-dependent selection):

$$\Phi(N_t) = \max\left(0,\ 1 - \frac{N_t}{C}\right)$$

The effective replication rate for replicator $i$ becomes:

$$R_{i, \text{effective}} = (R_i + E_{i, +}) \cdot \Phi(N_t)$$

The discrete number of offspring $O_i$ produced by replicator $i$ in a timestep is:

$$O_i = \lfloor R_{i, \text{effective}} \rfloor + \mathbb{I}_{\left\{U < (R_{i, \text{effective}} - \lfloor R_{i, \text{effective}} \rfloor)\right\}}$$

where $U \sim \text{Uniform}(0, 1)$ and $\mathbb{I}$ is the indicator function.

---

### 2.4 Survival, Senescence, and Combat Injury Dynamics

In nature, genes that express lethal effects early in life are rapidly purged by natural selection, while genes with deleterious effects expressed later in life (after reproduction) accumulate—a theory formalized by **Peter Medawar (1952)** and **George C. Williams (1957)**, discussed extensively in Dawkins (*Chapter 3*). Furthermore, aggressive physical conflicts entail serious risk of injury or death (*Chapter 5*).

In this simulator, mortality combines senescence with combat injuries:

$$S_{i, \text{effective}}(a_i, E_i) = \text{clamp}_{[0, 1]}\left( S_i \cdot e^{-\gamma \cdot a_i} \cdot f_{\text{injury}}(E_i) \right)$$

where $\gamma \ge 0$ is the `--senescence-rate`, and $f_{\text{injury}}(E_i) = \max\left(0.05, 1.0 + \frac{E_i}{C_{\text{cost}}}\right)$ when net game payoff $E_i < 0$.

---

### 2.5 Mutation and Heritability (Gaussian Trait Perturbation)

When an offspring is produced, mutation occurs with probability $M_i$:

$$\text{mutate} \sim \text{Bernoulli}(M_i)$$

If mutation occurs, all traits undergo independent Gaussian perturbations with noise scale $\sigma_{\text{mutation}}$:

$$\begin{aligned}
S_{\text{child}} &= \text{clamp}_{[0, 1]}\left(S_i + \delta_S\right), \quad &\delta_S \sim \mathcal{N}(0, \sigma_{\text{mutation}}) \\
R_{\text{child}} &= \text{clamp}_{[0, \infty)}\left(R_i + \delta_R\right), \quad &\delta_R \sim \mathcal{N}(0, \sigma_{\text{mutation}}) \\
M_{\text{child}} &= \text{clamp}_{[0, 1]}\left(M_i + \delta_M\right), \quad &\delta_M \sim \mathcal{N}(0, \sigma_{\text{mutation}}) \\
A_{\text{child}} &= \text{clamp}_{[0, 1]}\left(A_i + \delta_A\right), \quad &\delta_A \sim \mathcal{N}(0, \sigma_{\text{mutation}})
\end{aligned}$$

If no mutation occurs, the offspring inherits exact copies of the parental traits, with age reset to zero ($a_{\text{child}} = 0$).

---

### 2.6 Evolutionary Game Theory & ESS (Hawk-Dove Model)

In Chapter 5 of *The Selfish Gene*, Dawkins formalizes John Maynard Smith's **Evolutionarily Stable Strategy (ESS)**. Replicators participate in pairwise competitive encounters for a contested resource of value $V$, risking fighting injury cost $C$.

#### Payoff Matrix:
| Player $i$ \ Player $j$ | Hawk | Dove |
| :--- | :---: | :---: |
| **Hawk** | $\frac{V - C}{2}$ | $V$ |
| **Dove** | $0$ | $\frac{V}{2}$ |

#### ESS Equilibrium Derivation:
In a population where fraction $p$ plays Hawk and $1-p$ plays Dove:
$$E(\text{Hawk}) = p \cdot \frac{V - C}{2} + (1 - p) \cdot V = V - p \cdot \frac{V + C}{2}$$
$$E(\text{Dove}) = p \cdot 0 + (1 - p) \cdot \frac{V}{2} = (1 - p) \cdot \frac{V}{2}$$

At evolutionary equilibrium ($E(\text{Hawk}) = E(\text{Dove})$):
$$V - p \frac{V + C}{2} = \frac{V}{2} - p \frac{V}{2} \iff p^* = \frac{V}{C}$$

- **When $V \ge C$:** Hawk is unconditionally ESS ($p^* = 1.0$).
- **When $C > V$:** A mixed/polymorphic equilibrium emerges at $p^* = \frac{V}{C}$.

---

## 3. Evolutionary Phenomena Observed in the Model

1. **Survival Maximization ($S \to 1.0$):**
   In the absence of a trade-off cost, selection pressure drives survival to its theoretical ceiling.
2. **Optimal Fecundity Balancing ($R \to 1.1 - 1.4$):**
   Because resource availability $\Phi(N)$ drops as $N \to C$, excessively high $R$ leads to boom-and-bust overshoot, whereas balanced $R$ provides sustained lineage dominance.
3. **Mutation Rate Stabilization ($M \to 0.005 - 0.01$):**
   High mutation rates destroy well-adapted genomes (error catastrophe), while zero mutation prevents adaptation to changing densities.
4. **Dynamic ESS Convergence ($A \to V/C$):**
   When game theory is active with $C > V$, the population mean aggression self-organizes to the theoretical ESS ratio $p^* = V/C$.

---

## 4. Theoretical Roadmap for Future Extensions

The simulator architecture is designed to accommodate richer Dawkinsian evolutionary mechanics:

### A. Evolutionarily Stable Strategies (ESS) & Game Theory (Dawkins Ch. 5)
- Pairwise interactions between individuals using game payoff matrices (Hawks vs. Doves, Retaliators, Probers).
- Payoffs directly augment or penalize energy/resource factors $\Phi$.

### B. Kin Selection & Inclusive Fitness (Dawkins Ch. 6)
- Replicator genetic lineage tracking ($r$, coefficient of relatedness).
- Altruistic actions governed by **Hamilton's Rule**:
  $$r \cdot B > C$$
  where $B$ is recipient benefit and $C$ is actor cost.

### C. Reciprocal Altruism (Dawkins Ch. 10)
- Iterated Prisoner's Dilemma strategies (Tit-for-Tat, Grudger, Always Defect) with memory across timesteps.

### D. Trade-offs & Metabolic Costs
- Coupling $R$ and $S$ via an energetic budget constraint $f(S, R) \le E_{\text{budget}}$ to reflect physiological trade-offs.

---

## 5. Bibliography

1. **Dawkins, R.** (1976). *The Selfish Gene*. Oxford University Press.
2. **Dawkins, R.** (1982). *The Extended Phenotype*. Oxford University Press.
3. **Maynard Smith, J.** (1982). *Evolution and the Theory of Games*. Cambridge University Press.
4. **Medawar, P. B.** (1952). *An Unsolved Problem of Biology*. H. K. Lewis, London.
5. **Williams, G. C.** (1957). *Pleiotropy, natural selection, and the evolution of senescence*. Evolution, 11(4), 398-411.
6. **Hamilton, W. D.** (1964). *The genetical evolution of social behaviour. I and II*. Journal of Theoretical Biology, 7(1), 1-52.

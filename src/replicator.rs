use rand::Rng;
use rand_distr::{Distribution, Normal};
use serde::{Deserialize, Serialize};

/// A replicator with heritable traits
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Replicator {
    /// Probability of surviving each timestep
    pub survival_rate: f64,

    /// Expected number of offspring per timestep (before resource adjustment)
    pub replication_rate: f64,

    /// Probability of mutation occurring during replication
    pub mutation_rate: f64,
}

impl Replicator {
    /// Create a new replicator with specified traits
    pub fn new(survival_rate: f64, replication_rate: f64, mutation_rate: f64) -> Self {
        Self {
            survival_rate: survival_rate.max(0.0).min(1.0),
            replication_rate: replication_rate.max(0.0),
            mutation_rate: mutation_rate.max(0.0).min(1.0),
        }
    }

    /// Create a replicator with traits sampled from normal distributions
    pub fn from_distributions<R: Rng>(
        rng: &mut R,
        survival_mean: f64,
        survival_std: f64,
        replication_mean: f64,
        replication_std: f64,
        mutation_mean: f64,
        mutation_std: f64,
    ) -> Self {
        let survival_dist = Normal::new(survival_mean, survival_std).unwrap();
        let replication_dist = Normal::new(replication_mean, replication_std).unwrap();
        let mutation_dist = Normal::new(mutation_mean, mutation_std).unwrap();

        Self::new(
            survival_dist.sample(rng),
            replication_dist.sample(rng),
            mutation_dist.sample(rng),
        )
    }

    /// Check if this replicator survives this timestep
    #[allow(dead_code)]
    pub fn survives<R: Rng>(&self, rng: &mut R) -> bool {
        rng.gen::<f64>() < self.survival_rate
    }

    /// Replicate this replicator, applying resource factor
    /// Returns the number of offspring produced
    #[allow(dead_code)]
    pub fn replicate<R: Rng>(&self, rng: &mut R, resource_factor: f64) -> usize {
        let effective_rate = self.replication_rate * resource_factor;

        // Split into integer and fractional parts
        let base = effective_rate.floor() as usize;
        let fractional = effective_rate - effective_rate.floor();

        // Add one more with probability equal to fractional part
        if rng.gen::<f64>() < fractional {
            base + 1
        } else {
            base
        }
    }

    /// Create an offspring, possibly with mutations
    pub fn create_offspring<R: Rng>(&self, rng: &mut R, mutation_sigma: f64) -> Self {
        let mutate = rng.gen::<f64>() < self.mutation_rate;

        if mutate {
            let noise_dist = Normal::new(0.0, mutation_sigma).unwrap();
            Self::new(
                self.survival_rate + noise_dist.sample(rng),
                self.replication_rate + noise_dist.sample(rng),
                self.mutation_rate + noise_dist.sample(rng),
            )
        } else {
            self.clone()
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use rand::SeedableRng;
    use rand::rngs::StdRng;

    #[test]
    fn test_replicator_creation() {
        let rep = Replicator::new(0.8, 1.5, 0.02);
        assert_eq!(rep.survival_rate, 0.8);
        assert_eq!(rep.replication_rate, 1.5);
        assert_eq!(rep.mutation_rate, 0.02);
    }

    #[test]
    fn test_replicator_bounds() {
        let rep = Replicator::new(-0.5, -1.0, 1.5);
        assert_eq!(rep.survival_rate, 0.0);
        assert_eq!(rep.replication_rate, 0.0);
        assert_eq!(rep.mutation_rate, 1.0);
    }

    #[test]
    fn test_survival() {
        let mut rng = StdRng::seed_from_u64(42);
        let rep = Replicator::new(1.0, 1.0, 0.0);
        assert!(rep.survives(&mut rng));

        let rep_dead = Replicator::new(0.0, 1.0, 0.0);
        assert!(!rep_dead.survives(&mut rng));
    }

    #[test]
    fn test_replication() {
        let mut rng = StdRng::seed_from_u64(42);
        let rep = Replicator::new(0.8, 2.5, 0.0);

        let offspring_count = rep.replicate(&mut rng, 1.0);
        assert!(offspring_count >= 2 && offspring_count <= 3);
    }
}

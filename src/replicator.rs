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

    /// Propensity for aggressive/Hawk behavior in game interactions [0.0, 1.0]
    pub aggression: f64,

    /// Age in timesteps (starts at 0 for new individuals)
    pub age: usize,
}

impl Replicator {
    /// Create a new replicator with specified traits
    pub fn new(
        survival_rate: f64,
        replication_rate: f64,
        mutation_rate: f64,
        aggression: f64,
    ) -> Self {
        Self {
            survival_rate: survival_rate.clamp(0.0, 1.0),
            replication_rate: replication_rate.max(0.0),
            mutation_rate: mutation_rate.clamp(0.0, 1.0),
            aggression: aggression.clamp(0.0, 1.0),
            age: 0,
        }
    }

    /// Create a replicator with traits sampled from normal distributions
    #[allow(clippy::too_many_arguments)]
    pub fn from_distributions<R: Rng>(
        rng: &mut R,
        survival_mean: f64,
        survival_std: f64,
        replication_mean: f64,
        replication_std: f64,
        mutation_mean: f64,
        mutation_std: f64,
        aggression_mean: f64,
        aggression_std: f64,
    ) -> Self {
        let survival_dist = Normal::new(survival_mean, survival_std).unwrap();
        let replication_dist = Normal::new(replication_mean, replication_std).unwrap();
        let mutation_dist = Normal::new(mutation_mean, mutation_std).unwrap();
        let aggression_dist = Normal::new(aggression_mean, aggression_std).unwrap();

        Self::new(
            survival_dist.sample(rng),
            replication_dist.sample(rng),
            mutation_dist.sample(rng),
            aggression_dist.sample(rng),
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
                self.aggression + noise_dist.sample(rng),
            )
        } else {
            // Non-mutated offspring is a clone of parent traits but age resets to 0
            let mut child = self.clone();
            child.age = 0;
            child
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use rand::rngs::StdRng;
    use rand::SeedableRng;

    #[test]
    fn test_replicator_creation() {
        let rep = Replicator::new(0.8, 1.5, 0.02, 0.6);
        assert_eq!(rep.survival_rate, 0.8);
        assert_eq!(rep.replication_rate, 1.5);
        assert_eq!(rep.mutation_rate, 0.02);
        assert_eq!(rep.aggression, 0.6);
        assert_eq!(rep.age, 0);
    }

    #[test]
    fn test_replicator_bounds() {
        let rep = Replicator::new(-0.5, -1.0, 1.5, 2.0);
        assert_eq!(rep.survival_rate, 0.0);
        assert_eq!(rep.replication_rate, 0.0);
        assert_eq!(rep.mutation_rate, 1.0);
        assert_eq!(rep.aggression, 1.0);

        let rep_low = Replicator::new(0.5, 1.0, 0.01, -0.5);
        assert_eq!(rep_low.aggression, 0.0);
    }

    #[test]
    fn test_survival() {
        let mut rng = StdRng::seed_from_u64(42);
        let rep = Replicator::new(1.0, 1.0, 0.0, 0.5);
        assert!(rep.survives(&mut rng));

        let rep_dead = Replicator::new(0.0, 1.0, 0.0, 0.5);
        assert!(!rep_dead.survives(&mut rng));
    }

    #[test]
    fn test_replication() {
        let mut rng = StdRng::seed_from_u64(42);
        let rep = Replicator::new(0.8, 2.5, 0.0, 0.5);

        let offspring_count = rep.replicate(&mut rng, 1.0);
        assert!((2..=3).contains(&offspring_count));
    }

    #[test]
    fn test_create_offspring_mutation() {
        let mut rng = StdRng::seed_from_u64(42);
        // Guaranteed mutation (mutation_rate = 1.0)
        let parent = Replicator::new(0.7, 1.2, 1.0, 0.5);
        let child = parent.create_offspring(&mut rng, 0.1);
        assert_eq!(child.age, 0);
        assert!(
            (child.aggression - parent.aggression).abs() > 0.0
                || (child.survival_rate - parent.survival_rate).abs() > 0.0
        );
        assert!((0.0..=1.0).contains(&child.aggression));
    }
}

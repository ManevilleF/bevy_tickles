use bevy::prelude::Component;
use rand::rngs::SmallRng;
use rand::SeedableRng;

/// Component responsible for the randomization of particles
#[derive(Debug, Clone, Component)]
pub struct ParticleRng(pub SmallRng);

impl Default for ParticleRng {
    fn default() -> Self {
        Self(SmallRng::from_entropy())
    }
}

impl ParticleRng {
    /// Retrieves the random generator
    pub fn rng(&mut self) -> &mut SmallRng {
        &mut self.0
    }
}

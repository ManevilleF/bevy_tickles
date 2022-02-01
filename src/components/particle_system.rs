use crate::particle::Particle;
use bevy::prelude::Component;
use std::ops::{Deref, DerefMut};

/// Particle System simulation container
#[derive(Debug, Clone, Default, Component)]
pub struct ParticleSystem {
    /// Every simulated particles
    pub particles: Vec<Particle>,
    /// Simulation start time
    pub start_time: f32,
}

impl Deref for ParticleSystem {
    type Target = Vec<Particle>;

    fn deref(&self) -> &Self::Target {
        &self.particles
    }
}

impl DerefMut for ParticleSystem {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.particles
    }
}

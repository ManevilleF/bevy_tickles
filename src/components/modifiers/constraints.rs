use crate::modifiers::ParticleSystemModifier;
use crate::{Particle, ParticleModifier, ParticleSystem};
use bevy::prelude::{Component, Reflect};

/// Constraints [`ParticleSystem`] particle system count to a max value
#[derive(Debug, Copy, Clone, Component, Reflect)]
pub struct MaxParticleCount(pub usize);

impl Default for MaxParticleCount {
    fn default() -> Self {
        Self(1_000)
    }
}

impl ParticleSystemModifier for MaxParticleCount {
    fn apply(&self, particles: &mut ParticleSystem, _: f32) {
        let delta = particles.len().saturating_sub(self.0);
        if delta > 0 {
            particles.particles.drain(0..delta);
        }
    }
}

/// Constraints [`ParticleSystem`] particle system count to a max speed
#[derive(Debug, Copy, Clone, Component, Reflect)]
pub struct MaxParticleSpeed(pub f32);

impl Default for MaxParticleSpeed {
    fn default() -> Self {
        Self(100.0)
    }
}

impl ParticleModifier for MaxParticleSpeed {
    fn apply(&self, particle: &mut Particle, _: f32) {
        if particle.speed() > self.0 {
            particle.velocity = particle.velocity.normalize() * self.0;
        }
    }
}

/// Constraints [`ParticleSystem`] particle system count to a max speed
#[derive(Debug, Copy, Clone, Component, Reflect)]
pub struct MaxParticleSize(pub f32);

impl Default for MaxParticleSize {
    fn default() -> Self {
        Self(100.0)
    }
}

impl ParticleModifier for MaxParticleSize {
    fn apply(&self, particle: &mut Particle, _: f32) {
        if particle.size > self.0 {
            particle.size = self.0;
        }
    }
}

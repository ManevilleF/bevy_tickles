use crate::modifiers::ParticleSystemModifier;
use crate::ParticleSystem;
use bevy::prelude::{Component, Reflect};

/// Constraints [`ParticleSystem`] particle system count to a fixed value
#[derive(Debug, Copy, Clone, Component, Reflect)]
pub struct MaxParticleCount(pub u32);

impl Default for MaxParticleCount {
    fn default() -> Self {
        Self(1_000)
    }
}

impl ParticleSystemModifier for MaxParticleCount {
    fn apply(&self, particles: &mut ParticleSystem, _: f32) {
        let delta = particles.len().saturating_sub(self.0 as usize);
        if delta > 0 {
            particles.drain(0..delta);
        }
    }
}

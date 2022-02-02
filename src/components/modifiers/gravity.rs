use crate::{Particle, ParticleModifier};
use bevy::prelude::{Component, Reflect, Vec3};

/// Gravity for particles
#[derive(Debug, Clone, Component, Reflect)]
pub struct ParticleGravity(pub Vec3);

impl Default for ParticleGravity {
    fn default() -> Self {
        Self(Vec3::new(0., -9.81, 0.))
    }
}

impl ParticleModifier for ParticleGravity {
    #[inline]
    fn apply(&self, particle: &mut Particle, _delta_time: f32) {
        particle.velocity += self.0; // TODO: test this
    }
}

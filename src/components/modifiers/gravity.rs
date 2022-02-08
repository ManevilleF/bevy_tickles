use crate::modifiers::ParticleModifier;
use crate::Particle;
use bevy::prelude::{Component, Reflect, Vec3};

/// Gravity for particles
#[derive(Debug, Clone, Component, Reflect)]
#[cfg_attr(feature = "inspector", derive(bevy_inspector_egui::Inspectable))]
pub struct ParticleGravity(pub Vec3);

impl Default for ParticleGravity {
    fn default() -> Self {
        Self(Vec3::new(0., -9.81, 0.))
    }
}

impl ParticleModifier for ParticleGravity {
    #[inline]
    fn apply(&self, particle: &mut Particle, delta_time: f32) {
        particle.velocity += self.0 * delta_time;
    }
}

impl From<Vec3> for ParticleGravity {
    fn from(gravity: Vec3) -> Self {
        Self(gravity)
    }
}

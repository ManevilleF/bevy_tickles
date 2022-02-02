use crate::{Particle, ParticleModifier};
use bevy::prelude::{Component, Reflect};

#[derive(Debug, Copy, Clone, Component, Reflect)]
pub struct VelocityOverTime(pub f32);

impl ParticleModifier for VelocityOverTime {
    fn apply(&self, particle: &mut Particle, delta_time: f32) {
        particle.velocity += delta_time * self.0;
    }
}

#[derive(Debug, Copy, Clone, Component, Reflect)]
pub struct AngularVelocityOverTime(pub f32);

impl ParticleModifier for AngularVelocityOverTime {
    fn apply(&self, particle: &mut Particle, delta_time: f32) {
        particle.angular_velocity += delta_time * self.0;
    }
}

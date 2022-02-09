use crate::modifiers::ParticleModifier;
use crate::Particle;
use bevy::prelude::{Component, Reflect};

/// Increases particle rotation over time
#[derive(Debug, Copy, Clone, Default, Component, Reflect)]
#[cfg_attr(feature = "inspector", derive(bevy_inspector_egui::Inspectable))]
pub struct RotationOverTime(pub f32);

impl ParticleModifier for RotationOverTime {
    fn apply(&self, particle: &mut Particle, delta_time: f32) {
        particle.try_rotate(delta_time * self.0);
    }
}

/// Increases particle rotation over its velocity
#[derive(Debug, Copy, Clone, Default, Component, Reflect)]
#[cfg_attr(feature = "inspector", derive(bevy_inspector_egui::Inspectable))]
pub struct RotationOverVelocity {
    /// The rotation coefficient
    pub value: f32,
    /// Ignore velocity direction and use absolute value
    pub abs: bool,
}

impl ParticleModifier for RotationOverVelocity {
    fn apply(&self, particle: &mut Particle, delta_time: f32) {
        let mut dir = particle.non_zero_direction().x;
        if self.abs {
            dir = dir.abs();
        }
        particle.try_rotate(dir.signum() * particle.speed() * delta_time * self.value);
    }
}

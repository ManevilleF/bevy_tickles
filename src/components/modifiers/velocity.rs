use crate::{Particle, ParticleModifier};
use bevy::prelude::{Component, Reflect, Vec3};

/// Increases particle speed over time
#[derive(Debug, Copy, Clone, Default, Component, Reflect)]
#[cfg_attr(feature = "inspector", derive(bevy_inspector_egui::Inspectable))]
pub struct SpeedOverTime(pub f32);

impl ParticleModifier for SpeedOverTime {
    fn apply(&self, particle: &mut Particle, delta_time: f32) {
        particle.velocity += delta_time * self.0;
    }
}

/// Increases particle velocity over time
#[derive(Debug, Copy, Clone, Default, Component, Reflect)]
#[cfg_attr(feature = "inspector", derive(bevy_inspector_egui::Inspectable))]
pub struct VelocityOverTime(pub Vec3);

impl ParticleModifier for VelocityOverTime {
    fn apply(&self, particle: &mut Particle, delta_time: f32) {
        particle.velocity += delta_time * self.0;
    }
}

/// Increases particle velocity over time
#[derive(Debug, Copy, Clone, Default, Component, Reflect)]
#[cfg_attr(feature = "inspector", derive(bevy_inspector_egui::Inspectable))]
pub struct AngularVelocityOverTime(pub f32);

impl ParticleModifier for AngularVelocityOverTime {
    fn apply(&self, particle: &mut Particle, delta_time: f32) {
        particle.angular_velocity += delta_time * self.0;
    }
}

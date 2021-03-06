use crate::modifiers::ParticleModifier;
use crate::Particle;
use bevy::prelude::{Component, Reflect};

/// Increases particle size over time
#[derive(Debug, Copy, Clone, Default, Component, Reflect)]
#[cfg_attr(feature = "inspector", derive(bevy_inspector_egui::Inspectable))]
pub struct SizeOverTime(pub f32);

impl ParticleModifier for SizeOverTime {
    fn apply(&self, particle: &mut Particle, delta_time: f32) {
        particle.size += delta_time * self.0;
    }
}

impl From<f32> for SizeOverTime {
    fn from(v: f32) -> Self {
        Self(v)
    }
}

/// Increases particle size over its speed
#[derive(Debug, Copy, Clone, Default, Component, Reflect)]
#[cfg_attr(feature = "inspector", derive(bevy_inspector_egui::Inspectable))]
pub struct SizeOverSpeed(pub f32);

impl ParticleModifier for SizeOverSpeed {
    fn apply(&self, particle: &mut Particle, delta_time: f32) {
        particle.size += particle.speed() * delta_time * self.0;
    }
}

impl From<f32> for SizeOverSpeed {
    fn from(v: f32) -> Self {
        Self(v)
    }
}

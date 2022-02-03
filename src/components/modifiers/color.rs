use crate::{Color, ColorGradient, Particle, ParticleModifier};
use bevy::prelude::{Component, Reflect};

/// Evaluates particle color over its lifetime
#[derive(Debug, Copy, Clone, Default, Component, Reflect)]
#[cfg_attr(feature = "inspector", derive(bevy_inspector_egui::Inspectable))]
pub struct ColorOverLifeTime(pub ColorGradient);

impl ParticleModifier for ColorOverLifeTime {
    fn apply(&self, particle: &mut Particle, _delta_time: f32) {
        particle.color = self.0.evaluate_linear(particle.alive_time_ratio())
    }
}

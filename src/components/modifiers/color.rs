use crate::modifiers::ParticleModifier;
use crate::{ColorGradient, Particle};
use bevy::prelude::{Component, Reflect};

/// Evaluates particle color over its lifetime
#[derive(Debug, Clone, Default, Component, Reflect)]
#[cfg_attr(feature = "inspector", derive(bevy_inspector_egui::Inspectable))]
pub struct ColorOverLifeTime(pub ColorGradient);

impl ParticleModifier for ColorOverLifeTime {
    fn apply(&self, particle: &mut Particle, _delta_time: f32) {
        particle.color = self.0.evaluate(particle.alive_time_ratio());
    }
}

impl From<ColorGradient> for ColorOverLifeTime {
    fn from(gradient: ColorGradient) -> Self {
        Self(gradient)
    }
}

/// Evaluates particle color according to its speed
#[derive(Debug, Clone, Default, Component, Reflect)]
#[cfg_attr(feature = "inspector", derive(bevy_inspector_egui::Inspectable))]
pub struct ColorOverSpeed {
    /// Color gradient
    pub gradient: ColorGradient,
    /// Range of the speed values to evaluate the gradient which is always between 0 and 1
    pub speed_range: (f32, f32),
}

impl ParticleModifier for ColorOverSpeed {
    fn apply(&self, particle: &mut Particle, _delta_time: f32) {
        let delta =
            (particle.speed() - self.speed_range.0) / (self.speed_range.1 - self.speed_range.0);
        particle.color = self.gradient.evaluate(delta);
    }
}

impl From<ColorGradient> for ColorOverSpeed {
    fn from(gradient: ColorGradient) -> Self {
        Self {
            gradient,
            speed_range: (0.0, 1.0),
        }
    }
}

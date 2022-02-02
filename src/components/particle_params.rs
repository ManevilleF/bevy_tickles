use crate::particle::Particle;
use crate::RangeOrFixed;
use bevy::prelude::{Color, Component, Reflect, Vec3};
use rand::Rng;

/// Defines the initial state of emitted particles
#[derive(Debug, Clone, Component, Reflect)]
pub struct ParticleParams {
    /// Start lifetime of simulated particles
    pub start_lifetime: RangeOrFixed<f32>,
    /// Start size of simulated particles
    pub start_size: RangeOrFixed<f32>,
    /// Start rotation for simulated particles
    pub start_rotation: RangeOrFixed<f32>,
    /// Start velocity for simulated particles
    pub start_velocity: f32,
    /// Start color for simulated particles
    pub start_color: RangeOrFixed<Color>,
}

impl ParticleParams {
    /// Creates a [`Particle`] using current params
    ///
    /// # Arguments
    ///
    /// * `position` - The translation of the particle
    /// * `direction` - the direction of the particle
    /// * `rng`- random generator
    pub fn get_particle(&self, position: Vec3, direction: Vec3, rng: &mut impl Rng) -> Particle {
        let lifetime = self.start_lifetime.evaluate(rng);
        Particle {
            translation: position,
            rotation: self.start_rotation.evaluate(rng),
            size: self.start_size.evaluate(rng),
            lifetime,
            start_lifetime: lifetime,
            color: self.start_color.evaluate(rng),
            velocity: direction * self.start_velocity,
            angular_velocity: 0.0,
        }
    }
}

impl Default for ParticleParams {
    fn default() -> Self {
        Self {
            start_lifetime: RangeOrFixed::Fixed(5.0),
            start_size: RangeOrFixed::Fixed(1.0),
            start_rotation: RangeOrFixed::Fixed(0.0),
            start_velocity: 1.0,
            start_color: RangeOrFixed::Fixed(Color::WHITE),
        }
    }
}

use crate::particle::Particle;
use crate::RangeOrFixed;
use bevy::ecs::reflect::ReflectComponent;
use bevy::prelude::{Color, Component, Reflect, Vec3};
use rand::Rng;

/// Defines the initial state of emitted particles
#[derive(Debug, Clone, Component, Reflect)]
#[reflect(Component)]
pub struct ParticleParams {
    /// Start lifetime of simulated particles
    pub start_lifetime: RangeOrFixed<f32>,
    /// Start size of simulated particles
    pub start_size: RangeOrFixed<f32>,
    /// Start rotation for simulated particles
    pub start_rotation: RangeOrFixed<f32>,
    /// Start speed for simulated particles
    pub start_speed: RangeOrFixed<f32>,
    /// Start velocity for simulated particles
    pub start_angular_velocity: RangeOrFixed<f32>,
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
    pub fn get_particle(
        &self,
        position: Vec3,
        start_direction: Vec3,
        rng: &mut impl Rng,
    ) -> Particle {
        let lifetime = self.start_lifetime.evaluate(rng);
        Particle {
            translation: position,
            rotation: self.start_rotation.evaluate(rng),
            size: self.start_size.evaluate(rng),
            lifetime,
            start_lifetime: lifetime,
            color: self.start_color.evaluate(rng),
            velocity: start_direction * self.start_speed.evaluate(rng),
            angular_velocity: self.start_angular_velocity.evaluate(rng),
            start_direction,
        }
    }
}

impl Default for ParticleParams {
    fn default() -> Self {
        Self {
            start_lifetime: RangeOrFixed::Fixed(5.0),
            start_size: RangeOrFixed::Fixed(1.0),
            start_rotation: RangeOrFixed::Fixed(0.0),
            start_speed: RangeOrFixed::Fixed(1.0),
            start_angular_velocity: RangeOrFixed::Fixed(0.0),
            start_color: RangeOrFixed::Fixed(Color::WHITE),
        }
    }
}

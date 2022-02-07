use crate::particle::{Particle, ParticleRotation};
use crate::RangeOrFixed;
use bevy::ecs::reflect::ReflectComponent;
use bevy::prelude::{Color, Component, Reflect, Vec3};
use rand::Rng;

/// Defines the `z` rotation behaviour of particles
#[derive(Debug, Clone, Reflect)]
#[cfg_attr(feature = "inspector", derive(bevy_inspector_egui::Inspectable))]
pub enum RotationMode {
    /// The particle rotation will always align to its current direction.
    ///
    /// Note: No modifier will be able to alter the particles rotation
    AlignToDirection {
        /// Base rotation offset for rotation alignment
        offset: f32,
    },
    /// The rotation is free and you may define a start rotation value
    FreeRotation {
        /// Start rotation for simulated particles
        start_rotation: RangeOrFixed<f32>,
        /// Start angular velocity for simulated particles
        start_angular_velocity: RangeOrFixed<f32>,
    },
}

/// Defines the initial state of emitted particles
#[derive(Debug, Clone, Component, Reflect)]
#[reflect(Component)]
#[cfg_attr(feature = "inspector", derive(bevy_inspector_egui::Inspectable))]
pub struct ParticleParams {
    /// Start lifetime of simulated particles
    pub start_lifetime: RangeOrFixed<f32>,
    /// Start size of simulated particles
    pub start_size: RangeOrFixed<f32>,
    /// rotation mode for simulated particles
    pub rotation: RotationMode,
    /// Start speed for simulated particles
    pub start_speed: RangeOrFixed<f32>,
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
            rotation: match self.rotation {
                RotationMode::AlignToDirection { offset } => {
                    ParticleRotation::AlignToDirection { offset }
                }
                RotationMode::FreeRotation {
                    start_rotation,
                    start_angular_velocity,
                } => ParticleRotation::FreeRotation {
                    rotation: start_rotation.evaluate(rng),
                    angular_velocity: start_angular_velocity.evaluate(rng),
                },
            },
            size: self.start_size.evaluate(rng),
            lifetime,
            start_lifetime: lifetime,
            color: self.start_color.evaluate(rng),
            velocity: start_direction * self.start_speed.evaluate(rng),
            start_direction,
        }
    }
}

impl Default for RotationMode {
    fn default() -> Self {
        Self::FreeRotation {
            start_rotation: RangeOrFixed::Fixed(0.0),
            start_angular_velocity: RangeOrFixed::Fixed(0.0),
        }
    }
}

impl Default for ParticleParams {
    fn default() -> Self {
        Self {
            start_lifetime: RangeOrFixed::Fixed(5.0),
            start_size: RangeOrFixed::Fixed(1.0),
            rotation: RotationMode::default(),
            start_speed: RangeOrFixed::Fixed(1.0),
            start_color: RangeOrFixed::Fixed(Color::WHITE),
        }
    }
}

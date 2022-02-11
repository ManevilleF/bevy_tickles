use crate::Shape;
use bevy::prelude::{Reflect, Vec3};
use rand::Rng;
use std::fmt::Debug;

pub mod shape_enum;
/// Declares available shapes for the particle emitter
pub mod shapes;

/// Defines the direction of the particles after emission
#[derive(Debug, Copy, Clone, Reflect)]
#[cfg_attr(feature = "inspector", derive(bevy_inspector_egui::Inspectable))]
pub struct EmitterDirectionParams {
    /// Base direction mode
    pub base_mode: EmitterDirectionMode,
    /// Amount of randomization on top of the `base_mode` direction (between 0 and 1)
    #[cfg_attr(feature = "inspector", inspectable(min = 0.0, max = 1.0))]
    pub randomize_direction: f32,
    /// Amount of "spherization" on top of the `base_mode` direction (between 0 and 1)
    #[cfg_attr(feature = "inspector", inspectable(min = 0.0, max = 1.0))]
    pub spherize_direction: f32,
}

/// Different emission spread loop modes
#[derive(Debug, Copy, Clone, Reflect)]
#[cfg_attr(feature = "inspector", derive(bevy_inspector_egui::Inspectable))]
pub enum SpreadLoopMode {
    /// loops back to the start at the end of each cycle
    Loop,
    /// Each consecutive loop happens in the opposite direction to the last
    PingPong,
}

/// Defines [`EmissionMode::Spread`] parameters
#[derive(Debug, Copy, Clone, Reflect)]
#[cfg_attr(feature = "inspector", derive(bevy_inspector_egui::Inspectable))]
pub struct EmissionSpread {
    /// A value of 0 will allow particles to spawn anywhere in the volume, and a value of 0.1 will only spawn particles at 10% intervals around the shape
    #[cfg_attr(feature = "inspector", inspectable(min = 0.0, max = 1.0))]
    pub amount: f32,
    /// Lopping mode for the spread
    pub loop_mode: SpreadLoopMode,
    /// Particles will be evenly distributed in the shape
    pub uniform: bool,
    #[doc(hidden)]
    #[cfg_attr(feature = "inspector", inspectable(read_only))]
    pub current_index: f32,
}

/// Emission modes
#[derive(Debug, Copy, Clone, Reflect)]
#[cfg_attr(feature = "inspector", derive(bevy_inspector_egui::Inspectable))]
pub enum EmissionMode {
    /// Default mode, particles are placed randomly in the volume
    Random,
    /// Particles are spawned using discrete intervals in the volume
    Spread(EmissionSpread),
}

/// Defines the direction of the particles after emission
#[derive(Debug, Copy, Clone, Reflect)]
#[cfg_attr(feature = "inspector", derive(bevy_inspector_egui::Inspectable))]
pub enum EmitterDirectionMode {
    /// default, The direction is taken from the shape
    Automatic,
    /// All particles will have a fixed direction
    Fixed(Vec3),
}

#[derive(Debug, Clone)]
pub struct EmittedParticle {
    pub position: Vec3,
    pub direction: Vec3,
}

pub trait Emitter: Debug + Clone {
    fn emit_particle(
        &self,
        rng: &mut impl Rng,
        thickness: f32,
        direction_mode: EmitterDirectionMode,
    ) -> EmittedParticle;
}

/// Defines the particle emission volume and various emission option
#[derive(Debug, Clone, Reflect)]
#[cfg_attr(feature = "inspector", derive(bevy_inspector_egui::Inspectable))]
pub struct EmitterShape {
    /// The emission shape
    pub shape: Shape,
    /// The proportion of the volume that emits particles.
    /// A value of 0 emits particles from the outer surface of the shape.
    /// A value of 1 emits particles from the entire volume.
    /// Values in between will use a proportion of the volume.
    #[cfg_attr(feature = "inspector", inspectable(min = 0.0, max = 1.0))]
    pub thickness: f32,
    /// Particle direction additional parameters
    pub direction_params: EmitterDirectionParams,
    /// Emission mode
    pub mode: EmissionMode,
}

impl EmitterShape {
    pub(crate) fn emit_particle(&self, rng: &mut impl Rng) -> EmittedParticle {
        let mut particle =
            self.shape
                .emit_particle(rng, self.thickness, self.direction_params.base_mode);
        if self.direction_params.randomize_direction > 0.0 {
            let random_direction = Vec3::new(
                rng.gen_range(-1.0..=1.0),
                rng.gen_range(-1.0..=1.0),
                rng.gen_range(-1.0..=1.0),
            )
            .try_normalize()
            .unwrap_or(Vec3::Y);
            particle.direction = (random_direction * self.direction_params.randomize_direction
                + particle.direction * (1.0 - self.direction_params.randomize_direction))
                .try_normalize()
                .unwrap_or(Vec3::Y);
        }
        if self.direction_params.spherize_direction > 0.0 {
            particle.direction = (particle.position * self.direction_params.spherize_direction
                + particle.direction * (1.0 - self.direction_params.spherize_direction))
                .try_normalize()
                .unwrap_or(Vec3::Y);
        }
        particle
    }
}

impl Default for EmitterDirectionMode {
    fn default() -> Self {
        Self::Automatic
    }
}

impl Default for EmitterShape {
    fn default() -> Self {
        Self {
            shape: Default::default(),
            thickness: 1.0,
            direction_params: EmitterDirectionParams::default(),
            mode: EmissionMode::default(),
        }
    }
}

impl Default for EmissionMode {
    fn default() -> Self {
        Self::Random
    }
}

impl Default for SpreadLoopMode {
    fn default() -> Self {
        Self::Loop
    }
}

impl Default for EmissionSpread {
    fn default() -> Self {
        Self {
            amount: 0.1,
            loop_mode: Default::default(),
            uniform: false,
            current_index: 0.0,
        }
    }
}

impl Default for EmitterDirectionParams {
    fn default() -> Self {
        Self {
            base_mode: Default::default(),
            randomize_direction: 0.0,
            spherize_direction: 0.0,
        }
    }
}

pub mod shape_enum;
/// Declares available shapes for the particle emitter
pub mod shapes;

use crate::{RangeOrFixed, Shape};
use bevy::ecs::reflect::ReflectComponent;
use bevy::prelude::{Component, Reflect, Transform, Vec3};
use bevy::reflect::FromReflect;
use rand::Rng;
use std::fmt::Debug;

#[derive(Debug, Clone)]
pub struct EmittedParticle {
    pub position: Vec3,
    pub direction: Vec3,
}

/// Defines the direction of the particles after emission
#[derive(Debug, Copy, Clone, Reflect)]
#[cfg_attr(feature = "inspector", derive(bevy_inspector_egui::Inspectable))]
pub enum EmitterDirectionMode {
    /// default, The direction is taken from the shape
    Automatic,
    /// All particles will have a fixed direction
    Fixed(Vec3),
    /// All particles will have random directions
    Randomized,
}

pub trait EmitterShape: Debug + Clone {
    fn emit_particle(&self, rng: &mut impl Rng) -> EmittedParticle;
}

/// Describes a single Particle emitter burst
#[derive(Debug, Default, Clone, Reflect)]
#[cfg_attr(feature = "inspector", derive(bevy_inspector_egui::Inspectable))]
pub struct Burst {
    /// Time after the start of the emission
    pub time: f32,
    /// The count of particles to be emitted
    pub count: RangeOrFixed<usize>,
}

/// Duration of the particle emitter
#[derive(Debug, Copy, Clone, Reflect)]
#[cfg_attr(feature = "inspector", derive(bevy_inspector_egui::Inspectable))]
pub enum EmitterDuration {
    /// No duration limit
    Infinite,
    /// Fixed duration limit
    FixedDuration {
        /// Max emission duration
        duration: f32,
        /// Does the emitter loop back and restarts?
        looping: bool,
    },
}

/// Emitter of particles, works with [`ParticleSystem`]
#[derive(Debug, Clone, Component, Reflect)]
#[reflect(Component)]
#[cfg_attr(feature = "inspector", derive(bevy_inspector_egui::Inspectable))]
pub struct ParticleEmitter {
    /// Emitter duration
    pub duration: EmitterDuration,
    /// The shape of the emitter
    pub shape: Shape,
    /// The rate of particle emission over time (`1.0` means 1 particle per second)
    pub rate: f32,
    /// Custom bursts of particle emission
    pub bursts: Vec<Burst>,
    /// time since first tick
    pub current_delta_time: f32,
    /// Time since last `rate` emission
    pub last_emitted_delta_time: f32,
    /// The shape transform
    pub transform: Transform,
    /// Particle directions after emission
    pub direction_mode: EmitterDirectionMode,
}

impl FromReflect for Burst {
    fn from_reflect(reflect: &dyn Reflect) -> Option<Self> {
        reflect.any().downcast_ref::<Self>().cloned()
    }
}

impl Default for EmitterDuration {
    fn default() -> Self {
        Self::Infinite
    }
}

impl Default for EmitterDirectionMode {
    fn default() -> Self {
        Self::Automatic
    }
}

impl Default for ParticleEmitter {
    fn default() -> Self {
        Self {
            duration: Default::default(),
            shape: Default::default(),
            rate: 5.0,
            bursts: vec![],
            current_delta_time: 0.0,
            last_emitted_delta_time: 0.0,
            transform: Default::default(),
            direction_mode: Default::default(),
        }
    }
}

impl ParticleEmitter {
    /// Computes particles to emit
    pub fn emit(&mut self, delta_time: f32, rng: &mut impl Rng) -> Vec<EmittedParticle> {
        // Check duration
        if let EmitterDuration::FixedDuration { duration, looping } = self.duration {
            if self.current_delta_time > duration {
                if looping {
                    self.current_delta_time = 0.0;
                    self.last_emitted_delta_time = 0.0;
                }
                return vec![];
            }
        }
        // bursts
        let mut emission_count = self
            .bursts
            .iter()
            .filter(|b| {
                b.time >= self.current_delta_time && b.time < self.current_delta_time + delta_time
            })
            .map(|b| b.count.evaluate(rng))
            .sum();
        self.current_delta_time += delta_time;
        // emission over time
        if self.rate > 0.0 {
            let delta_per_particle = 1.0 / self.rate;
            let delay_since_emission = self.current_delta_time - self.last_emitted_delta_time;
            let particles_to_emit = (delay_since_emission / delta_per_particle) as usize;
            emission_count += particles_to_emit;
            self.last_emitted_delta_time += delta_per_particle * particles_to_emit as f32;
        }

        let matrix = self.transform.compute_matrix();
        (0..emission_count)
            .map(|_| {
                let mut particle = self.shape.emit_particle(rng);
                particle.position = matrix.transform_point3(particle.position);
                match self.direction_mode {
                    EmitterDirectionMode::Automatic => (),
                    EmitterDirectionMode::Fixed(dir) => {
                        particle.direction = dir;
                    }
                    EmitterDirectionMode::Randomized => {
                        particle.direction = Vec3::new(
                            rng.gen_range(-1.0..=1.0),
                            rng.gen_range(-1.0..=1.0),
                            rng.gen_range(-1.0..=1.0),
                        );
                    }
                }
                particle.direction =
                    matrix.transform_point3(particle.direction.try_normalize().unwrap_or(Vec3::Y));
                particle
            })
            .collect()
    }
}

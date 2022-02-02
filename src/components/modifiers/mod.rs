mod constraints;
mod gravity;
mod size;
mod velocity;

use crate::ParticleSystem;
use bevy::prelude::Component;

use crate::particle::Particle;
pub use {
    constraints::{MaxParticleCount, MaxParticleSize, MaxParticleSpeed},
    gravity::ParticleGravity,
    size::{SizeOverSpeed, SizeOverTime},
    velocity::{AngularVelocityOverTime, VelocityOverTime},
};

/// Common trait for particle system modifiers
pub trait ParticleSystemModifier: Component {
    /// Applies modification to the particle system
    fn apply(&self, particles: &mut ParticleSystem, delta_time: f32);
}

/// Common trait for particle modifiers
pub trait ParticleModifier: Component {
    /// Applies modification to the particle
    fn apply(&self, particle: &mut Particle, delta_time: f32);
}

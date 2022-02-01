mod color_modifiers;
mod max_particle_count;

use crate::ParticleSystem;
use bevy::prelude::Component;

pub use {color_modifiers::*, max_particle_count::*};

/// Common trait for particle system modifiers
pub trait ParticleSystemModifier: Component {
    /// Applies modification to the particle system
    fn apply(&self, particles: &mut ParticleSystem, delta_time: f32);
}

use crate::{ParticleEmitter, ParticleSystem};
use bevy::prelude::{Bundle, GlobalTransform, Handle, Image, Transform};

/// Particle System bundle
#[derive(Debug, Clone, Default, Bundle)]
pub struct ParticleSystemBundle {
    /// The main particle system component
    pub particle_system: ParticleSystem,
    /// The particle emitter component
    pub particle_emitter: ParticleEmitter,
    /// The entity local translation/rotation/scale
    pub transform: Transform,
    /// The entity global translation/rotation/scale
    pub global_transform: GlobalTransform,
    /// The texture for the particles
    pub image: Handle<Image>,
}

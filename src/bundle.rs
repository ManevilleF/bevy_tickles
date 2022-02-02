use crate::{ParticleEmitter, ParticleMaterial, ParticleParams, ParticleRng, ParticleSystem};
use bevy::prelude::{Bundle, GlobalTransform, Transform, Visibility};

/// Particle System bundle
#[derive(Debug, Clone, Default, Bundle)]
pub struct ParticleSystemBundle {
    /// The main particle system component
    pub particle_system: ParticleSystem,
    /// The particle emitter component
    pub particle_emitter: ParticleEmitter,
    /// The particle params component
    pub particle_params: ParticleParams,
    /// The particle randomizer component
    pub particle_rng: ParticleRng,
    /// The entity local translation/rotation/scale
    pub transform: Transform,
    /// The entity global translation/rotation/scale
    pub global_transform: GlobalTransform,
    /// The visual for the particles
    pub material: ParticleMaterial,
    /// Particle visibility
    pub visibility: Visibility,
}

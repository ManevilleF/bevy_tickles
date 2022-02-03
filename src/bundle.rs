use crate::{
    AngularVelocityOverTime, ColorOverLifeTime, MaxParticleCount, MaxParticleSize,
    MaxParticleSpeed, ParticleEmitter, ParticleGravity, ParticleMaterial, ParticleParams,
    ParticleRng, ParticleSystem, SizeOverSpeed, SizeOverTime, VelocityOverTime,
};
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

/// Exhaustive Particle System bundle containing all built-in modifiers
#[derive(Debug, Clone, Default, Bundle)]
pub struct ExhaustiveParticleSystemBundle {
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
    pub max_particles: MaxParticleCount,
    pub max_particle_size: MaxParticleSize,
    pub max_particle_speed: MaxParticleSpeed,
    pub velocity_over_time: VelocityOverTime,
    pub angular_velocity_over_time: AngularVelocityOverTime,
    pub gravity: ParticleGravity,
    pub size_over_time: SizeOverTime,
    pub size_over_speed: SizeOverSpeed,
    pub color_over_lifetime: ColorOverLifeTime,
}

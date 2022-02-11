/// Modifier components for particles
pub mod modifiers;
/// Particle Emission logic
mod particle_emitter;
mod particle_material;
mod particle_params;
mod particle_render_mode;
mod particle_rng;
mod particle_system;

pub use {
    particle_emitter::{
        emitter_shape::{
            shape_enum::Shape, shapes, EmissionSpread, EmitterDirectionMode,
            EmitterDirectionParams, EmitterShape, SpreadLoopMode,
        },
        Burst, EmitterDuration, ParticleEmitter,
    },
    particle_material::{
        ParticleMaterial, ParticleTextureSheet, TextureSheetAnimation, TextureSheetLoopingMode,
        TextureSheetMode,
    },
    particle_params::{ParticleParams, RotationMode},
    particle_render_mode::{BillBoardAlignment, ParticleRenderMode},
    particle_rng::ParticleRng,
    particle_system::ParticleSystem,
};

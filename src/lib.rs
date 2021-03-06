//! # Bevy Tickles
//!
//! [![workflow](https://github.com/ManevilleF/bevy_tickles/actions/workflows/rust.yml/badge.svg)](https://github.com/ManevilleF/bevy_tickles/actions/workflows/rust.yml)
//!
//! [![MIT licensed](https://img.shields.io/badge/license-MIT-blue.svg)](./LICENSE)
//! [![unsafe forbidden](https://img.shields.io/badge/unsafe-forbidden-success.svg)](https://github.com/rust-secure-code/safety-dance/)
//!
//! CPU based Particle systems plugin for [bevy](https://bevyengine.org) inspired by `Unity3D` *shuriken* particle system.
//!
//! > This is a work in progress with many missing features, it is not suitable for production.
//! > As this lib is in very early stage, expect the API to change often

#![forbid(unsafe_code)]
#![warn(
    rustdoc::broken_intra_doc_links,
    clippy::nursery,
    missing_docs,
    clippy::pedantic,
    clippy::cargo
)]
#![allow(
    clippy::default_trait_access,
    clippy::module_name_repetitions,
    clippy::cast_sign_loss,
    clippy::cast_possible_truncation,
    clippy::cast_precision_loss,
    clippy::multiple_crate_versions
)]

extern crate core;

mod bundle;
/// Particle system components
pub mod components;
mod particle;
mod render;
mod systems;
/// Utility structs
pub mod utilities;

use crate::render::draw::DrawParticle;
use crate::render::pipeline::{ParticlePipeline, PARTICLE_SHADER_HANDLE};
use crate::render::{ExtractedParticles, ParticleImageBindGroups, ParticleMeta};
use bevy::core_pipeline::Transparent3d;
use bevy::log;
use bevy::prelude::*;
use bevy::render::{
    render_phase::AddRenderCommand, render_resource::SpecializedRenderPipelines, RenderApp,
    RenderStage,
};
#[cfg(feature = "inspector")]
use bevy_inspector_egui::RegisterInspectable;

///
pub mod prelude {
    pub use crate::bundle::ParticleSystemBundle;
    pub use crate::components::*;
    pub use crate::particle::Particle;
    pub use crate::utilities::*;
    pub use crate::ParticlesPlugin;
}

use crate::modifiers::{
    AngularVelocityOverTime, ColorOverLifeTime, ColorOverSpeed, LinearVelocityOverLifeTime,
    MaxParticleCount, MaxParticleSize, MaxParticleSpeed, OrbitalVelocityOverLifeTime,
    ParticleGravity, PerlinNoise, RotationOverTime, RotationOverVelocity, SizeOverSpeed,
    SizeOverTime, SpeedOverTime, VelocityOverTime,
};
use prelude::*;
use systems::{apply_modifier, apply_rng_modifier, apply_system_modifier};

const PARTICLE_UPDATE: &str = "particle_update";
const PARTICLE_EMISSION: &str = "particle_emission";

/// Particle System plugin
pub struct ParticlesPlugin;

/// Particle render system labels
#[derive(Debug, Hash, PartialEq, Eq, Clone, SystemLabel)]
pub enum ParticleLabel {
    /// Extraction system
    ExtractParticles,
    /// Prepare system
    PrepareParticles,
    /// Queue system
    QueueParticles,
}

impl Plugin for ParticlesPlugin {
    fn build(&self, app: &mut App) {
        app.register_type::<ParticleGravity>()
            .register_type::<MaxParticleSpeed>()
            .register_type::<MaxParticleCount>()
            .register_type::<MaxParticleSize>()
            .register_type::<ParticleMaterial>()
            .register_type::<ParticleParams>()
            .register_type::<ParticleEmitter>()
            .register_type::<ParticleSystem>()
            .register_type::<ParticleRenderMode>()
            .register_type::<ColorGradient>();
        #[cfg(feature = "inspector")]
        app.init_resource::<bevy_inspector_egui::InspectableRegistry>()
            .register_inspectable::<RotationMode>()
            .register_inspectable::<ParticleMaterial>()
            .register_inspectable::<ParticleSystem>()
            .register_inspectable::<ParticleRenderMode>()
            .register_inspectable::<ParticleEmitter>()
            .register_inspectable::<ColorOverLifeTime>()
            .register_inspectable::<ColorOverSpeed>()
            .register_inspectable::<RangeOrFixed<f32>>()
            .register_inspectable::<RangeOrFixed<usize>>()
            .register_inspectable::<AngularVelocityOverTime>()
            .register_inspectable::<SpeedOverTime>()
            .register_inspectable::<VelocityOverTime>()
            .register_inspectable::<OrbitalVelocityOverLifeTime>()
            .register_inspectable::<LinearVelocityOverLifeTime>()
            .register_inspectable::<ParticleGravity>()
            .register_inspectable::<SizeOverTime>()
            .register_inspectable::<SizeOverSpeed>()
            .register_inspectable::<RotationOverVelocity>()
            .register_inspectable::<RotationOverTime>()
            .register_inspectable::<PerlinNoise>()
            .register_inspectable::<ColorOrGradient>();

        app.add_system(systems::update_particle_system.label(PARTICLE_UPDATE))
            .add_system(systems::emit_particles.label(PARTICLE_EMISSION))
            .add_system(systems::compute_particles_aabb.after(PARTICLE_UPDATE))
            // TODO: merge all systems in one to avoid so many queries
            .add_system(apply_system_modifier::<MaxParticleCount>.after(PARTICLE_EMISSION))
            .add_system(apply_modifier::<MaxParticleSize>.after(PARTICLE_EMISSION))
            .add_system(apply_modifier::<ParticleGravity>.after(PARTICLE_UPDATE))
            .add_system(apply_modifier::<MaxParticleSpeed>.after(PARTICLE_UPDATE))
            .add_system(apply_modifier::<SpeedOverTime>.after(PARTICLE_UPDATE))
            .add_system(apply_modifier::<VelocityOverTime>.after(PARTICLE_UPDATE))
            .add_system(apply_modifier::<LinearVelocityOverLifeTime>.after(PARTICLE_UPDATE))
            .add_system(apply_modifier::<OrbitalVelocityOverLifeTime>.after(PARTICLE_UPDATE))
            .add_system(apply_modifier::<AngularVelocityOverTime>.after(PARTICLE_UPDATE))
            .add_system(apply_modifier::<SizeOverTime>.after(PARTICLE_UPDATE))
            .add_system(apply_modifier::<SizeOverSpeed>.after(PARTICLE_UPDATE))
            .add_system(apply_modifier::<RotationOverVelocity>.after(PARTICLE_UPDATE))
            .add_system(apply_modifier::<RotationOverTime>.after(PARTICLE_UPDATE))
            .add_system(apply_modifier::<ColorOverLifeTime>.after(PARTICLE_UPDATE))
            .add_system(apply_modifier::<ColorOverSpeed>.after(PARTICLE_UPDATE))
            .add_system(apply_rng_modifier::<PerlinNoise>.after(PARTICLE_UPDATE));

        let mut shaders = app
            .world
            .get_resource_mut::<Assets<Shader>>()
            .expect("Could not load the `Assets<Shader>` resource from the world");
        let particle_shader = Shader::from_wgsl(include_str!("render/particles.wgsl"));
        shaders.set_untracked(PARTICLE_SHADER_HANDLE, particle_shader);
        if let Ok(render_app) = app.get_sub_app_mut(RenderApp) {
            render_app
                .init_resource::<ParticleImageBindGroups>()
                .init_resource::<ParticlePipeline>()
                .init_resource::<SpecializedRenderPipelines<ParticlePipeline>>()
                .init_resource::<ParticleMeta>()
                .init_resource::<ExtractedParticles>()
                .add_render_command::<Transparent3d, DrawParticle>()
                .add_system_to_stage(
                    RenderStage::Extract,
                    render::extract::extract_particles.label(ParticleLabel::ExtractParticles),
                )
                .add_system_to_stage(
                    RenderStage::Prepare,
                    render::prepare::prepare_particles.label(ParticleLabel::PrepareParticles),
                )
                .add_system_to_stage(
                    RenderStage::Queue,
                    render::queue::queue_particles.label(ParticleLabel::QueueParticles),
                );
        };
        log::info!("Loaded Particles Plugin");
    }
}

//! # bevy particles
//!
//! Particle Systems in bevy
//!
//! ## TODO:
//!
//! - [ ] Working computed visibility with AAB
//! - [ ] Complete modifier list
//! - [ ] Sub Emitters and callbacks (trails/death)
//! - [ ] Curves and color gradients

#![forbid(unsafe_code)]
#![warn(
    broken_intra_doc_links,
    clippy::nursery,
    clippy::pedantic,
    missing_docs
)]
#![allow(clippy::default_trait_access, clippy::module_name_repetitions)]

mod bundle;
mod components;
mod particle;
mod render;
mod systems;
mod utilities;

use bevy::core_pipeline::Transparent3d;
pub use bundle::*;
pub use components::*;
pub use particle::Particle;
pub use utilities::*;

use crate::modifiers::*;
use crate::render::draw::DrawParticle;
use crate::render::pipeline::{ParticlePipeline, PARTICLE_SHADER_HANDLE};
use crate::render::{ExtractedParticles, ParticleImageBindGroups, ParticleMeta};
use bevy::log;
use bevy::prelude::*;
use bevy::render::{
    render_phase::AddRenderCommand, render_resource::SpecializedPipelines, RenderApp, RenderStage,
};
#[cfg(feature = "inspector")]
use bevy_inspector_egui::RegisterInspectable;

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
            .register_type::<ParticleSystem>();
        #[cfg(feature = "inspector")]
        app.init_resource::<bevy_inspector_egui::InspectableRegistry>()
            .register_inspectable::<ParticleMaterial>()
            .register_inspectable::<ParticleEmitter>()
            .register_inspectable::<ColorOverLifeTime>()
            .register_inspectable::<RangeOrFixed<f32>>()
            .register_inspectable::<RangeOrFixed<usize>>()
            .register_inspectable::<RangeOrFixed<Color>>()
            .register_inspectable::<AngularVelocityOverTime>()
            .register_inspectable::<VelocityOverTime>()
            .register_inspectable::<ParticleGravity>()
            .register_inspectable::<SizeOverSpeed>()
            .register_inspectable::<SizeOverTime>();

        app.add_system(systems::update_particle_system.label(PARTICLE_UPDATE))
            .add_system(systems::emit_particles.label(PARTICLE_EMISSION))
            // TODO: merge all systems in one to avoid so many queries
            .add_system(systems::apply_system_modifier::<MaxParticleCount>.after(PARTICLE_EMISSION))
            .add_system(systems::apply_modifier::<MaxParticleSize>.after(PARTICLE_EMISSION))
            .add_system(systems::apply_modifier::<ParticleGravity>.after(PARTICLE_UPDATE))
            .add_system(systems::apply_modifier::<MaxParticleSpeed>.after(PARTICLE_UPDATE))
            .add_system(systems::apply_modifier::<VelocityOverTime>.after(PARTICLE_UPDATE))
            .add_system(systems::apply_modifier::<AngularVelocityOverTime>.after(PARTICLE_UPDATE))
            .add_system(systems::apply_modifier::<SizeOverTime>.after(PARTICLE_UPDATE))
            .add_system(systems::apply_modifier::<SizeOverSpeed>.after(PARTICLE_UPDATE))
            .add_system(systems::apply_modifier::<ColorOverLifeTime>.after(PARTICLE_UPDATE));

        let mut shaders = app.world.get_resource_mut::<Assets<Shader>>().unwrap();
        let particle_shader = Shader::from_wgsl(include_str!("render/particles.wgsl"));
        shaders.set_untracked(PARTICLE_SHADER_HANDLE, particle_shader);
        if let Ok(render_app) = app.get_sub_app_mut(RenderApp) {
            render_app
                .init_resource::<ParticleImageBindGroups>()
                .init_resource::<ParticlePipeline>()
                .init_resource::<SpecializedPipelines<ParticlePipeline>>()
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

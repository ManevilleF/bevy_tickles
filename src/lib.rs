//! # bevy particles
//!
//! Particle Systems in bevy
//!
//! ## TODO:
//!
//! - [ ] Working computed visibility with AAB
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

pub use bundle::ParticleSystemBundle;
pub use components::*;
pub use particle::Particle;
pub use utilities::*;

use crate::modifiers::*;
use crate::render::extract::ExtractedParticles;
use bevy::log;
use bevy::prelude::*;
use bevy::render::{RenderApp, RenderStage};

const PARTICLE_UPDATE: &str = "particle_update";
const PARTICLE_EMISSION: &str = "particle_emission";

/// Particle System plugin
pub struct ParticlesPlugin;

/// Particle render system labels
#[derive(Debug, Hash, PartialEq, Eq, Clone, SystemLabel)]
pub enum ParticleLabel {
    /// Extraction system
    ExtractParticles,
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
            .add_system(systems::apply_modifier::<SizeOverSpeed>.after(PARTICLE_UPDATE));
        if let Ok(render_app) = app.get_sub_app_mut(RenderApp) {
            render_app
                // .init_resource::<ImageBindGroups>()
                // .init_resource::<ParticlesPipeline>()
                // .init_resource::<SpecializedPipelines<ParticlesPipeline>>()
                // .init_resource::<ParticleMeta>()
                .init_resource::<ExtractedParticles>()
                // .add_render_command::<Transparent2d, DrawParticle>()
                .add_system_to_stage(
                    RenderStage::Extract,
                    render::extract::extract_particles.label(ParticleLabel::ExtractParticles),
                );
            // .add_system_to_stage(
            //     RenderStage::Queue,
            //     render::queue::queue_particles.label(ParticlesSystem::QueueParticles),
            // )
        };
        log::info!("Loaded Particles Plugin");
    }
}

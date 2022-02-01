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

pub use bundle::ParticleSystemBundle;
pub use components::*;

use bevy::log;
use bevy::prelude::*;

const PARTICLE_UPDATE: &str = "particle_update";

/// Particle System plugin
pub struct ParticlesPlugin;

impl Plugin for ParticlesPlugin {
    fn build(&self, app: &mut App) {
        log::info!("Loaded Particles Plugin");
    }
}

use bevy::prelude::*;
use bevy_inspector_egui::WorldInspectorPlugin;
use bevy_particles::{ParticleSystemBundle, ParticlesPlugin};

fn main() {
    App::new()
        .add_plugins(DefaultPlugins)
        .add_plugin(ParticlesPlugin)
        .add_plugin(WorldInspectorPlugin::default())
        .add_startup_system(spawn_particle_system)
        .run();
}

fn spawn_particle_system(mut commands: Commands) {
    commands.spawn_bundle(ParticleSystemBundle::default());
}

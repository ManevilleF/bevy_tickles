use bevy::prelude::*;
use bevy_inspector_egui::WorldInspectorPlugin;
use bevy_particles::{ExhaustiveParticleSystemBundle, ParticlesPlugin};

fn main() {
    App::new()
        .add_plugins(DefaultPlugins)
        .add_plugin(ParticlesPlugin)
        .add_plugin(WorldInspectorPlugin::default())
        .add_startup_system(spawn_particle_system)
        .run();
}

fn spawn_particle_system(mut commands: Commands) {
    commands.spawn_bundle(PerspectiveCameraBundle {
        transform: Transform::from_xyz(10.0, 0.0, 10.0).looking_at(Vec3::ZERO, Vec3::Z),
        ..PerspectiveCameraBundle::new_3d()
    });
    commands.spawn_bundle(DirectionalLightBundle::default());
    commands
        .spawn_bundle(ExhaustiveParticleSystemBundle::default())
        .insert(Name::new("Particle System"));
}

use bevy::prelude::*;
use bevy_inspector_egui::WorldInspectorPlugin;
use bevy_particles::{ExhaustiveParticleSystemBundle, ParticleMaterial, ParticlesPlugin};

fn main() {
    App::new()
        .add_plugins(DefaultPlugins)
        .add_plugin(ParticlesPlugin)
        .add_plugin(WorldInspectorPlugin::default())
        .add_startup_system(spawn_particle_system)
        .run();
}

fn spawn_particle_system(mut commands: Commands, asset_server: Res<AssetServer>) {
    commands.spawn_bundle(PerspectiveCameraBundle {
        transform: Transform::from_xyz(10.0, 0.0, 10.0).looking_at(Vec3::ZERO, Vec3::Z),
        ..PerspectiveCameraBundle::new_3d()
    });
    commands.spawn_bundle(DirectionalLightBundle::default());
    commands
        .spawn_bundle(ExhaustiveParticleSystemBundle {
            material: ParticleMaterial::Image(asset_server.load("wrench.png")),
            ..Default::default()
        })
        .insert(Name::new("Particle System"));
    commands.spawn_bundle(PbrBundle::default());
}

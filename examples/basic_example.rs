use bevy::prelude::*;
use bevy_inspector_egui::WorldInspectorPlugin;
use bevy_particles::prelude::modifiers::*;
use bevy_particles::prelude::*;

fn main() {
    App::new()
        .add_plugins(DefaultPlugins)
        .add_plugin(ParticlesPlugin)
        .add_plugin(WorldInspectorPlugin::default())
        .add_startup_system(spawn_particle_system)
        .add_startup_system(spawn_cubes)
        .run();
}

fn spawn_particle_system(mut commands: Commands, asset_server: Res<AssetServer>) {
    commands.spawn_bundle(PerspectiveCameraBundle {
        transform: Transform::from_xyz(10.0, 10.0, 10.0).looking_at(Vec3::ZERO, Vec3::Y),
        ..PerspectiveCameraBundle::new_3d()
    });
    commands.spawn_bundle(DirectionalLightBundle::default());
    commands
        .spawn_bundle(ParticleSystemBundle {
            transform: Transform::from_xyz(0., 5., 0.),
            material: ParticleMaterial::Image(asset_server.load("wrench.png")),
            particle_params: ParticleParams {
                rotation: RotationMode::FreeRotation {
                    start_rotation: RangeOrFixed::Range {
                        min: -6.0,
                        max: 6.0,
                    },
                    start_angular_velocity: Default::default(),
                },
                start_size: 0.0.into(),
                start_speed: 5.0.into(),
                ..Default::default()
            },
            particle_emitter: ParticleEmitter {
                rate: 20.0,
                shape: EmitterShape::Sphere {
                    radius: 0.2,
                    edge_only: false,
                },
                ..Default::default()
            },
            ..Default::default()
        })
        .insert(ColorOverLifeTime(ColorGradient::from((
            Color::WHITE,
            Color::rgba(0.5, 0.5, 1.0, 0.0),
        ))))
        .insert(SizeOverTime(0.5))
        .insert(ParticleGravity(Vec3::new(0., -1.5, 0.)))
        .insert(AngularVelocityOverTime(1.0))
        .insert(Name::new("Particle System"));
}

fn spawn_cubes(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<StandardMaterial>>,
) {
    let mesh = meshes.add(shape::Cube::new(1.).into());
    commands.spawn_bundle(PbrBundle {
        mesh: mesh.clone(),
        material: materials.add(Color::WHITE.into()),
        transform: Transform::from_xyz(0., 0., 0.),
        ..Default::default()
    });
    commands.spawn_bundle(PbrBundle {
        mesh: mesh.clone(),
        material: materials.add(Color::RED.into()),
        transform: Transform::from_xyz(-5., 0., 0.),
        ..Default::default()
    });
    commands.spawn_bundle(PbrBundle {
        mesh: mesh.clone(),
        material: materials.add(Color::BLUE.into()),
        transform: Transform::from_xyz(5., 0., 0.),
        ..Default::default()
    });
    commands.spawn_bundle(PbrBundle {
        mesh: mesh.clone(),
        material: materials.add(Color::GREEN.into()),
        transform: Transform::from_xyz(0., 0., 5.),
        ..Default::default()
    });
    commands.spawn_bundle(PbrBundle {
        mesh,
        material: materials.add(Color::YELLOW.into()),
        transform: Transform::from_xyz(0., 0., -5.),
        ..Default::default()
    });
}

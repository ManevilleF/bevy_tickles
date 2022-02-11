use bevy::prelude::shape::Torus;
use bevy::prelude::*;
use bevy_inspector_egui::WorldInspectorPlugin;
use bevy_tickles::prelude::modifiers::*;
use bevy_tickles::prelude::shapes::ConvexMesh;
use bevy_tickles::prelude::*;

fn main() {
    App::new()
        .add_plugins(DefaultPlugins)
        .add_plugin(ParticlesPlugin)
        .add_plugin(WorldInspectorPlugin::default())
        .add_startup_system(spawn_cubes)
        .add_startup_system(init)
        .add_startup_system(spawn_particle_system)
        .run();
}

fn init(mut commands: Commands) {
    commands.spawn_bundle(PerspectiveCameraBundle {
        transform: Transform::from_xyz(10.0, 10.0, 10.0).looking_at(Vec3::ZERO, Vec3::Y),
        ..PerspectiveCameraBundle::new_3d()
    });
    commands.spawn_bundle(DirectionalLightBundle::default());
}

fn spawn_particle_system(mut commands: Commands, asset_server: Res<AssetServer>) {
    commands
        .spawn_bundle(ParticleSystemBundle {
            material: ParticleMaterial::Image(asset_server.load("wrench.png")),
            particle_params: ParticleParams {
                start_size: 1.0.into(),
                start_speed: 0.0.into(),
                rotation: RotationMode::AlignToDirection { offset: -1.8 },
                ..Default::default()
            },
            particle_emitter: ParticleEmitter {
                rate: 100.0,
                shape: EmitterShape {
                    shape: Shape::ConvexMesh(ConvexMesh(Mesh::from(Torus {
                        radius: 5.0,
                        ring_radius: 2.0,
                        subdivisions_segments: 30,
                        subdivisions_sides: 10,
                    }))),
                    thickness: 0.0,
                    ..Default::default()
                },
                ..Default::default()
            },
            ..Default::default()
        })
        .insert(ColorOverLifeTime(
            ColorGradient::empty()
                .add_point(0.0, Color::rgba(1.0, 1.0, 1.0, 0.0))
                .add_point(0.1, Color::WHITE)
                .add_point(0.9, Color::WHITE)
                .add_point(1.0, Color::rgba(1.0, 1.0, 1.0, 0.0)),
        ))
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

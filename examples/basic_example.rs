use bevy::prelude::*;
use bevy_inspector_egui::WorldInspectorPlugin;
use bevy_tickles::prelude::modifiers::*;
use bevy_tickles::prelude::shapes::{Circle, Sphere};
use bevy_tickles::prelude::*;

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
                rotation: RotationMode::AlignToDirection { offset: -1.8 },
                start_size: 0.0.into(),
                start_speed: 0.0.into(),
                ..Default::default()
            },
            particle_emitter: ParticleEmitter {
                rate: 20.0,
                shape: EmitterShape {
                    shape: Shape::Circle(Circle {
                        radius: 2.0,
                        ..Default::default()
                    }),
                    thickness: 1.0,
                    direction_params: Default::default(),
                    mode: EmissionMode::Spread(EmissionSpread {
                        spreads: [
                            AxisSpread::default(), // Ignored for circles
                            AxisSpread {
                                amount: 0.05,
                                uniform: true,
                                ..Default::default()
                            },
                            AxisSpread {
                                amount: 0.1,
                                uniform: true,
                                loop_mode: SpreadLoopMode::PingPong,
                            },
                        ],
                        ..Default::default()
                    }),
                },
                ..Default::default()
            },
            ..Default::default()
        })
        .insert(ColorOverLifeTime(
            ColorGradient::empty()
                .add_point(0.0, Color::WHITE)
                .add_point(0.3, Color::ORANGE)
                .add_point(0.5, Color::GREEN)
                .add_point(1.0, Color::rgba(0.5, 0.5, 1.0, 0.0)),
        ))
        .insert(SizeOverTime(0.5))
        .insert(ParticleGravity(Vec3::new(0., -1.5, 0.)))
        .insert(AngularVelocityOverTime(1.0))
        .insert(OrbitalVelocityOverLifeTime::default())
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

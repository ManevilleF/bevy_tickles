use bevy::prelude::*;
use bevy_flycam::*;
use bevy_inspector_egui::WorldInspectorPlugin;
use bevy_tickles::components::modifiers::ParticleGravity;
use bevy_tickles::prelude::modifiers::{ColorOverLifeTime, RotationOverVelocity, SizeOverTime};
use bevy_tickles::prelude::shapes::{Circle, Sphere};
use bevy_tickles::prelude::*;

fn main() {
    App::new()
        .add_plugins(DefaultPlugins)
        .add_plugin(ParticlesPlugin)
        .add_plugin(WorldInspectorPlugin::default())
        .add_plugin(NoCameraPlayerPlugin)
        .add_startup_system(spawn_particle_system)
        .add_startup_system(spawn_cubes)
        .run();
}

fn spawn_particle_system(
    mut commands: Commands,
    asset_server: Res<AssetServer>,
    mut atlases: ResMut<Assets<TextureAtlas>>,
) {
    commands
        .spawn_bundle(PerspectiveCameraBundle {
            transform: Transform::from_xyz(0.0, 5.0, 20.0).looking_at(Vec3::ZERO, Vec3::Y),
            ..PerspectiveCameraBundle::new_3d()
        })
        .insert(FlyCam);
    commands.spawn_bundle(DirectionalLightBundle::default());
    let smoke_texture = asset_server.load("smoke.png");
    let explosion_texture = asset_server.load("explosion.png");
    let explosion_texture_atlas = atlases.add(TextureAtlas::from_grid(
        explosion_texture,
        Vec2::new(256.0, 256.0),
        8,
        6,
    ));
    let duration = EmitterDuration::FixedDuration {
        duration: 5.0,
        looping: true,
    };
    commands
        .spawn_bundle(ParticleSystemBundle {
            transform: Transform::from_xyz(0., 1., 0.),
            material: ParticleTextureSheet {
                texture_atlas: explosion_texture_atlas.clone(),
                mode: TextureSheetMode::AnimateOverLifetime(TextureSheetAnimation {
                    start_index: 0,
                    looping_mode: TextureSheetLoopingMode::None,
                    ..Default::default()
                }),
            }
            .into(),
            particle_params: ParticleParams {
                start_size: 10.0.into(),
                start_speed: 0.0.into(),
                start_lifetime: 1.0.into(),
                ..Default::default()
            },
            particle_emitter: ParticleEmitter {
                rate: 0.0,
                shape: Shape::Sphere(Sphere {
                    radius: 0.0,
                    ..Default::default()
                }),
                bursts: vec![Burst {
                    time: 0.0,
                    count: RangeOrFixed::Fixed(1),
                }],
                duration,
                ..Default::default()
            },
            ..Default::default()
        })
        .insert(Name::new("Explosion System"));
    commands
        .spawn_bundle(ParticleSystemBundle {
            transform: Transform::from_xyz(0., 0., 0.),
            material: ParticleMaterial::Image(smoke_texture),
            particle_params: ParticleParams {
                start_size: 0.1.into(),
                start_speed: 6.0.into(),
                start_lifetime: 2.0.into(),
                rotation: RotationMode::FreeRotation {
                    start_rotation: (-6.0..=6.0).into(),
                    start_angular_velocity: 0.0.into(),
                },
                ..Default::default()
            },
            particle_emitter: ParticleEmitter {
                rate: 0.0,
                shape: Shape::Circle(Circle {
                    radius: 1.0,
                    thickness: 0.0,
                }),
                bursts: vec![Burst {
                    time: 0.0,
                    count: RangeOrFixed::Fixed(50),
                }],
                duration,
                ..Default::default()
            },
            ..Default::default()
        })
        .insert(ColorOverLifeTime(
            ColorGradient::empty()
                .add_point(0.0, Color::NONE)
                .add_point(0.5, Color::WHITE)
                .add_point(1.0, Color::NONE),
        ))
        .insert(SizeOverTime(1.5))
        .insert(RotationOverVelocity {
            value: 1.0,
            abs: false,
        })
        .insert(Name::new("Smoke System"));
    commands
        .spawn_bundle(ParticleSystemBundle {
            transform: Transform::from_xyz(0., 0., 0.),
            material: ParticleTextureSheet {
                texture_atlas: explosion_texture_atlas,
                mode: TextureSheetMode::FixedIndex(3),
            }
            .into(),
            particle_params: ParticleParams {
                start_size: 1.0.into(),
                start_speed: (5.0..=6.0).into(),
                start_lifetime: 3.0.into(),
                rotation: RotationMode::AlignToDirection { offset: 0.0 },
                ..Default::default()
            },
            particle_emitter: ParticleEmitter {
                rate: 0.0,
                shape: Shape::Sphere(Sphere {
                    radius: 2.0,
                    hemisphere: true,
                    thickness: 0.0,
                }),
                bursts: vec![Burst {
                    time: 0.0,
                    count: RangeOrFixed::Fixed(50),
                }],
                duration,
                ..Default::default()
            },
            ..Default::default()
        })
        .insert(ColorOverLifeTime(
            ColorGradient::empty()
                .add_point(0.0, Color::WHITE)
                .add_point(1.0, Color::NONE),
        ))
        .insert(ParticleGravity(Vec3::Y * -1.5))
        .insert(Name::new("Fire balls System"));
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

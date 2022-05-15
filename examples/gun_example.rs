use bevy::prelude::*;
use bevy_flycam::*;
use bevy_inspector_egui::WorldInspectorPlugin;
use bevy_tickles::components::modifiers::ColorOverLifeTime;
use bevy_tickles::components::shapes::Cone;
use bevy_tickles::prelude::modifiers::{ParticleGravity, SizeOverTime};
use bevy_tickles::prelude::*;

fn main() {
    App::new()
        .add_plugins(DefaultPlugins)
        .add_plugin(ParticlesPlugin)
        .add_plugin(WorldInspectorPlugin::default())
        .add_plugin(NoCameraPlayerPlugin)
        .add_startup_system(spawn_gun_and_particles)
        .add_startup_system(spawn_cubes)
        .add_system(rotate_gun)
        .run();
}

struct Gun(Entity);

fn spawn_gun_and_particles(mut commands: Commands, asset_server: Res<AssetServer>) {
    commands
        .spawn_bundle(PerspectiveCameraBundle {
            transform: Transform::from_xyz(0.0, 5.0, 5.0).looking_at(Vec3::ZERO, Vec3::Y),
            ..PerspectiveCameraBundle::new_3d()
        })
        .insert(FlyCam);
    commands.spawn_bundle(DirectionalLightBundle::default());
    //  let bullet_texture = asset_server.load("kenney/bullet.png");
    let gun_mesh = asset_server.load("kenney/blaster.glb#Scene0");
    let muzzle_texture = asset_server.load("kenney/muzzle_01.png");
    let bullet_texture = asset_server.load("kenney/bullet.png");
    let smoke_texture = asset_server.load("kenney/smoke_05.png");
    let gun = commands
        .spawn()
        .insert_bundle((
            Transform::from_xyz(0.0, 1.0, 0.0),
            GlobalTransform::default(),
        ))
        .insert(Name::new("Gun"))
        .with_children(|b| {
            b.spawn_bundle((Transform::from_xyz(0.2, 0., 1.), GlobalTransform::default()))
                .with_children(|b2| {
                    // Gun mesh
                    b2.spawn_scene(gun_mesh);
                    // Particle systems
                    b2.spawn_bundle(ParticleSystemBundle {
                        material: ParticleMaterial::Image(muzzle_texture),
                        particle_system: ParticleSystem::default(),
                        particle_params: ParticleParams {
                            start_lifetime: 0.1.into(),
                            start_size: 0.5.into(),
                            start_speed: 0.0.into(),
                            rotation: RotationMode::AlignToDirection { offset: 0.0 },
                            ..Default::default()
                        },
                        particle_emitter: ParticleEmitter {
                            shape: EmitterShape {
                                shape: Cone { angle: 0.0 }.into(),
                                thickness: 0.0,
                                ..Default::default()
                            },
                            rate: 10.0,
                            ..Default::default()
                        },
                        particle_render_mode: ParticleRenderMode::BillBoard {
                            alignment: BillBoardAlignment::Local,
                        },
                        transform: Transform::from_xyz(-0.2, 0.0, 0.3),
                        ..Default::default()
                    })
                    .insert(SizeOverTime(1.5))
                    .insert(ColorOverLifeTime(ColorGradient::white_to_none()))
                    .insert(Name::new("Muzzle"));
                    b2.spawn_bundle(ParticleSystemBundle {
                        material: ParticleMaterial::Image(smoke_texture),
                        particle_system: ParticleSystem::world_space(),
                        particle_params: ParticleParams {
                            start_lifetime: 1.0.into(),
                            start_size: (0.1..=0.3).into(),
                            start_speed: (0.5..=1.0).into(),
                            rotation: RotationMode::FreeRotation {
                                start_rotation: (-6.0..=6.0).into(),
                                start_angular_velocity: 0.5.into(),
                            },
                            ..Default::default()
                        },
                        particle_emitter: ParticleEmitter {
                            shape: EmitterShape {
                                shape: Cone { angle: 0.0 }.into(),
                                thickness: 0.0,
                                ..Default::default()
                            },
                            rate: 10.0,
                            ..Default::default()
                        },
                        transform: Transform::from_xyz(-0.2, 0.0, 0.3),
                        ..Default::default()
                    })
                    .insert(SizeOverTime(0.1))
                    .insert(ColorOverLifeTime(ColorGradient::smooth_white()))
                    .insert(Name::new("Smoke"));
                    b2.spawn_bundle(ParticleSystemBundle {
                        material: ParticleMaterial::Image(bullet_texture),
                        particle_system: ParticleSystem::world_space(),
                        particle_params: ParticleParams {
                            start_lifetime: 2.0.into(),
                            start_size: 0.15.into(),
                            start_speed: 2.0.into(),
                            rotation: RotationMode::AlignToDirection { offset: 0.0 },
                            ..Default::default()
                        },
                        particle_emitter: ParticleEmitter {
                            shape: EmitterShape {
                                shape: Cone { angle: 0.2 }.into(),
                                thickness: 1.0,
                                ..Default::default()
                            },
                            rate: 15.0,
                            transform: Transform::from_rotation(Quat::from_rotation_x(0.5)),
                            ..Default::default()
                        },
                        transform: Transform::from_xyz(-0.3, 0.1, -0.3),
                        ..Default::default()
                    })
                    .insert(ParticleGravity(Vec3::Y * -1.5))
                    .insert(ColorOverLifeTime(ColorGradient::white_to_none()))
                    .insert(Name::new("Bullets"));
                })
                .insert(Name::new("Pivot"));
        })
        .id();
    commands.insert_resource(Gun(gun));
}

fn rotate_gun(time: Res<Time>, gun: Res<Gun>, mut query: Query<&mut Transform>) {
    let mut transform = query.get_mut(gun.0).unwrap();
    transform.rotation *= Quat::from_rotation_y(time.delta_seconds());
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

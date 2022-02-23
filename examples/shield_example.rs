use bevy::prelude::*;
use bevy_flycam::*;
use bevy_inspector_egui::WorldInspectorPlugin;
use bevy_tickles::components::modifiers::ColorOverLifeTime;
use bevy_tickles::components::shapes::Sphere;
use bevy_tickles::prelude::modifiers::SizeOverTime;
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

fn spawn_particle_system(mut commands: Commands, asset_server: Res<AssetServer>) {
    commands
        .spawn_bundle(PerspectiveCameraBundle {
            transform: Transform::from_xyz(0.0, 5.0, 20.0).looking_at(Vec3::ZERO, Vec3::Y),
            ..PerspectiveCameraBundle::new_3d()
        })
        .insert(FlyCam);
    commands.spawn_bundle(DirectionalLightBundle::default());
    let shield_texture = asset_server.load("kenney/circle_03.png");
    commands
        .spawn_bundle(ParticleSystemBundle {
            particle_system: Default::default(),
            transform: Transform::from_xyz(0., 0., 0.).with_rotation(Quat::from_rotation_x(1.5)),
            global_transform: Default::default(),
            material: shield_texture.into(),
            visibility: Default::default(),
            aab: Default::default(),
            particle_params: ParticleParams {
                start_size: RangeOrFixed::Fixed(0.5),
                start_speed: 0.0.into(),
                start_lifetime: 1.5.into(),
                ..Default::default()
            },
            particle_emitter: ParticleEmitter {
                rate: 50.0,
                shape: EmitterShape {
                    shape: Sphere {
                        radius: 5.0,
                        hemisphere: false,
                    }
                    .into(),
                    thickness: 0.0,
                    mode: EmissionSpread {
                        spreads: [
                            AxisSpread {
                                amount: 0.1,
                                loop_mode: SpreadLoopMode::Loop,
                                uniform: true,
                            },
                            AxisSpread {
                                amount: 0.1,
                                loop_mode: SpreadLoopMode::Loop,
                                uniform: true,
                            },
                            AxisSpread::none(),
                        ],
                        ..Default::default()
                    }
                    .into(),
                    ..Default::default()
                },
                ..Default::default()
            },
            particle_render_mode: ParticleRenderMode::BillBoard {
                alignment: BillBoardAlignment::Direction,
            },
            ..Default::default()
        })
        .insert(ColorOverLifeTime(
            ColorGradient::empty()
                .add_point(0.0, Color::rgba(0., 0., 1., 0.))
                .add_point(0.5, Color::BLUE)
                .add_point(1.0, Color::rgba(0., 0., 1., 0.)),
        ))
        .insert(SizeOverTime(1.1))
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

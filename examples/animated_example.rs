use bevy::prelude::*;
use bevy_flycam::*;
use bevy_inspector_egui::WorldInspectorPlugin;
use bevy_tickles::prelude::shapes::Sphere;
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
    let smoke_texture = asset_server.load("fireworks.png");
    let texture_atlas = atlases.add(TextureAtlas::from_grid(
        smoke_texture,
        Vec2::new(256.0, 256.0),
        6,
        5,
    ));
    commands
        .spawn_bundle(ParticleSystemBundle {
            transform: Transform::from_xyz(0., 0., 0.),
            material: ParticleTextureSheet {
                texture_atlas,
                mode: TextureSheetMode::AnimateOverLifetime(TextureSheetAnimation {
                    start_index: 0,
                    looping_mode: TextureSheetLoopingMode::None,
                    ..Default::default()
                }),
            }
            .into(),
            particle_params: ParticleParams {
                start_size: RangeOrFixed::Range { min: 1.0, max: 4.0 },
                start_speed: 0.0.into(),
                start_lifetime: 1.0.into(),
                start_color: ColorGradient::rainbow().into(),
                ..Default::default()
            },
            particle_emitter: ParticleEmitter {
                rate: 300.0,
                shape: Shape::Sphere(Sphere {
                    radius: 10.0,
                    edge_only: true,
                    ..Default::default()
                }),
                ..Default::default()
            },
            ..Default::default()
        })
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

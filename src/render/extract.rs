#![allow(clippy::needless_pass_by_value)]
use crate::render::{ExtractedParticle, ExtractedParticles};
use crate::{Particle, ParticleMaterial, ParticleRenderMode, ParticleSystem};
use bevy::prelude::*;
use bevy::render::RenderWorld;

#[allow(clippy::type_complexity)]
pub fn extract_particles(
    mut render_world: ResMut<RenderWorld>,
    cameras: Query<
        &GlobalTransform,
        (
            With<Camera>,
            Or<(With<PerspectiveProjection>, With<OrthographicProjection>)>,
        ),
    >,
    texture_atlases: Res<Assets<TextureAtlas>>,
    query: Query<(
        &GlobalTransform,
        &ParticleSystem,
        &ParticleMaterial,
        &ParticleRenderMode,
        &ComputedVisibility,
    )>,
) {
    let mut extracted_particles = render_world
        .get_resource_mut::<ExtractedParticles>()
        .unwrap();
    // TODO: Handle multiple cameras
    let camera_transform = cameras
        .get_single()
        .expect("Particle systems do not support multiple cameras yet");
    // Clear last frame extracted particles
    extracted_particles.particles.clear();
    for (ps_transform, particles, material, render_mode, visibility) in query.iter() {
        // skips invisible particle systems
        if !visibility.is_visible {
            continue;
        }
        let (image_handle_id, anim) = match material {
            ParticleMaterial::Image(image) => (image.id, None),
            ParticleMaterial::TextureSheet(sheet) => {
                let atlas = texture_atlases
                    .get(sheet.texture_atlas.clone_weak())
                    .unwrap_or_else(|| {
                        panic!(
                            "Failed to retrieve `TextureAtlas` from handle {:?}",
                            sheet.texture_atlas.id
                        )
                    });
                (atlas.texture.id, Some((sheet, atlas)))
            }
        };
        let matrix: Mat4 = ps_transform.compute_matrix();
        let extracted = particles.iter().cloned().map(|mut particle: Particle| {
            if !particles.world_space {
                particle = particle.transformed(&matrix);
            }
            let mut transform = Transform::from_translation(particle.translation);
            render_mode.apply_to_particle(&particle, &mut transform, camera_transform);
            ExtractedParticle {
                image_handle_id,
                transform,
                color: particle.color,
                rect: if let Some((sheet, atlas)) = anim {
                    Some((sheet.mode.rect(atlas, &particle), atlas.size))
                } else {
                    None
                },
                size: Vec2::splat(particle.size),
            }
        });
        extracted_particles.particles.extend(extracted);
    }
}

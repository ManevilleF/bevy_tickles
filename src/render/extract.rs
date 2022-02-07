#![allow(clippy::needless_pass_by_value)]
use crate::render::{ExtractedParticle, ExtractedParticles};
use crate::{ParticleMaterial, ParticleRenderMode, ParticleRng, ParticleSystem};
use bevy::prelude::*;
use bevy::render::RenderWorld;

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
    mut query: Query<(
        &GlobalTransform,
        &ParticleSystem,
        &mut ParticleRng,
        &ParticleMaterial,
        &ParticleRenderMode,
        &ComputedVisibility,
    )>,
) {
    let mut extracted_particles = render_world
        .get_resource_mut::<ExtractedParticles>()
        .unwrap();
    // TODO: Handle multiple cameras
    let camera_transform = cameras.single();
    // Clear last frame extracted particles
    extracted_particles.particles.clear();
    for (transform, particles, mut rng, material, render_mode, visibility) in query.iter_mut() {
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
        let matrix: Mat4 = transform.compute_matrix();
        let extracted = particles.iter().map(|p| ExtractedParticle {
            image_handle_id,
            position: if particles.world_space {
                p.translation
            } else {
                matrix.transform_point3(p.translation)
            },
            rotation: p.rotation,
            color: p.color,
            rect: if let Some((sheet, atlas)) = anim {
                Some((sheet.mode.rect(atlas, p, rng.rng()), atlas.size))
            } else {
                None
            },
            size: Vec2::splat(p.size), // TODO: support stretched particles
        });
        extracted_particles.particles.extend(extracted);
    }
}

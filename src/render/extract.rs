#![allow(clippy::needless_pass_by_value)]
use crate::{ParticleMaterial, ParticleRng, ParticleSystem};
use bevy::asset::HandleId;
use bevy::prelude::*;
use bevy::render::RenderWorld;
use bevy::sprite::Rect;

#[derive(Component, Clone, Copy)]
pub struct ExtractedParticle {
    /// Texture handle id
    pub image_handle_id: HandleId,
    /// World space position
    pub position: Vec3,
    /// rotation
    pub rotation: f32,
    /// color tint
    pub color: Color,
    /// Select an area of the texture
    pub rect: Option<Rect>,
    /// Size of the sprite
    pub size: Vec2,
}

#[derive(Default)]
pub struct ExtractedParticles {
    pub particles: Vec<ExtractedParticle>,
}

pub fn extract_particles(
    mut render_world: ResMut<RenderWorld>,
    texture_atlases: Res<Assets<TextureAtlas>>,
    mut query: Query<(
        &GlobalTransform,
        &ParticleSystem,
        &mut ParticleRng,
        &ParticleMaterial,
        &Visibility,
    )>,
) {
    let mut extracted_particles = render_world
        .get_resource_mut::<ExtractedParticles>()
        .unwrap();
    // Clear last frame extracted particles
    extracted_particles.particles.clear();
    for (transform, particles, mut rng, material, visibility) in query.iter_mut() {
        // skips invisible particle systems
        if !visibility.is_visible {
            continue;
        }
        let matrix: Mat4 = transform.compute_matrix();
        let extracted = match material {
            ParticleMaterial::Image(image) => {
                particles
                    .iter()
                    .map(|p| ExtractedParticle {
                        image_handle_id: image.id,
                        position: if particles.world_space {
                            p.translation
                        } else {
                            matrix.transform_point3(p.translation)
                        },
                        rotation: p.rotation,
                        color: p.color,
                        rect: None,
                        size: Vec2::splat(p.size), // TODO: support stretched particles
                    })
                    .collect::<Vec<ExtractedParticle>>()
            }
            ParticleMaterial::TextureSheet(sheet) => {
                let atlas = texture_atlases
                    .get(sheet.texture_atlas.clone_weak())
                    .unwrap_or_else(|| {
                        panic!(
                            "Failed to retrieve `TextureAtlas` from handle {:?}",
                            sheet.texture_atlas.id
                        )
                    });
                particles
                    .iter()
                    .map(|p| ExtractedParticle {
                        image_handle_id: atlas.texture.id,
                        position: if particles.world_space {
                            p.translation
                        } else {
                            matrix.transform_point3(p.translation)
                        },
                        rotation: p.rotation,
                        color: p.color,
                        rect: Some(sheet.mode.rect(atlas, p, rng.rng())),
                        size: Vec2::splat(p.size), // TODO: support stretched particles
                    })
                    .collect::<Vec<ExtractedParticle>>()
            }
        };
        extracted_particles.particles.extend(extracted);
    }
}

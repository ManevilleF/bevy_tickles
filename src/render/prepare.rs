use crate::render::{ExtractedParticles, ParticleBatch, ParticleMeta, ParticleVertex};
use bevy::math::const_vec2;
use bevy::prelude::*;
use bevy::render::renderer::{RenderDevice, RenderQueue};
use itertools::Itertools;
use std::cmp::Ordering;

/// Vertex indices for quad (2 triangles)
const QUAD_INDICES: [usize; 6] = [0, 2, 3, 0, 1, 2];

/// Relative vertex positions for quads
const QUAD_VERTEX_POSITIONS: [Vec2; 4] = [
    const_vec2!([-0.5, -0.5]),
    const_vec2!([0.5, -0.5]),
    const_vec2!([0.5, 0.5]),
    const_vec2!([-0.5, 0.5]),
];

/// UV coordinates for quads
const QUAD_UVS: [Vec2; 4] = [
    const_vec2!([0., 1.]),
    const_vec2!([1., 1.]),
    const_vec2!([1., 0.]),
    const_vec2!([0., 0.]),
];

pub fn prepare_particles(
    mut commands: Commands,
    mut extracted_particles: ResMut<ExtractedParticles>,
    mut particle_meta: ResMut<ParticleMeta>,
    render_device: Res<RenderDevice>,
    render_queue: Res<RenderQueue>,
) {
    let particles = &mut extracted_particles.particles;
    // Sort particles by z for correct transparency and then by handle to improve batching
    particles.sort_unstable_by(|a, b| match a.position.z.partial_cmp(&b.position.z) {
        Some(Ordering::Equal) | None => a.image_handle_id.cmp(&b.image_handle_id),
        Some(other) => other,
    });
    // Clear the vertex buffers
    particle_meta.vertices.clear();
    let mut index = 0;
    // We group every consecutive particle with equal `image_handle_id` and create batches
    for (image_handle_id, group) in &particles.iter().group_by(|p| p.image_handle_id) {
        // We compute the vertices for each group
        let vertices: Vec<ParticleVertex> = group
            .flat_map(|particle| {
                let mut uvs = QUAD_UVS;
                // If a rect is specified, adjust UVs and the size of the quad
                if let Some(rect) = particle.rect {
                    let rect_size = rect.size();
                    for uv in &mut uvs {
                        *uv = (rect.min + *uv * rect_size) / particle.size;
                    }
                }
                // encode color as a single u32 to save space
                let color = particle.color.as_linear_rgba_f32();
                let color = (color[0] * 255.0) as u32
                    | ((color[1] * 255.0) as u32) << 8
                    | ((color[2] * 255.0) as u32) << 16
                    | ((color[3] * 255.0) as u32) << 24;
                let positions = QUAD_VERTEX_POSITIONS
                    .map(|quad_pos| (particle.position + quad_pos.extend(0.)).into());
                QUAD_INDICES
                    .iter()
                    .map(|i| ParticleVertex {
                        position: positions[*i],
                        uv: uvs[*i].into(),
                        color,
                    })
                    .collect::<Vec<ParticleVertex>>()
            })
            .collect();
        let len = vertices.len() as u32;
        // TODO: `BufferVec` should implement an `extend` method
        for vertex in vertices.clone() {
            particle_meta.vertices.push(vertex);
        }
        commands.spawn_bundle((ParticleBatch {
            image_handle_id,
            range: (index..(index + len)),
        },));
        index += len;
    }
    particle_meta
        .vertices
        .write_buffer(&render_device, &render_queue);
}

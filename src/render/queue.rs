use crate::render::draw::DrawParticle;
use crate::render::pipeline::ParticlePipeline;
use crate::ExtractedParticles;
use bevy::asset::HandleId;
use bevy::core_pipeline::Transparent3d;
use bevy::math::const_vec2;
use bevy::prelude::*;
use bevy::render::{
    render_asset::RenderAssets,
    render_phase::{DrawFunctions, RenderPhase},
    render_resource::{
        BindGroup, BindGroupDescriptor, BindGroupEntry, BufferUsages, BufferVec,
        RenderPipelineCache, SpecializedPipelines,
    },
    renderer::{RenderDevice, RenderQueue},
    view::ViewUniforms,
};
use bytemuck::{Pod, Zeroable};
use itertools::Itertools;
use std::cmp::Ordering;
use std::ops::Range;

#[repr(C)]
#[derive(Copy, Clone, Pod, Zeroable)]
pub struct ParticleVertex {
    /// Vertex position
    pub position: [f32; 3],
    /// UV Coordinates (texturing)
    pub uv: [f32; 2],
    /// Vertex color
    pub color: u32,
}

pub struct ParticleMeta {
    /// Every particle vertex information
    pub vertices: BufferVec<ParticleVertex>,
    pub view_bind_group: Option<BindGroup>,
}

/// Particle batch by texture handle
#[derive(Component, Clone)]
pub struct ParticleBatch {
    pub image_handle_id: HandleId,
    pub range: Range<u32>,
}

pub struct TmpBatch {
    batch: ParticleBatch,
    vertices: Vec<ParticleVertex>,
}

impl Default for ParticleMeta {
    fn default() -> Self {
        Self {
            vertices: BufferVec::new(BufferUsages::VERTEX),
            view_bind_group: None,
        }
    }
}

const QUAD_INDICES: [usize; 6] = [0, 2, 3, 0, 1, 2];

const QUAD_VERTEX_POSITIONS: [Vec2; 4] = [
    const_vec2!([-0.5, -0.5]),
    const_vec2!([0.5, -0.5]),
    const_vec2!([0.5, 0.5]),
    const_vec2!([-0.5, 0.5]),
];

const QUAD_UVS: [Vec2; 4] = [
    const_vec2!([0., 1.]),
    const_vec2!([1., 1.]),
    const_vec2!([1., 0.]),
    const_vec2!([0., 0.]),
];

pub fn queue_particles(
    mut commands: Commands,
    draw_functions: Res<DrawFunctions<Transparent3d>>,
    render_device: Res<RenderDevice>,
    render_queue: Res<RenderQueue>,
    mut particle_meta: ResMut<ParticleMeta>,
    view_uniforms: Res<ViewUniforms>,
    particle_pipeline: Res<ParticlePipeline>,
    mut pipelines: ResMut<SpecializedPipelines<ParticlePipeline>>,
    mut pipeline_cache: ResMut<RenderPipelineCache>,
    gpu_images: Res<RenderAssets<Image>>,
    mut extracted_particles: ResMut<ExtractedParticles>,
    mut views: Query<&mut RenderPhase<Transparent3d>>,
) {
    if let Some(view_binding) = view_uniforms.uniforms.binding() {
        let meta = &mut particle_meta;

        // Clear the vertex buffers
        meta.vertices.clear();
        // Define the view bind group
        meta.view_bind_group = Some(render_device.create_bind_group(&BindGroupDescriptor {
            entries: &[BindGroupEntry {
                binding: 0,
                resource: view_binding,
            }],
            label: Some("particle_view_bind_group"),
            layout: &particle_pipeline.view_layout,
        }));
        // Retrieve the particle drawing function
        let draw_particle_function = draw_functions.read().get_id::<DrawParticle>().unwrap();
        // Cache the specialized pipeline
        let pipeline = pipelines.specialize(&mut pipeline_cache, &particle_pipeline, ());

        // We retrieve the extracted particles
        let extracted_particles = &mut extracted_particles.particles;
        // Sort particles by z for correct transparency and then by handle to improve batching
        extracted_particles.sort_unstable_by(|a, b| {
            match a.position.z.partial_cmp(&b.position.z) {
                Some(Ordering::Equal) | None => a.image_handle_id.cmp(&b.image_handle_id),
                Some(other) => other,
            }
        });
        let mut index = 0;
        let mut batches = vec![];
        // We group every consecutive particle with equal `image_handle_id` and create batches
        for (image_handle_id, group) in &extracted_particles.iter().group_by(|p| p.image_handle_id)
        {
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
            batches.push(TmpBatch {
                batch: ParticleBatch {
                    image_handle_id,
                    range: (index..(index + len)),
                },
                vertices,
            });
            index += len;
        }

        for mut transparent_phase in views.iter_mut() {
            for tmp_batch in &batches {
                let batch = &tmp_batch.batch;
                if !gpu_images.contains_key(&Handle::weak(batch.image_handle_id)) {
                    // Skip this item if the texture is not ready
                    continue;
                }
                let entity = commands.spawn_bundle((batch.clone(),)).id();
                for vertex in tmp_batch.vertices.clone() {
                    transparent_phase.add(Transparent3d {
                        distance: vertex.position[2], // TODO: distance to camera
                        draw_function: draw_particle_function,
                        pipeline,
                        entity,
                    });
                    particle_meta.vertices.push(vertex);
                }
            }
        }
        particle_meta
            .vertices
            .write_buffer(&render_device, &render_queue);
    }
}

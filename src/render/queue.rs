use crate::render::draw::DrawParticle;
use crate::render::pipeline::ParticlePipeline;
use crate::render::{ParticleBatch, ParticleImageBindGroups, ParticleMeta};
use bevy::core_pipeline::Transparent3d;
use bevy::prelude::*;
use bevy::render::render_resource::BindingResource;
use bevy::render::{
    render_asset::RenderAssets,
    render_phase::{DrawFunctions, RenderPhase},
    render_resource::{
        BindGroupDescriptor, BindGroupEntry, PipelineCache, SpecializedRenderPipelines,
    },
    renderer::RenderDevice,
    view::ViewUniforms,
};
use bevy::sprite::SpriteAssetEvents;

#[allow(clippy::needless_pass_by_value, clippy::too_many_arguments)]
pub fn queue_particles(
    draw_functions: Res<DrawFunctions<Transparent3d>>,
    render_device: Res<RenderDevice>,
    view_uniforms: Res<ViewUniforms>,
    particle_pipeline: Res<ParticlePipeline>,
    mut pipelines: ResMut<SpecializedRenderPipelines<ParticlePipeline>>,
    mut pipeline_cache: ResMut<PipelineCache>,
    mut particle_meta: ResMut<ParticleMeta>,
    gpu_images: Res<RenderAssets<Image>>,
    batch_query: Query<(Entity, &ParticleBatch)>,
    mut image_bind_groups: ResMut<ParticleImageBindGroups>,
    mut views: Query<&mut RenderPhase<Transparent3d>>,
    events: Res<SpriteAssetEvents>,
) {
    // If an image has changed, the GpuImage has (probably) changed
    for event in &events.images {
        match event {
            AssetEvent::Created { .. } => None,
            AssetEvent::Modified { handle } | AssetEvent::Removed { handle } => {
                image_bind_groups.values.remove(handle)
            }
        };
    }

    if let Some(view_binding) = view_uniforms.uniforms.binding() {
        // Define the view bind group
        particle_meta.view_bind_group =
            Some(render_device.create_bind_group(&BindGroupDescriptor {
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

        for mut transparent_phase in views.iter_mut() {
            for (entity, batch) in batch_query.iter() {
                if let Some(gpu_image) = gpu_images.get(&Handle::weak(batch.image_handle_id)) {
                    image_bind_groups
                        .values
                        .entry(Handle::weak(batch.image_handle_id))
                        .or_insert_with(|| {
                            render_device.create_bind_group(&BindGroupDescriptor {
                                entries: &[
                                    BindGroupEntry {
                                        binding: 0,
                                        resource: BindingResource::TextureView(
                                            &gpu_image.texture_view,
                                        ),
                                    },
                                    BindGroupEntry {
                                        binding: 1,
                                        resource: BindingResource::Sampler(&gpu_image.sampler),
                                    },
                                ],
                                label: Some("particle_image_bind_group"),
                                layout: &particle_pipeline.image_layout,
                            })
                        });
                    transparent_phase.add(Transparent3d {
                        distance: 10., // TODO: Try using `batch.range.min`
                        draw_function: draw_particle_function,
                        pipeline,
                        entity,
                    });
                }
            }
        }
    }
}

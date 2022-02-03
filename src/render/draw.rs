use crate::render::{ParticleBatch, ParticleMeta};
use crate::ParticleImageBindGroups;
use bevy::ecs::system::{
    lifetimeless::{Read, SQuery, SRes},
    SystemParamItem,
};
use bevy::prelude::*;
use bevy::render::render_phase::{
    EntityRenderCommand, RenderCommandResult, SetItemPipeline, TrackedRenderPass,
};
use bevy::render::view::ViewUniformOffset;

pub type DrawParticle = (
    SetItemPipeline,
    SetParticleViewBindGroup<0>,
    SetParticleTextureBindGroup<1>,
    DrawParticleBatch,
);

pub struct SetParticleViewBindGroup<const I: usize>;
impl<const I: usize> EntityRenderCommand for SetParticleViewBindGroup<I> {
    type Param = (SRes<ParticleMeta>, SQuery<Read<ViewUniformOffset>>);

    fn render<'w>(
        view: Entity,
        _item: Entity,
        (meta, view_query): SystemParamItem<'w, '_, Self::Param>,
        pass: &mut TrackedRenderPass<'w>,
    ) -> RenderCommandResult {
        let view_uniform = view_query.get(view).unwrap();
        pass.set_bind_group(
            I,
            meta.into_inner().view_bind_group.as_ref().unwrap(),
            &[view_uniform.offset],
        );
        RenderCommandResult::Success
    }
}

pub struct SetParticleTextureBindGroup<const I: usize>;
impl<const I: usize> EntityRenderCommand for SetParticleTextureBindGroup<I> {
    type Param = (SRes<ParticleImageBindGroups>, SQuery<Read<ParticleBatch>>);

    fn render<'w>(
        _view: Entity,
        item: Entity,
        (image_bind_groups, query_batch): SystemParamItem<'w, '_, Self::Param>,
        pass: &mut TrackedRenderPass<'w>,
    ) -> RenderCommandResult {
        let batch = query_batch.get(item).unwrap();
        let image_bind_groups = image_bind_groups.into_inner();

        pass.set_bind_group(
            I,
            image_bind_groups
                .values
                .get(&Handle::weak(batch.image_handle_id))
                .unwrap(),
            &[],
        );
        RenderCommandResult::Success
    }
}

pub struct DrawParticleBatch;
impl EntityRenderCommand for DrawParticleBatch {
    type Param = (SRes<ParticleMeta>, SQuery<Read<ParticleBatch>>);

    fn render<'w>(
        _view: Entity,
        item: Entity,
        (particle_meta, query_batch): SystemParamItem<'w, '_, Self::Param>,
        pass: &mut TrackedRenderPass<'w>,
    ) -> RenderCommandResult {
        // We retrieve the `ParticleBatch` component from the item entity
        let batch = query_batch.get(item).unwrap();
        let particle_meta = particle_meta.into_inner();
        // We pass the entire vertex buffer to the render pass? TODO: This seems wrong
        pass.set_vertex_buffer(0, particle_meta.vertices.buffer().unwrap().slice(..));
        // We draw only the vertices contained in the batch range
        pass.draw(batch.range.clone(), 0..1);
        RenderCommandResult::Success

        // TODO: Where do I retrieve the Texture ?
    }
}

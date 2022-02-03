use crate::render::queue::{ParticleBatch, ParticleMeta};
use crate::Transparent3d;
use bevy::ecs::system::{
    lifetimeless::{Read, SQuery, SRes},
    SystemParamItem,
};
use bevy::prelude::*;
use bevy::render::render_phase::{
    RenderCommand, RenderCommandResult, SetItemPipeline, TrackedRenderPass,
};

pub type DrawParticle = (SetItemPipeline, DrawParticleBatch);

pub struct DrawParticleBatch;
impl RenderCommand<Transparent3d> for DrawParticleBatch {
    type Param = (SRes<ParticleMeta>, SQuery<Read<ParticleBatch>>);

    fn render<'w>(
        _view: Entity,
        item: &Transparent3d,
        (particle_meta, query_batch): SystemParamItem<'w, '_, Self::Param>,
        pass: &mut TrackedRenderPass<'w>,
    ) -> RenderCommandResult {
        // We retrieve the `ParticleBatch` component from the `Transparent3d::entity`
        let batch = query_batch.get(item.entity).unwrap();
        let particle_meta = particle_meta.into_inner();
        // We pass the entire vertex buffer to the render pass? TODO: This seems wrong
        pass.set_vertex_buffer(0, particle_meta.vertices.buffer().unwrap().slice(..));
        // We draw only the vertices contained in the batch range
        pass.draw(batch.range.clone(), 0..1);
        RenderCommandResult::Success

        // TODO: Where do I retrieve the Texture ?
    }
}

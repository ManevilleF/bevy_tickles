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
        let batch = query_batch.get(item.entity).unwrap();
        let particle_meta = particle_meta.into_inner();
        pass.set_vertex_buffer(0, particle_meta.vertices.buffer().unwrap().slice(..));
        pass.draw(batch.range.clone(), 0..1);
        RenderCommandResult::Success
    }
}

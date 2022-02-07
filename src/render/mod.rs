use bevy::asset::HandleId;
use bevy::prelude::{Color, Component, Handle, Image, Vec2, Vec3};
use bevy::render::render_resource::{BindGroup, BufferUsages, BufferVec};
use bevy::sprite::Rect;
use bevy::utils::HashMap;
use bytemuck::{Pod, Zeroable};
use std::ops::Range;

pub mod draw;
pub mod extract;
pub mod pipeline;
pub mod prepare;
pub mod queue;

#[derive(Default)]
pub struct ParticleImageBindGroups {
    pub values: HashMap<Handle<Image>, BindGroup>,
}

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
    /// (Custom area of the texture, the texture size)
    pub rect: Option<(Rect, Vec2)>,
    /// Size of the sprite
    pub size: Vec2,
}

#[derive(Default)]
pub struct ExtractedParticles {
    pub particles: Vec<ExtractedParticle>,
}

/// Particle batch by texture handle
#[derive(Component, Clone)]
pub struct ParticleBatch {
    /// Texture handle
    pub image_handle_id: HandleId,
    /// Vertex buffer index range matching the texture id
    pub range: Range<u32>,
}

/// Single particle vertex representation
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

impl Default for ParticleMeta {
    fn default() -> Self {
        Self {
            vertices: BufferVec::new(BufferUsages::VERTEX),
            view_bind_group: None,
        }
    }
}

pub struct ParticleMeta {
    /// Every particle vertex information
    pub vertices: BufferVec<ParticleVertex>,
    /// Bind group corresponding to the pipeline `view_layout` bind group layout
    pub view_bind_group: Option<BindGroup>,
}

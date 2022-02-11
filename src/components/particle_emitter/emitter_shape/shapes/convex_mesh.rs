use crate::components::particle_emitter::emitter_shape::{EmittedParticle, Emitter};
use crate::{EmissionSpread, EmitterDirectionMode};
use bevy::prelude::{shape::Cube, Mesh, Vec3};
use bevy::render::mesh::VertexAttributeValues;
use rand::Rng;

/// Initializes particles at randomly-sampled positions within a convex mesh and directs them outwards from the center
#[derive(Debug, Clone)]
#[cfg_attr(feature = "inspector", derive(bevy_inspector_egui::Inspectable))]
pub struct ConvexMesh(pub Mesh);

impl Emitter for ConvexMesh {
    fn emit_random_particle(
        &self,
        rng: &mut impl Rng,
        thickness: f32,
        direction_mode: EmitterDirectionMode,
    ) -> EmittedParticle {
        let mesh = &self.0;
        if mesh.count_vertices() == 0 {
            return Default::default();
        }
        let positions = mesh
            .attribute(Mesh::ATTRIBUTE_POSITION)
            .expect("No vertex positions set for `ConvexMesh`");
        let position: Vec3 = if let VertexAttributeValues::Float32x3(positions) = positions {
            positions[rng.gen_range(0..positions.len())].into()
        } else {
            panic!("Expected a mesh with `Float32x3` positions");
        };
        let coef = rng.gen_range((1.0 - thickness)..=1.0);
        EmittedParticle {
            position: position * coef,
            direction: match direction_mode {
                EmitterDirectionMode::Automatic => position.try_normalize().unwrap_or(Vec3::Y),
                EmitterDirectionMode::Fixed(dir) => dir,
            },
        }
    }

    fn spread_particle(
        &self,
        spread: &mut EmissionSpread,
        thickness: f32,
        direction_mode: EmitterDirectionMode,
    ) -> EmittedParticle {
        todo!()
    }
}

impl Default for ConvexMesh {
    fn default() -> Self {
        Self(Mesh::from(Cube::default()))
    }
}

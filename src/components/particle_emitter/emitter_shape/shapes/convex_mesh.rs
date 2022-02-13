use crate::components::particle_emitter::emitter_shape::{EmittedParticle, Emitter};
use crate::{radius_spread, EmissionSpread, EmitterDirectionMode};
use bevy::prelude::{shape::Cube, Mesh, Vec3};
use bevy::render::mesh::VertexAttributeValues;
use rand::Rng;

/// Initializes particles at randomly-sampled positions within a convex mesh and directs them outwards from the `nominal_center`
#[derive(Debug, Clone)]
#[cfg_attr(feature = "inspector", derive(bevy_inspector_egui::Inspectable))]
pub struct ConvexMesh {
    /// The mesh object
    pub mesh: Mesh,
    /// The *nominal center* of the convex mesh
    pub nominal_center: Vec3,
}

impl Emitter for ConvexMesh {
    // TODO: use triangles ?
    fn emit_random_particle(
        &self,
        rng: &mut impl Rng,
        thickness: f32,
        direction_mode: EmitterDirectionMode,
    ) -> EmittedParticle {
        let mesh = &self.mesh;
        if mesh.count_vertices() == 0 {
            return Default::default();
        }
        let positions = mesh
            .attribute(Mesh::ATTRIBUTE_POSITION)
            .expect("No vertex positions set for `ConvexMesh`");
        // TODO: use triangles
        //  let indices = mesh.indices().expect("No indices set for `ConvexMesh");
        //         let iterator = match indices {
        //             Indices::U16(vec) => vec.iter().map(|i| i as usize),
        //             Indices::U32(vec) => vec.iter().map(|i| i as usize),
        //         };
        //         let triangles: Vec<[usize; 3]> = iterator.chunks(3).collect();
        //         let triangle = triangles[rng.gen_range(0..triangles.len())];
        //         let positions = if let VertexAttributeValues::Float32x3(positions) = positions {
        //             [positions[triangle[0]],positions[triangle[1]],positions[triangle[2]]]
        //         } else {
        //             panic!("Expected a mesh with `Float32x3` positions");
        //         };
        let position: Vec3 = if let VertexAttributeValues::Float32x3(positions) = positions {
            positions[rng.gen_range(0..positions.len())].into()
        } else {
            panic!("Expected a mesh with `Float32x3` positions");
        };
        let coef = rng.gen_range((1.0 - thickness)..=1.0);
        EmittedParticle {
            position: position * coef,
            direction: match direction_mode {
                EmitterDirectionMode::Automatic => (position - self.nominal_center)
                    .try_normalize()
                    .unwrap_or(Vec3::Y),
                EmitterDirectionMode::Fixed(dir) => dir,
            },
        }
    }

    fn spread_particle(
        &self,
        spread: &mut EmissionSpread,
        _rng: &mut impl Rng,
        thickness: f32,
        direction_mode: EmitterDirectionMode,
    ) -> EmittedParticle {
        let mesh = &self.mesh;
        if mesh.count_vertices() == 0 {
            return Default::default();
        }
        let (_previous_index, index) = spread.update_index();
        let positions = mesh
            .attribute(Mesh::ATTRIBUTE_POSITION)
            .expect("No vertex positions set for `ConvexMesh`");
        // TODO: support non uniform spread
        let vertex_index = (index.z * (positions.len() - 1) as f32) as usize;
        let position: Vec3 = if let VertexAttributeValues::Float32x3(positions) = positions {
            positions[vertex_index].into()
        } else {
            panic!("Expected a mesh with `Float32x3` positions");
        };
        // TODO: support non uniform spread
        let coef = radius_spread(1.0, thickness, index.y);
        EmittedParticle {
            position: position * coef,
            direction: match direction_mode {
                EmitterDirectionMode::Automatic => (position - self.nominal_center)
                    .try_normalize()
                    .unwrap_or(Vec3::Y),
                EmitterDirectionMode::Fixed(dir) => dir,
            },
        }
    }
}

impl Default for ConvexMesh {
    fn default() -> Self {
        Self {
            mesh: Mesh::from(Cube::default()),
            nominal_center: Default::default(),
        }
    }
}

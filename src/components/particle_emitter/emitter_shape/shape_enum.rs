use super::{EmittedParticle, Emitter, EmitterDirectionMode};
use crate::shapes;
use bevy::prelude::Reflect;
use rand::Rng;

/// Available shapes for the particle emitter
#[derive(Debug, Clone, Reflect)]
#[cfg_attr(feature = "inspector", derive(bevy_inspector_egui::Inspectable))]
pub enum Shape {
    /// Initializes particles at randomly-sampled positions within a sphere and directs them outwards from the center
    Sphere(shapes::Sphere),
    /// Initializes particles at randomly-sampled positions within a circle in the direction of the emitter’s up axis
    Circle(shapes::Circle),
    /// Initializes particles at the tip of a cone and directs them at random angles out of the cone.
    /// The cone is oriented along the up axis of the emitter.
    Cone(shapes::Cone),
    /// Initializes particles at randomly-sampled positions within a box and directs them out of one of the six box faces.
    Box(shapes::Box),
    /// Emit particles from a line segment. The particles move in the emitter object’s upward (Y) direction.
    Edge(shapes::Edge),
    /// Emits particles at randomly-sampled positions within a convex mesh and directs them outwards from the center
    ConvexMesh(shapes::ConvexMesh),
}

impl Default for Shape {
    fn default() -> Self {
        Self::Sphere(Default::default())
    }
}

impl Emitter for Shape {
    fn emit_particle(
        &self,
        rng: &mut impl Rng,
        thickness: f32,
        direction_mode: EmitterDirectionMode,
    ) -> EmittedParticle {
        match self {
            Shape::Sphere(s) => s.emit_particle(rng, thickness, direction_mode),
            Shape::Circle(s) => s.emit_particle(rng, thickness, direction_mode),
            Shape::Cone(s) => s.emit_particle(rng, thickness, direction_mode),
            Shape::Box(s) => s.emit_particle(rng, thickness, direction_mode),
            Shape::Edge(s) => s.emit_particle(rng, thickness, direction_mode),
            Shape::ConvexMesh(s) => s.emit_particle(rng, thickness, direction_mode),
        }
    }
}

impl From<shapes::Sphere> for Shape {
    fn from(v: shapes::Sphere) -> Self {
        Self::Sphere(v)
    }
}

impl From<shapes::Box> for Shape {
    fn from(v: shapes::Box) -> Self {
        Self::Box(v)
    }
}

impl From<shapes::Cone> for Shape {
    fn from(v: shapes::Cone) -> Self {
        Self::Cone(v)
    }
}

impl From<shapes::Circle> for Shape {
    fn from(v: shapes::Circle) -> Self {
        Self::Circle(v)
    }
}

impl From<shapes::Edge> for Shape {
    fn from(v: shapes::Edge) -> Self {
        Self::Edge(v)
    }
}

impl From<shapes::ConvexMesh> for Shape {
    fn from(v: shapes::ConvexMesh) -> Self {
        Self::ConvexMesh(v)
    }
}

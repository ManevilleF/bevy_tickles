use crate::components::particle_emitter::{EmittedParticle, EmitterShape};
use bevy::prelude::Vec3;
use rand::Rng;
use std::f32::consts::PI;

const PI_2: f32 = PI * 2.0;

/// Initializes particles at randomly-sampled positions within a sphere and directs them outwards from the center
#[derive(Debug, Clone)]
#[cfg_attr(feature = "inspector", derive(bevy_inspector_egui::Inspectable))]
pub struct Sphere {
    /// Sphere radius
    pub radius: f32,
    /// Emit only on the sphere edges
    pub edge_only: bool,
    /// Uses a hemisphere instead
    pub hemisphere: bool,
    // TODO: Add uniform algorithm
}

/// Initializes particles at randomly-sampled positions within a box and directs them out of one of the six box faces.
#[derive(Debug, Clone)]
#[cfg_attr(feature = "inspector", derive(bevy_inspector_egui::Inspectable))]
pub struct Box {
    /// Box half extents
    pub extents: Vec3,
    // TODO: Implement edge_only feature
}

/// Initializes particles at the tip of a cone and directs them at random angles out of the cone.
/// The cone is oriented along the up axis of the emitter.
#[derive(Debug, Clone)]
#[cfg_attr(feature = "inspector", derive(bevy_inspector_egui::Inspectable))]
pub struct Cone {
    /// The cone angle, between `0` and `1` representing between 0 and 90 degrees
    pub angle: f32,
}

/// Initializes particles at randomly-sampled positions within a circle in the direction of the emitter’s up axis
#[derive(Debug, Clone)]
#[cfg_attr(feature = "inspector", derive(bevy_inspector_egui::Inspectable))]
pub struct Circle {
    /// Circle radius
    pub radius: f32,
}

impl EmitterShape for Sphere {
    fn emit_particle(&self, rng: &mut impl Rng) -> EmittedParticle {
        let range = if self.edge_only {
            self.radius
        } else {
            rng.gen_range(0.0..=self.radius)
        };
        let theta = PI_2 * rng.gen_range(0.0..=1.0);
        let phi = PI * rng.gen_range(0.0..=1.0);
        let sin_phi = phi.sin();
        let y = range * sin_phi * theta.sin();
        let position = Vec3::new(
            range * sin_phi * theta.cos(),
            if self.hemisphere { y.abs() } else { y },
            range * phi.cos(),
        );
        EmittedParticle {
            position,
            direction: position.try_normalize().unwrap_or(Vec3::Y),
        }
    }
}

impl EmitterShape for Circle {
    fn emit_particle(&self, rng: &mut impl Rng) -> EmittedParticle {
        let range = rng.gen_range(0.0..=self.radius);
        let theta = PI_2 * rng.gen_range(0.0..=1.0);
        let position = Vec3::new(range * theta.cos(), 0., range * theta.sin());
        EmittedParticle {
            position,
            direction: position.try_normalize().unwrap_or(Vec3::Y),
        }
    }
}

impl EmitterShape for Box {
    fn emit_particle(&self, rng: &mut impl Rng) -> EmittedParticle {
        let position = Vec3::new(
            rng.gen_range(-self.extents.x..=self.extents.x),
            rng.gen_range(-self.extents.y..=self.extents.y),
            rng.gen_range(-self.extents.z..=self.extents.z),
        );
        EmittedParticle {
            position,
            direction: match position.abs().to_array() {
                [x, y, z] if x > y && x > z => {
                    if position.z > 0. {
                        Vec3::X
                    } else {
                        -Vec3::X
                    }
                }
                [x, y, z] if y > x && y > z => {
                    if position.y > 0. {
                        Vec3::Y
                    } else {
                        -Vec3::Y
                    }
                }
                _ => {
                    if position.z > 0. {
                        Vec3::Z
                    } else {
                        -Vec3::Z
                    }
                }
            },
        }
    }
}

impl EmitterShape for Cone {
    fn emit_particle(&self, rng: &mut impl Rng) -> EmittedParticle {
        let angle = self.angle.clamp(0., 1.);
        let delta_x = rng.gen_range(-angle..=angle);
        let delta_y = rng.gen_range(-angle..=angle);
        EmittedParticle {
            position: Vec3::ZERO,
            direction: Vec3::new(delta_x, 1., delta_y),
        }
    }
}

impl Default for Sphere {
    fn default() -> Self {
        Self {
            radius: 1.0,
            edge_only: false,
            hemisphere: false,
        }
    }
}

impl Default for Cone {
    fn default() -> Self {
        Self { angle: 0.5 }
    }
}

impl Default for Circle {
    fn default() -> Self {
        Self { radius: 1.0 }
    }
}

impl Default for Box {
    fn default() -> Self {
        Self { extents: Vec3::ONE }
    }
}

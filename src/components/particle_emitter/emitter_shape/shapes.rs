use super::{EmittedParticle, Emitter};
use crate::{random_in_line, random_in_radius, EmitterDirectionMode};
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
    /// Uses a hemisphere instead
    pub hemisphere: bool,
}

/// Initializes particles at randomly-sampled positions within a box and directs them out of one of the six box faces.
#[derive(Debug, Clone)]
#[cfg_attr(feature = "inspector", derive(bevy_inspector_egui::Inspectable))]
pub struct Box {
    /// Box half extents
    pub extents: Vec3,
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

/// Emit particles from a line segment. The particles move in the emitter object’s upward (Y) direction.
#[derive(Debug, Clone)]
#[cfg_attr(feature = "inspector", derive(bevy_inspector_egui::Inspectable))]
pub struct Edge {
    /// The edge length
    pub length: f32,
}

impl Emitter for Sphere {
    fn emit_particle(
        &self,
        rng: &mut impl Rng,
        thickness: f32,
        direction_mode: EmitterDirectionMode,
    ) -> EmittedParticle {
        let range = random_in_radius(self.radius, thickness, rng);
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
            direction: match direction_mode {
                EmitterDirectionMode::Automatic => position.try_normalize().unwrap_or(Vec3::Y),
                EmitterDirectionMode::Fixed(dir) => dir,
            },
        }
    }
}

impl Emitter for Circle {
    fn emit_particle(
        &self,
        rng: &mut impl Rng,
        thickness: f32,
        direction_mode: EmitterDirectionMode,
    ) -> EmittedParticle {
        let range = random_in_radius(self.radius, thickness, rng);
        let theta = PI_2 * rng.gen_range(0.0..=1.0);
        let position = Vec3::new(range * theta.cos(), 0., range * theta.sin());
        EmittedParticle {
            position,
            direction: match direction_mode {
                EmitterDirectionMode::Automatic => position.try_normalize().unwrap_or(Vec3::Y),
                EmitterDirectionMode::Fixed(dir) => dir,
            },
        }
    }
}

impl Emitter for Box {
    fn emit_particle(
        &self,
        rng: &mut impl Rng,
        thickness: f32,
        direction_mode: EmitterDirectionMode,
    ) -> EmittedParticle {
        let mut position = Vec3::new(
            rng.gen_range(-self.extents.x..=self.extents.x),
            rng.gen_range(-self.extents.y..=self.extents.y),
            rng.gen_range(-self.extents.z..=self.extents.z),
        );
        match rng.gen_range(0..=2) {
            0 => position.x = random_in_line(self.extents.x, thickness, rng),
            1 => position.y = random_in_line(self.extents.y, thickness, rng),
            _ => position.z = random_in_line(self.extents.z, thickness, rng),
        }
        EmittedParticle {
            position,
            direction: match direction_mode {
                EmitterDirectionMode::Automatic => match position.abs().to_array() {
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
                EmitterDirectionMode::Fixed(dir) => dir,
            },
        }
    }
}

impl Emitter for Cone {
    fn emit_particle(
        &self,
        rng: &mut impl Rng,
        thickness: f32,
        direction_mode: EmitterDirectionMode,
    ) -> EmittedParticle {
        let angle = self.angle.clamp(0., 1.);
        let y = thickness - angle;
        let mut remaining = 1.0 - y;
        let x = rng.gen_range(-remaining..=remaining);
        remaining -= x.abs();
        let z = remaining;
        EmittedParticle {
            position: Vec3::ZERO,
            direction: match direction_mode {
                EmitterDirectionMode::Automatic => Vec3::new(x, y, z), // Already normalized
                EmitterDirectionMode::Fixed(dir) => dir,
            },
        }
    }
}

impl Emitter for Edge {
    fn emit_particle(
        &self,
        rng: &mut impl Rng,
        thickness: f32,
        direction_mode: EmitterDirectionMode,
    ) -> EmittedParticle {
        let value = random_in_line(self.length / 2.0, thickness, rng);
        EmittedParticle {
            position: Vec3::new(0.0, 0.0, value),
            direction: match direction_mode {
                EmitterDirectionMode::Automatic => Vec3::Y,
                EmitterDirectionMode::Fixed(dir) => dir,
            },
        }
    }
}

impl Default for Sphere {
    fn default() -> Self {
        Self {
            radius: 1.0,
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

impl Default for Edge {
    fn default() -> Self {
        Self { length: 1.0 }
    }
}

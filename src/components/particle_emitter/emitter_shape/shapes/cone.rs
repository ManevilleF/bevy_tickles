use crate::components::particle_emitter::emitter_shape::{EmittedParticle, Emitter};
use crate::shapes::PI_2;
use crate::{EmissionSpread, EmitterDirectionMode};
use bevy::prelude::Vec3;
use rand::Rng;

/// Initializes particles at the tip of a cone and directs them at random angles out of the cone.
/// The cone is oriented along the up axis of the emitter.
#[derive(Debug, Clone)]
#[cfg_attr(feature = "inspector", derive(bevy_inspector_egui::Inspectable))]
pub struct Cone {
    /// The cone angle, between `0` and `1` representing between 0 and 90 degrees
    #[cfg_attr(feature = "inspector", inspectable(min = 0.0, max = 1.0))]
    pub angle: f32,
}

impl Emitter for Cone {
    fn emit_random_particle(
        &self,
        rng: &mut impl Rng,
        thickness: f32,
        direction_mode: EmitterDirectionMode,
    ) -> EmittedParticle {
        let angle = self.angle.clamp(0., 1.);
        let min_y = 1.0 - angle;
        let y = rng.gen_range(min_y..=(min_y + thickness).clamp(0.0, 1.0));
        let mut remaining = 1.0 - y;
        let x = rng.gen_range(-remaining..=remaining);
        remaining -= x.abs();
        let z = if rng.gen_ratio(1, 2) {
            remaining
        } else {
            -remaining
        };
        EmittedParticle {
            position: Vec3::ZERO,
            direction: match direction_mode {
                EmitterDirectionMode::Automatic => Vec3::new(x, y, z), // Already normalized
                EmitterDirectionMode::Fixed(dir) => dir,
            },
        }
    }

    fn spread_particle(
        &self,
        spread: &mut EmissionSpread,
        rng: &mut impl Rng,
        thickness: f32,
        direction_mode: EmitterDirectionMode,
    ) -> EmittedParticle {
        if let EmitterDirectionMode::Fixed(direction) = direction_mode {
            return EmittedParticle {
                position: Vec3::ZERO,
                direction,
            };
        }
        let (previous_index, index) = spread.update_index();
        let angle = self.angle.clamp(0., 1.);
        let min_y = 1.0 - angle;
        // TODO: support non uniform
        let y = (((1.0 - min_y) * thickness).mul_add(index.y, min_y)).clamp(0.0, 1.0);
        let range = 1.0 - y;
        let theta = PI_2
            * if spread.spreads[2].uniform {
                index.z
            } else {
                rng.gen_range(previous_index.z.min(index.z)..=index.z.max(previous_index.z))
            };
        let direction = Vec3::new(range * theta.cos(), y, range * theta.sin());
        EmittedParticle {
            position: Vec3::ZERO,
            direction,
        }
    }
}

impl Default for Cone {
    fn default() -> Self {
        Self { angle: 0.5 }
    }
}

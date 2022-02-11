use crate::components::particle_emitter::emitter_shape::{EmittedParticle, Emitter};
use crate::{EmissionSpread, EmitterDirectionMode};
use bevy::prelude::Vec3;
use rand::Rng;

/// Initializes particles at the tip of a cone and directs them at random angles out of the cone.
/// The cone is oriented along the up axis of the emitter.
#[derive(Debug, Clone)]
#[cfg_attr(feature = "inspector", derive(bevy_inspector_egui::Inspectable))]
pub struct Cone {
    /// The cone angle, between `0` and `1` representing between 0 and 90 degrees
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

    fn spread_particle(
        &self,
        spread: &mut EmissionSpread,
        rng: &mut impl Rng,
        thickness: f32,
        direction_mode: EmitterDirectionMode,
    ) -> EmittedParticle {
        todo!()
    }
}

impl Default for Cone {
    fn default() -> Self {
        Self { angle: 0.5 }
    }
}

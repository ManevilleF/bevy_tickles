use crate::components::particle_emitter::emitter_shape::{EmittedParticle, Emitter};
use crate::{random_in_line, EmissionSpread, EmitterDirectionMode};
use bevy::prelude::Vec3;
use rand::Rng;
/// Emit particles from a line segment. The particles move in the emitter object’s upward (Y) direction.
#[derive(Debug, Clone)]
#[cfg_attr(feature = "inspector", derive(bevy_inspector_egui::Inspectable))]
pub struct Edge {
    /// The edge length
    pub length: f32,
}

impl Emitter for Edge {
    fn emit_random_particle(
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

    fn spread_particle(
        &self,
        spread: &mut EmissionSpread,
        rng: &mut impl Rng,
        thickness: f32,
        direction_mode: EmitterDirectionMode,
    ) -> EmittedParticle {
    }
}

impl Default for Edge {
    fn default() -> Self {
        Self { length: 1.0 }
    }
}

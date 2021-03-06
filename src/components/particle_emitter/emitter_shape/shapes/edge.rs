use crate::components::particle_emitter::emitter_shape::{EmittedParticle, Emitter};
use crate::{line_spread, random_in_line, EmissionSpread, EmitterDirectionMode};
use bevy::prelude::Vec3;
use rand::Rng;

/// Emit particles from a line segment. The particles move in the emitter object’s upward (Y) direction.
///
/// ## Spread
///
/// Axes:
/// * `x` - Not used
/// * `y` - Not used
/// * `z` - Spread amount
///
/// ### Missing Spread features:
///
/// * Non `uniform` spread is not available
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
        _rng: &mut impl Rng,
        thickness: f32,
        direction_mode: EmitterDirectionMode,
    ) -> EmittedParticle {
        let (_previous_index, index) = spread.update_index();
        let value = line_spread(self.length / 2.0, thickness, index.z);
        // TODO: implement non uniform
        EmittedParticle {
            position: Vec3::new(0.0, 0.0, value),
            direction: match direction_mode {
                EmitterDirectionMode::Automatic => Vec3::Y,
                EmitterDirectionMode::Fixed(dir) => dir,
            },
        }
    }
}

impl Default for Edge {
    fn default() -> Self {
        Self { length: 1.0 }
    }
}

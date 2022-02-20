use crate::components::particle_emitter::emitter_shape::{EmittedParticle, Emitter};
use crate::{line_spread, random_in_line, EmissionSpread, EmitterDirectionMode};
use bevy::prelude::Vec3;
use rand::Rng;

/// Initializes particles at randomly-sampled positions within a box and directs them out of one of the six box faces.
///
/// ## Spread
///
/// Axes:
/// * `x` - spread on the `x` axis
/// * `y` - spread on the `y` axis
/// * `z` - spread on the `z` axis
///
/// ### Missing Spread features:
///
/// * The `thickness` is not yet supported
/// * Non `uniform` spread is not available
#[derive(Debug, Clone)]
#[cfg_attr(feature = "inspector", derive(bevy_inspector_egui::Inspectable))]
pub struct Box {
    /// Box half extents
    pub extents: Vec3,
}

impl Emitter for Box {
    fn emit_random_particle(
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
            direction: Self::dir(direction_mode, position),
        }
    }

    fn spread_particle(
        &self,
        spread: &mut EmissionSpread,
        _rng: &mut impl Rng,
        _thickness: f32,
        direction_mode: EmitterDirectionMode,
    ) -> EmittedParticle {
        let (_previous_index, index) = spread.update_index();
        let position = Vec3::new(
            line_spread(self.extents.x, 1.0, index.x),
            line_spread(self.extents.y, 1.0, index.y),
            line_spread(self.extents.z, 1.0, index.z),
        );
        // TODO: support thickness
        // TODO: support non uniform spread
        EmittedParticle {
            position,
            direction: Self::dir(direction_mode, position),
        }
    }
}

impl Box {
    fn dir(direction_mode: EmitterDirectionMode, position: Vec3) -> Vec3 {
        match direction_mode {
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
        }
    }
}

impl Default for Box {
    fn default() -> Self {
        Self { extents: Vec3::ONE }
    }
}

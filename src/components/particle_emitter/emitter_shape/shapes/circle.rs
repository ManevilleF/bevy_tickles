use crate::components::particle_emitter::emitter_shape::{EmittedParticle, Emitter};
use crate::shapes::PI_2;
use crate::{radius_spread, random_in_radius, EmissionSpread, EmitterDirectionMode};
use bevy::prelude::Vec3;
use rand::Rng;

/// Initializes particles at randomly-sampled positions within a circle in the direction of the emitter’s up axis
#[derive(Debug, Clone)]
#[cfg_attr(feature = "inspector", derive(bevy_inspector_egui::Inspectable))]
pub struct Circle {
    /// Circle radius
    pub radius: f32,
}

impl Emitter for Circle {
    fn emit_random_particle(
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

    fn spread_particle(
        &self,
        spread: &mut EmissionSpread,
        rng: &mut impl Rng,
        thickness: f32,
        direction_mode: EmitterDirectionMode,
    ) -> EmittedParticle {
        let (previous_index, index) = spread.update_index();
        let theta = PI_2
            * if spread.spreads[1].uniform {
                index.y
            } else {
                rng.gen_range(previous_index.y.min(index.y)..=index.y.max(previous_index.y))
            };
        let range = if spread.spreads[2].uniform {
            radius_spread(self.radius, thickness, index.z)
        } else {
            random_in_radius(self.radius, thickness, rng)
        };
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

impl Default for Circle {
    fn default() -> Self {
        Self { radius: 1.0 }
    }
}

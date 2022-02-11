use crate::components::particle_emitter::emitter_shape::{EmittedParticle, Emitter};
use crate::shapes::PI_2;
use crate::{random_in_radius, EmissionSpread, EmitterDirectionMode};
use bevy::prelude::Vec3;
use rand::Rng;
use std::f32::consts::PI;

/// Initializes particles at randomly-sampled positions within a sphere and directs them outwards from the center
#[derive(Debug, Clone)]
#[cfg_attr(feature = "inspector", derive(bevy_inspector_egui::Inspectable))]
pub struct Sphere {
    /// Sphere radius
    pub radius: f32,
    /// Uses a hemisphere instead
    pub hemisphere: bool,
}

impl Emitter for Sphere {
    fn emit_random_particle(
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

    fn spread_particle(
        &self,
        spread: &mut EmissionSpread,
        thickness: f32,
        direction_mode: EmitterDirectionMode,
    ) -> EmittedParticle {
        todo!()
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

use crate::modifiers::ParticleModifier;
use crate::{Particle, RangeOrFixed};
use bevy::prelude::{Component, Reflect, Vec3};

/// Increases particle speed over time
#[derive(Debug, Copy, Clone, Default, Component, Reflect)]
#[cfg_attr(feature = "inspector", derive(bevy_inspector_egui::Inspectable))]
pub struct SpeedOverTime(pub f32);

impl ParticleModifier for SpeedOverTime {
    fn apply(&self, particle: &mut Particle, delta_time: f32) {
        particle.velocity += delta_time * self.0;
    }
}

/// Increases particle velocity over time
#[derive(Debug, Copy, Clone, Default, Component, Reflect)]
#[cfg_attr(feature = "inspector", derive(bevy_inspector_egui::Inspectable))]
pub struct VelocityOverTime(pub Vec3);

impl ParticleModifier for VelocityOverTime {
    fn apply(&self, particle: &mut Particle, delta_time: f32) {
        particle.velocity += delta_time * self.0;
    }
}

/// Increases particle angular velocity over time
///
/// Note: Will not work on particles set to align with their direction
#[derive(Debug, Copy, Clone, Default, Component, Reflect)]
#[cfg_attr(feature = "inspector", derive(bevy_inspector_egui::Inspectable))]
pub struct AngularVelocityOverTime(pub f32);

impl ParticleModifier for AngularVelocityOverTime {
    fn apply(&self, particle: &mut Particle, delta_time: f32) {
        particle.try_add_angular_velocity(delta_time * self.0);
    }
}

/// Changes particle velocity over its lifetime
#[derive(Debug, Copy, Clone, Default, Component, Reflect)]
#[cfg_attr(feature = "inspector", derive(bevy_inspector_egui::Inspectable))]
pub struct LinearVelocityOverLifeTime(RangeOrFixed<Vec3>);

impl ParticleModifier for LinearVelocityOverLifeTime {
    fn apply(&self, particle: &mut Particle, delta_time: f32) {
        particle.velocity += self.0.sample(particle.alive_time_ratio()) * delta_time;
    }
}

/// Allows particle to orbit around 3D axis
#[derive(Debug, Copy, Clone, Default, Component, Reflect)]
#[cfg_attr(feature = "inspector", derive(bevy_inspector_egui::Inspectable))]
pub struct OrbitalVelocityOverLifeTime {
    /// Orbital velocity of particles around the X, Y and Z axes.
    pub velocity: RangeOrFixed<Vec3>,
    /// The offset position of the center of orbit.
    pub offset: Vec3,
    // /// Radial velocity of particles away from/towards the center position
    // pub radial: f32,
}

impl ParticleModifier for OrbitalVelocityOverLifeTime {
    fn apply(&self, particle: &mut Particle, delta_time: f32) {
        let values = self.velocity.sample(particle.alive_time_ratio());
        let mut target_velocity = Vec3::ZERO;
        let translation = particle.translation - self.offset;
        if values.x.is_normal() {
            let (cos, sin) = (values.x.cos(), values.x.sin());
            target_velocity.y += translation.y * cos - translation.z * sin;
            target_velocity.z += translation.y.mul_add(sin, translation.z * cos);
        }
        if values.y.is_normal() {
            let (cos, sin) = (values.y.cos(), values.y.sin());
            target_velocity.z += translation.z * cos - translation.x * sin;
            target_velocity.x += translation.z.mul_add(sin, translation.x * cos);
        }
        if values.z.is_normal() {
            let (cos, sin) = (values.z.cos(), values.z.sin());
            target_velocity.x += translation.x * cos - translation.y * sin;
            target_velocity.y += translation.y.mul_add(sin, translation.y * cos);
        }
        particle.velocity += target_velocity * delta_time;
    }
}

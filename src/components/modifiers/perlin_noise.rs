use crate::modifiers::ParticleRngModifier;
use crate::{Particle, Vec3};
use bevy::math::{DVec2, DVec3};
use bevy::prelude::Component;
use noise::{NoiseFn, Perlin};
use rand::Rng;

/// Defines the quality of the perlin noise
#[derive(Debug, Copy, Clone)]
#[cfg_attr(feature = "inspector", derive(bevy_inspector_egui::Inspectable))]
pub enum NoiseQuality {
    /// 2D Noise (fast)
    Medium {
        /// Noise map bounds
        map_extents: DVec2,
    },
    ///3D Noise (slow)
    High {
        /// Noise map bounds
        map_extents: DVec3,
    },
}

/// Defines the quality of the perlin noise
#[derive(Debug, Copy, Clone)]
#[cfg_attr(feature = "inspector", derive(bevy_inspector_egui::Inspectable))]
pub enum VelocityInfluence {
    /// One noise value will be applied to all axis
    Uniform(f32),
    /// 3 noises values will be generated, each applied to an axis
    SeparateAxis(Vec3),
}

/// Perlin Noise modifier for particles
#[derive(Debug, Clone, Component)]
#[cfg_attr(feature = "inspector", derive(bevy_inspector_egui::Inspectable))]
pub struct PerlinNoise {
    #[cfg_attr(feature = "inspector", inspectable(ignore))]
    noise: Perlin,
    /// Noise quality
    pub quality: NoiseQuality,
    /// Influence factor over particles velocity, `0` means no influence
    pub velocity_influence: VelocityInfluence,
    /// Influence factor over particles `z` rotation, `0` means no influence
    ///
    /// Note: won't work on direction aligned particles
    pub rotation_influence: f32,
    /// Influence factor over particles size, `0` means no influence
    pub size_influence: f32,
}

impl Default for NoiseQuality {
    fn default() -> Self {
        Self::Medium {
            map_extents: DVec2::splat(5.0),
        }
    }
}

impl Default for PerlinNoise {
    fn default() -> Self {
        Self {
            noise: Perlin::new(),
            quality: Default::default(),
            velocity_influence: VelocityInfluence::Uniform(1.0),
            rotation_influence: 0.0,
            size_influence: 0.0,
        }
    }
}

impl PerlinNoise {
    pub(crate) fn get_value(&self, rng: &mut impl Rng) -> f32 {
        match self.quality {
            NoiseQuality::Medium { map_extents } => {
                let coords = [
                    rng.gen_range(-map_extents.x..=map_extents.x),
                    rng.gen_range(-map_extents.y..=map_extents.y),
                ];
                self.noise.get(coords) as f32
            }
            NoiseQuality::High { map_extents } => {
                let coords = [
                    rng.gen_range(-map_extents.x..=map_extents.x),
                    rng.gen_range(-map_extents.y..=map_extents.y),
                    rng.gen_range(-map_extents.z..=map_extents.z),
                ];
                self.noise.get(coords) as f32
            }
        }
    }
}

impl ParticleRngModifier for PerlinNoise {
    fn apply(&self, rng: &mut impl Rng, particle: &mut Particle, delta_time: f32) {
        let x = self.get_value(rng);
        particle.try_rotate(x * delta_time * self.rotation_influence);
        particle.size += x * delta_time * self.size_influence;
        match self.velocity_influence {
            VelocityInfluence::Uniform(influence) => particle.velocity += x * influence,
            VelocityInfluence::SeparateAxis(influence) => {
                particle.velocity +=
                    influence * Vec3::new(x, self.get_value(rng), self.get_value(rng));
            }
        }
    }
}

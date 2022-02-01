use bevy::prelude::{Color, Vec3};

/// Single particle representation
#[derive(Debug, Clone)]
pub struct Particle {
    /// 3D position
    pub translation: Vec3,
    /// 3D rotation
    pub rotation: f32,
    /// Size of the particle
    pub size: f32,
    /// Lifetime of the particle
    pub lifetime: f32,
    /// Particle color
    pub color: Color,
    /// Particle 3D velocity
    pub velocity: Vec3,
    /// Particle 3D angular velocity
    pub angular_velocity: f32,
}

impl Default for Particle {
    fn default() -> Self {
        Self {
            translation: Default::default(),
            rotation: 0.0,
            size: 1.0,
            lifetime: 1.0,
            color: Default::default(),
            velocity: Default::default(),
            angular_velocity: 0.0,
        }
    }
}

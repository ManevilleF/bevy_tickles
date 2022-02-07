use bevy::prelude::{Color, Mat4, Reflect, Vec3};
use bevy::reflect::FromReflect;

/// Single particle representation
#[derive(Debug, Clone, Reflect)]
pub struct Particle {
    /// 3D position
    pub translation: Vec3,
    /// 1D rotation as the particle will always face the camera
    pub rotation: f32,
    /// Size of the particle
    pub size: f32,
    /// Lifetime of the particle
    pub lifetime: f32,
    /// Start Lifetime of the particle
    pub start_lifetime: f32,
    /// Particle color
    pub color: Color,
    /// Particle 3D velocity
    pub velocity: Vec3,
    /// Particle 3D angular velocity
    pub angular_velocity: f32,
    pub(crate) start_direction: Vec3,
}

impl Default for Particle {
    fn default() -> Self {
        Self {
            translation: Vec3::ZERO,
            rotation: 0.0,
            size: 1.0,
            lifetime: 1.0,
            start_lifetime: 1.0,
            color: Default::default(),
            velocity: Default::default(),
            angular_velocity: 0.0,
            start_direction: Vec3::Y,
        }
    }
}

impl Particle {
    /// Retrieves the current particle speed, computed from the `length` of its `velocity`
    #[must_use]
    #[inline]
    pub fn speed(&self) -> f32 {
        self.velocity.length() // TODO optimize with `length_squared`
    }

    /// How long was the particle alive
    #[must_use]
    #[inline]
    pub fn alive_time(&self) -> f32 {
        self.start_lifetime - self.lifetime
    }

    /// How long was the particle alive compared to its original lifetime (between 0 and 1)
    #[must_use]
    #[inline]
    pub fn alive_time_ratio(&self) -> f32 {
        (self.start_lifetime - self.lifetime) / self.start_lifetime
    }

    /// returns `self` with its translation transformed by `matrix`
    #[must_use]
    #[inline]
    pub fn transformed(self, matrix: &Mat4) -> Self {
        Self {
            translation: matrix.transform_point3(self.translation),
            ..self
        }
    }

    /// Retrieves either the current direction from `velocity` or uses the initial direction of the particle
    #[must_use]
    #[inline]
    pub fn non_zero_direction(&self) -> Vec3 {
        self.velocity
            .try_normalize()
            .unwrap_or(self.start_direction)
    }
}

impl FromReflect for Particle {
    fn from_reflect(reflect: &dyn Reflect) -> Option<Self> {
        reflect.any().downcast_ref::<Self>().cloned()
    }
}

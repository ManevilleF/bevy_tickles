use crate::Particle;
use bevy::ecs::reflect::ReflectComponent;
use bevy::math::const_vec3;
use bevy::prelude::{Component, GlobalTransform, Reflect, Vec3};
use bevy::render::primitives::Aabb;
use std::ops::Deref;

/// Particle System simulation container
#[derive(Debug, Clone, Default, Component, Reflect)]
#[reflect(Component)]
#[cfg_attr(feature = "inspector", derive(bevy_inspector_egui::Inspectable))]
pub struct ParticleSystem {
    /// If enabled, the particles won't be stuck to the particle system entity
    pub world_space: bool,
    /// Every simulated particle
    #[cfg_attr(feature = "inspector", inspectable(ignore))]
    pub(crate) particles: Vec<Particle>,
}

impl Deref for ParticleSystem {
    type Target = Vec<Particle>;

    fn deref(&self) -> &Self::Target {
        &self.particles
    }
}

impl ParticleSystem {
    /// Creates a particle system in world space
    #[inline]
    #[must_use]
    pub const fn world_space() -> Self {
        Self {
            world_space: true,
            particles: vec![],
        }
    }

    // TODO: Benchmark this and try with `retain_mut` equivalent
    pub(crate) fn update(&mut self, delta_time: f32) {
        for particle in &mut self.particles {
            particle.update(delta_time);
        }
        self.particles.retain(|particle| particle.lifetime > 0.);
    }

    /// Computes the complete bounding box of the particle system
    #[must_use]
    pub fn compute_aabb(&self, transform: &GlobalTransform) -> Option<Aabb> {
        const VEC3_MIN: Vec3 = const_vec3!([std::f32::MIN, std::f32::MIN, std::f32::MIN]);
        const VEC3_MAX: Vec3 = const_vec3!([std::f32::MAX, std::f32::MAX, std::f32::MAX]);

        if self.particles.is_empty() {
            return None;
        }
        let mut minimum = VEC3_MAX;
        let mut maximum = VEC3_MIN;
        let matrix = self
            .world_space
            .then(|| transform.compute_matrix().inverse());

        for p in &self.particles {
            minimum =
                minimum.min(matrix.map_or(p.translation, |m| m.transform_point3(p.translation)));
            maximum =
                maximum.max(matrix.map_or(p.translation, |m| m.transform_point3(p.translation)));
        }
        Some(Aabb::from_min_max(minimum, maximum))
    }

    /// Adds a particle to the system
    ///
    /// # Arguments
    ///
    /// * `particle` - The particle to add
    /// * `transform` - The particle system global transform, which will be used to compute the
    /// real particle `translation` in [`ParticleSystem::world_space`] mode
    pub fn push(&mut self, mut particle: Particle, transform: &GlobalTransform) {
        if self.world_space {
            let matrix = transform.compute_matrix();
            particle = particle.transformed(&matrix);
        }
        self.particles.push(particle);
    }

    /// Adds multiple particles to the system
    ///
    /// # Arguments
    ///
    /// * `particles` - An interator of the particles to add
    /// * `transform` - The particle system global transform, which will be used to compute the
    /// real particle `translation` in [`ParticleSystem::world_space`] mode
    pub fn extend(
        &mut self,
        particles: impl Iterator<Item = Particle>,
        transform: &GlobalTransform,
    ) {
        if self.world_space {
            let matrix = transform.compute_matrix();
            self.particles
                .extend(particles.map(|p| p.transformed(&matrix)));
        } else {
            self.particles.extend(particles);
        }
    }
}

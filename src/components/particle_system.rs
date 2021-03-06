use crate::Particle;
use bevy::ecs::reflect::ReflectComponent;
use bevy::prelude::{Component, GlobalTransform, Reflect, Vec3};
use bevy::render::primitives::Aabb;
use itertools::{Itertools, MinMaxResult};
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
    // TODO: Benchmark this and try with `retain_mut` equivalent
    pub(crate) fn update(&mut self, delta_time: f32) {
        for particle in &mut self.particles {
            particle.update(delta_time);
        }
        self.particles.retain(|particle| particle.lifetime > 0.);
    }

    /// Computes the complete bounding box of the particle system
    #[must_use]
    pub fn compute_aabb(&self) -> Option<Aabb> {
        if self.particles.is_empty() {
            return None;
        }
        let (x_min, x_max) = match self.iter().map(|p| p.translation.x).minmax() {
            MinMaxResult::NoElements => return None,
            MinMaxResult::OneElement(p) => (p, p),
            MinMaxResult::MinMax(a, b) => (a, b),
        };
        let (y_min, y_max) = match self.iter().map(|p| p.translation.y).minmax() {
            MinMaxResult::NoElements => return None,
            MinMaxResult::OneElement(p) => (p, p),
            MinMaxResult::MinMax(a, b) => (a, b),
        };
        let (z_min, z_max) = match self.iter().map(|p| p.translation.z).minmax() {
            MinMaxResult::NoElements => return None,
            MinMaxResult::OneElement(p) => (p, p),
            MinMaxResult::MinMax(a, b) => (a, b),
        };
        let min = Vec3::new(x_min, y_min, z_min);
        let max = Vec3::new(x_max, y_max, z_max);
        // TODO: check if this works before enabling
        // if self.world_space {
        //     let matrix = transform.compute_matrix().inverse();
        //     min = matrix.transform_point3(min);
        //     max = matrix.transform_point3(max);
        // }
        Some(Aabb::from_min_max(min, max))
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

#![allow(clippy::needless_pass_by_value)]
use crate::modifiers::{ParticleModifier, ParticleRngModifier, ParticleSystemModifier};
use crate::{ParticleEmitter, ParticleParams, ParticleRng, ParticleSystem};
use bevy::prelude::*;
use bevy::render::primitives::Aabb;

pub fn update_particle_system(mut query: Query<&mut ParticleSystem>, time: Res<Time>) {
    let delta = time.delta_seconds();
    for mut particle_system in query.iter_mut() {
        particle_system.update(delta);
    }
}

pub fn emit_particles(
    mut query: Query<(
        &mut ParticleSystem,
        &mut ParticleEmitter,
        &mut ParticleRng,
        &ParticleParams,
        &GlobalTransform,
    )>,
    time: Res<Time>,
) {
    let delta = time.delta_seconds();
    for (mut particle_system, mut emitter, mut rng, params, transform) in query.iter_mut() {
        particle_system.extend(
            emitter
                .emit(delta, rng.rng())
                .into_iter()
                .map(|e| params.get_particle(e.position, e.direction, rng.rng())),
            transform,
        );
    }
}

pub fn apply_system_modifier<M>(mut query: Query<(&mut ParticleSystem, &M)>, time: Res<Time>)
where
    M: ParticleSystemModifier,
{
    let delta = time.delta_seconds();
    for (mut particle_system, modifier) in query.iter_mut() {
        modifier.apply(&mut particle_system, delta);
    }
}
pub fn apply_modifier<M>(mut query: Query<(&mut ParticleSystem, &M)>, time: Res<Time>)
where
    M: ParticleModifier,
{
    let delta = time.delta_seconds();
    for (mut particle_system, modifier) in query.iter_mut() {
        for particle in &mut particle_system.particles {
            modifier.apply(particle, delta);
        }
    }
}

pub fn apply_rng_modifier<M>(
    mut query: Query<(&mut ParticleSystem, &mut ParticleRng, &M)>,
    time: Res<Time>,
) where
    M: ParticleRngModifier,
{
    let delta = time.delta_seconds();
    for (mut particle_system, mut rng, modifier) in query.iter_mut() {
        for particle in &mut particle_system.particles {
            modifier.apply(rng.rng(), particle, delta);
        }
    }
}

pub fn compute_particles_aabb(mut query: Query<(&mut Aabb, &ParticleSystem)>) {
    for (mut aabb, particles) in query.iter_mut() {
        if let Some(bounding_box) = particles.compute_aabb() {
            *aabb = bounding_box;
        }
    }
}

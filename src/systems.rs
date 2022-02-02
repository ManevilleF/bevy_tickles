#![allow(clippy::needless_pass_by_value)]
use crate::modifiers::ParticleSystemModifier;
use crate::{ParticleEmitter, ParticleModifier, ParticleParams, ParticleRng, ParticleSystem};
use bevy::prelude::*;

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
        for particle in particle_system.particles_mut() {
            modifier.apply(particle, delta);
        }
    }
}

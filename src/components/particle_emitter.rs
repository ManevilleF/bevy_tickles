use crate::RangeOrFixed;
use bevy::ecs::reflect::ReflectComponent;
use bevy::prelude::{Component, Reflect, Vec3};
use bevy::reflect::FromReflect;
use rand::Rng;
use std::f32::consts::PI;

const PI_2: f32 = PI * 2.0;

#[derive(Debug, Clone)]
pub struct EmittedParticle {
    pub position: Vec3,
    pub direction: Vec3,
}

/// Possible [`ParticleEmitter`] shapes
#[derive(Debug, Clone, Reflect)]
#[cfg_attr(feature = "inspector", derive(bevy_inspector_egui::Inspectable))]
pub enum EmitterShape {
    /// Initializes particles at randomly-sampled positions within a sphere and directs them outwards from the center
    Sphere {
        /// Sphere radius
        radius: f32,
        /// Emit only on the sphere edges
        edge_only: bool,
    },
    // TODO: implement half sphere
    // /// Initializes particles at randomly-sampled positions within a semi sphere and directs them outwards from the center
    // HalfSphere {
    //     /// Sphere radius
    //     radius: f32,
    // },
    // /// Initializes particles at randomly-sampled positions on a ball and directs them outwards from the center
    /// Initializes particles at randomly-sampled positions within a circle in the direction of the emitter’s up axis
    Circle {
        /// Circle radius
        radius: f32,
    },
    /// Initializes particles at randomly-sampled positions within a box and directs them out of one of the six box faces.
    Box {
        /// Box half extents
        extents: Vec3,
    },
    /// Initializes particles at the tip of a cone and directs them at random angles out of the cone.
    /// The cone is oriented along the up axis of the emitter.
    Cone {
        /// The cone angle, between `0` and `1` representing between 0 and 90 degrees
        angle: f32,
    },
    // TODO: implement mesh
    // Mesh {
    //     mesh: Mesh
    // }
}

/// Describes a single Particle emitter burst
#[derive(Debug, Default, Clone, Reflect)]
#[cfg_attr(feature = "inspector", derive(bevy_inspector_egui::Inspectable))]
pub struct Burst {
    /// Time after the start of the emission
    pub time: f32,
    /// The count of particles to be emitted
    pub count: RangeOrFixed<usize>,
}

/// Emitter of particles, works with [`ParticleSystem`]
#[derive(Debug, Clone, Component, Reflect)]
#[reflect(Component)]
#[cfg_attr(feature = "inspector", derive(bevy_inspector_egui::Inspectable))]
pub struct ParticleEmitter {
    /// The shape of the emitter
    pub shape: EmitterShape,
    /// The rate of particle emission over time (`1.0` means 1 particle per second)
    pub rate: f32,
    /// Custom bursts of particle emission
    pub bursts: Vec<Burst>,
    /// time since first tick
    pub current_delta_time: f32,
    /// Time since last `rate` emission
    pub last_emitted_delta_time: f32,
}

impl Default for EmitterShape {
    fn default() -> Self {
        Self::Sphere {
            radius: 1.0,
            edge_only: false,
        }
    }
}

impl EmitterShape {
    pub(crate) fn emit_particle(&self, rng: &mut impl Rng) -> EmittedParticle {
        match self {
            EmitterShape::Sphere { radius, edge_only } => {
                let range = if *edge_only {
                    *radius
                } else {
                    rng.gen_range(0.0..=*radius)
                };
                let theta = PI_2 * rng.gen_range(0.0..=1.0);
                let phi = PI * rng.gen_range(0.0..=1.0);
                let sin_phi = phi.sin();
                let position = Vec3::new(
                    range * sin_phi * theta.cos(),
                    range * sin_phi * theta.sin(),
                    range * phi.cos(),
                );
                EmittedParticle {
                    position,
                    direction: position,
                }
            }
            EmitterShape::Circle { radius } => {
                let range = rng.gen_range(0.0..=*radius);
                let theta = PI_2 * rng.gen_range(0.0..=1.0);
                let position = Vec3::new(range * theta.cos(), 0., range * theta.sin());
                EmittedParticle {
                    position,
                    direction: position,
                }
            }
            EmitterShape::Box { extents } => {
                let position = Vec3::new(
                    rng.gen_range(-extents.x..=extents.x),
                    rng.gen_range(-extents.y..=extents.y),
                    rng.gen_range(-extents.z..=extents.z),
                );
                EmittedParticle {
                    position,
                    direction: match position.abs().to_array() {
                        [x, y, z] if x > y && x > z => {
                            if position.z > 0. {
                                Vec3::X
                            } else {
                                -Vec3::X
                            }
                        }
                        [x, y, z] if y > x && y > z => {
                            if position.y > 0. {
                                Vec3::Y
                            } else {
                                -Vec3::Y
                            }
                        }
                        _ => {
                            if position.z > 0. {
                                Vec3::Z
                            } else {
                                -Vec3::Z
                            }
                        }
                    },
                }
            }
            EmitterShape::Cone { angle } => {
                let delta = rng.gen_range(0.0..=angle.clamp(0., 1.));
                EmittedParticle {
                    position: Vec3::ZERO,
                    direction: Vec3::new(delta, 1., delta),
                }
            }
        }
    }
}

impl FromReflect for Burst {
    fn from_reflect(reflect: &dyn Reflect) -> Option<Self> {
        reflect.any().downcast_ref::<Self>().cloned()
    }
}

impl Default for ParticleEmitter {
    fn default() -> Self {
        Self {
            shape: Default::default(),
            rate: 5.0,
            bursts: vec![],
            current_delta_time: 0.0,
            last_emitted_delta_time: 0.0,
        }
    }
}

impl ParticleEmitter {
    /// Computes particles to emit
    pub fn emit(&mut self, delta_time: f32, rng: &mut impl Rng) -> Vec<EmittedParticle> {
        // bursts
        let mut emission_count = self
            .bursts
            .iter()
            .filter(|b| {
                b.time >= self.current_delta_time && b.time < self.current_delta_time + delta_time
            })
            .map(|b| b.count.evaluate(rng))
            .sum();
        self.current_delta_time += delta_time;
        // emission over time
        if self.rate > 0.0 {
            let delta_per_particle = 1.0 / self.rate;
            let delay_since_emission = self.current_delta_time - self.last_emitted_delta_time;
            let particles_to_emit = (delay_since_emission / delta_per_particle) as usize;
            emission_count += particles_to_emit;
            self.last_emitted_delta_time += delta_per_particle * particles_to_emit as f32;
        }
        (0..emission_count)
            .map(|_| self.shape.emit_particle(rng))
            .collect()
    }
}

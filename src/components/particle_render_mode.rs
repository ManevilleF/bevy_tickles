use crate::{Particle, Vec3};
use bevy::ecs::reflect::ReflectComponent;
use bevy::prelude::{Component, GlobalTransform, Quat, Reflect, Transform};

/// Defines how the particle billboard is aligned
#[derive(Debug, Copy, Clone, Reflect)]
#[cfg_attr(feature = "inspector", derive(bevy_inspector_egui::Inspectable))]
pub enum BillBoardAlignment {
    /// Particles face the Camera plane
    View,
    /// Particles are aligned with the world axes
    World,
    /// Particles are aligned to the local transform
    Local,
    /// Particles face the direct position of the Camera transform
    Facing,
    /// Particles face in the direction of their velocity. If the particles don't move, the original
    /// direction from the emitter is used
    Direction,
}

#[derive(Debug, Clone, Component, Reflect)]
#[cfg_attr(feature = "inspector", derive(bevy_inspector_egui::Inspectable))]
#[reflect(Component)]
/// Defines how the particle image is rendered
pub enum ParticleRenderMode {
    /// The particles render as billboards and face the direction you specify
    BillBoard {
        /// Billboard alignment
        alignment: BillBoardAlignment,
    },
    /// The particle is upright on the world Y-axis, but turns to face the Camera.
    VerticalBillboard,
    /// The particle plane is parallel to the XZ “floor” plane.
    HorizontalBillBoard,
}

impl Default for BillBoardAlignment {
    fn default() -> Self {
        Self::View
    }
}

impl Default for ParticleRenderMode {
    fn default() -> Self {
        Self::BillBoard {
            alignment: BillBoardAlignment::default(),
        }
    }
}

impl ParticleRenderMode {
    #[inline]
    #[must_use]
    /// [`Self::VerticalBillboard`]
    pub const fn vertical() -> Self {
        Self::VerticalBillboard
    }

    #[inline]
    #[must_use]
    /// [`Self::HorizontalBillboard`]
    pub const fn horizontal() -> Self {
        Self::VerticalBillboard
    }

    #[inline]
    #[must_use]
    /// [`Self::Billboard`]
    pub const fn billboard(alignment: BillBoardAlignment) -> Self {
        Self::BillBoard { alignment }
    }

    pub(crate) fn apply_to_particle(
        &self,
        particle: &Particle,
        transform: &mut Transform,
        camera_transform: &GlobalTransform,
    ) {
        match self {
            ParticleRenderMode::HorizontalBillBoard => {
                transform.rotation = Quat::from_rotation_x(1.5);
            }
            ParticleRenderMode::VerticalBillboard => {
                let delta = transform.translation - camera_transform.translation;
                transform.rotation = Quat::from_rotation_y(delta.x.atan2(delta.z));
            }
            ParticleRenderMode::BillBoard { alignment } => match alignment {
                BillBoardAlignment::View => {
                    transform.rotation = -camera_transform.rotation;
                }
                BillBoardAlignment::World => {
                    transform.rotation = Quat::IDENTITY;
                }
                BillBoardAlignment::Local => (),
                BillBoardAlignment::Facing => {
                    transform.look_at(camera_transform.translation, Vec3::Y);
                }
                BillBoardAlignment::Direction => {
                    transform.look_at(
                        particle.translation + particle.non_zero_direction(),
                        Vec3::Y,
                    );
                }
            },
        };
        transform.rotation *= Quat::from_rotation_z(particle.rotation());
    }
}

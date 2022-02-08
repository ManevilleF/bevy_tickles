use crate::{rotation_forward, Particle};
use bevy::ecs::reflect::ReflectComponent;
use bevy::prelude::{Component, EulerRot, GlobalTransform, Quat, Reflect, Transform};

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
        let (x, y) = match self {
            ParticleRenderMode::HorizontalBillBoard => (1.5, 0.0),
            ParticleRenderMode::VerticalBillboard => {
                let (_x, y, _z) = rotation_forward(
                    (camera_transform.translation - transform.translation).normalize_or_zero(),
                )
                .to_euler(EulerRot::XYZ);
                (0.0, y)
            }
            ParticleRenderMode::BillBoard { alignment } => match alignment {
                BillBoardAlignment::View => {
                    let (x, y, _z) =
                        rotation_forward(camera_transform.local_z()).to_euler(EulerRot::XYZ);
                    (x, y)
                }
                BillBoardAlignment::World => (0., 0.),
                BillBoardAlignment::Local => {
                    let (x, y, _z) = transform.rotation.to_euler(EulerRot::XYZ);
                    (x, y)
                }
                BillBoardAlignment::Facing => {
                    let (x, y, _z) = rotation_forward(
                        (camera_transform.translation - transform.translation).normalize_or_zero(),
                    )
                    .to_euler(EulerRot::XYZ);
                    (x, y)
                }
                BillBoardAlignment::Direction => {
                    let (x, y, _z) =
                        rotation_forward(particle.non_zero_direction()).to_euler(EulerRot::XYZ);
                    (x, y)
                }
            },
        };
        transform.rotation = Quat::from_euler(EulerRot::XYZ, x, y, particle.rotation());
    }
}

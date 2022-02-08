mod color_gradient;
mod color_or_gradient;
mod range_or_fixed;

use crate::Vec3;
use bevy::math::{Mat3, Quat};
pub use color_gradient::ColorGradient;
pub use color_or_gradient::ColorOrGradient;
pub use range_or_fixed::RangeOrFixed;

pub(crate) fn rotation_forward(forward: Vec3) -> Quat {
    let forward = forward.normalize();
    let right = Vec3::Y.cross(forward).normalize();
    let up = forward.cross(right);
    Quat::from_mat3(&Mat3::from_cols(right, up, forward))
}

mod color;
mod range_or_fixed;

use crate::Vec3;
use bevy::math::{Mat3, Quat};
pub use color::ColorGradient;
pub use range_or_fixed::*;

pub(crate) fn rotation_forward(forward: Vec3) -> Quat {
    let forward = forward.normalize();
    let right = Vec3::Y.cross(forward).normalize();
    let up = forward.cross(right).normalize();
    Quat::from_mat3(&Mat3::from_cols(right, up, forward))
}

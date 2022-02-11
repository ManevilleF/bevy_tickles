mod color_gradient;
mod color_or_gradient;
mod range_or_fixed;

use crate::Vec3;
use bevy::math::{Mat3, Quat};
pub use color_gradient::ColorGradient;
pub use color_or_gradient::ColorOrGradient;
use rand::Rng;
pub use range_or_fixed::RangeOrFixed;

pub(crate) fn rotation_forward(forward: Vec3) -> Quat {
    let forward = forward.normalize();
    let right = Vec3::Y.cross(forward).normalize();
    let up = forward.cross(right);
    Quat::from_mat3(&Mat3::from_cols(right, up, forward))
}

pub(crate) fn random_in_radius(radius: f32, thickness: f32, rng: &mut impl Rng) -> f32 {
    let ratio = (1.0 - thickness).clamp(0.0, 1.0);
    rng.gen_range((radius * ratio)..=radius)
}

pub(crate) fn random_in_line(half_extent: f32, thickness: f32, rng: &mut impl Rng) -> f32 {
    let point = random_in_radius(half_extent, thickness, rng);
    if rng.gen_range(0..=1) == 0 {
        point
    } else {
        -point
    }
}

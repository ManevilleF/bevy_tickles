mod color_gradient;
mod color_or_gradient;
mod range_or_fixed;

pub use color_gradient::ColorGradient;
pub use color_or_gradient::ColorOrGradient;
use rand::Rng;
pub use range_or_fixed::RangeOrFixed;

pub(crate) fn radius_spread(radius: f32, thickness: f32, spread_amount: f32) -> f32 {
    let ratio = (1.0 - thickness).clamp(0.0, 1.0);
    let min = radius * ratio;
    (radius - min).mul_add(spread_amount.clamp(0.0, 1.0), min)
}

pub(crate) fn random_in_radius(radius: f32, thickness: f32, rng: &mut impl Rng) -> f32 {
    let ratio = (1.0 - thickness).clamp(0.0, 1.0);
    rng.gen_range((radius * ratio)..=radius)
}

pub(crate) fn random_in_line(half_extent: f32, thickness: f32, rng: &mut impl Rng) -> f32 {
    let point = random_in_radius(half_extent, thickness, rng);
    if rng.gen_ratio(1, 2) {
        point
    } else {
        -point
    }
}

pub(crate) fn line_spread(half_extent: f32, thickness: f32, spread_amount: f32) -> f32 {
    if spread_amount > 0.5 {
        radius_spread(half_extent, thickness, spread_amount * 2.0 - 1.0)
    } else {
        -radius_spread(half_extent, thickness, -(spread_amount - 1.0) * 2.0 - 1.0)
    }
}

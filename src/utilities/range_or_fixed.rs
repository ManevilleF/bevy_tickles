use crate::utilities::ColorGradient;
use bevy::prelude::{Color, Reflect};
use rand::Rng;
use std::fmt::Debug;

/// Either a fixed value or a range
#[derive(Copy, Clone, Debug, Reflect)]
#[cfg_attr(feature = "inspector", derive(bevy_inspector_egui::Inspectable))]
pub enum RangeOrFixed<T: Copy + Clone + Debug + Send + Sync + Reflect + Default> {
    /// Fixed value
    Fixed(T),
    /// Linear Range value
    Range {
        /// Start value
        min: T,
        /// End Value
        max: T,
    },
    // TODO: Add more range options
}

impl Default for RangeOrFixed<f32> {
    fn default() -> Self {
        Self::Fixed(1.0)
    }
}

impl From<f32> for RangeOrFixed<f32> {
    fn from(v: f32) -> Self {
        Self::Fixed(v)
    }
}

impl Default for RangeOrFixed<usize> {
    fn default() -> Self {
        Self::Fixed(0)
    }
}

impl From<usize> for RangeOrFixed<usize> {
    fn from(v: usize) -> Self {
        Self::Fixed(v)
    }
}

impl Default for RangeOrFixed<Color> {
    fn default() -> Self {
        Self::Fixed(Color::default())
    }
}

impl RangeOrFixed<f32> {
    /// Evaluates the float value using `rng`
    pub fn evaluate(&self, rng: &mut impl Rng) -> f32 {
        match self {
            RangeOrFixed::Fixed(v) => *v,
            RangeOrFixed::Range { min, max } => rng.gen_range(*min..=*max),
        }
    }
}

impl RangeOrFixed<usize> {
    /// Evaluates the usize value using `rng`
    pub fn evaluate(&self, rng: &mut impl Rng) -> usize {
        match self {
            RangeOrFixed::Fixed(v) => *v,
            RangeOrFixed::Range { min, max } => rng.gen_range(*min..=*max),
        }
    }
}

impl RangeOrFixed<Color> {
    /// Evaluates the color value using `rng`
    pub fn evaluate(&self, rng: &mut impl Rng) -> Color {
        match self {
            RangeOrFixed::Fixed(v) => *v,
            RangeOrFixed::Range { min, max } => {
                let delta: f32 = rng.gen_range(0.0..=1.0);
                let gradient = ColorGradient::from((*min, *max));
                gradient.evaluate_linear(delta)
            }
        }
    }
}

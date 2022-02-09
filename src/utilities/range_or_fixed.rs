use bevy::prelude::Reflect;
use rand::Rng;
use std::fmt::Debug;
use std::ops::RangeInclusive;

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

impl From<RangeInclusive<f32>> for RangeOrFixed<f32> {
    fn from(range: RangeInclusive<f32>) -> Self {
        Self::Range {
            min: *range.start(),
            max: *range.end(),
        }
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

impl From<RangeInclusive<usize>> for RangeOrFixed<usize> {
    fn from(range: RangeInclusive<usize>) -> Self {
        Self::Range {
            min: *range.start(),
            max: *range.end(),
        }
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

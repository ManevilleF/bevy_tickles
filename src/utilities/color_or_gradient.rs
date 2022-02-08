use crate::utilities::ColorGradient;
use bevy::prelude::{Color, Reflect};

/// Either a fixed color or a color gradient
#[derive(Debug, Clone, Reflect)]
#[cfg_attr(feature = "inspector", derive(bevy_inspector_egui::Inspectable))]
pub enum ColorOrGradient {
    /// Single color
    FixedColor(Color),
    /// Color gradient
    Gradient(ColorGradient),
}

impl From<Color> for ColorOrGradient {
    fn from(color: Color) -> Self {
        Self::FixedColor(color)
    }
}

impl From<ColorGradient> for ColorOrGradient {
    fn from(gradient: ColorGradient) -> Self {
        Self::Gradient(gradient)
    }
}

impl Default for ColorOrGradient {
    fn default() -> Self {
        Self::FixedColor(Color::WHITE)
    }
}

impl ColorOrGradient {
    /// Evaluates a color with `delta` (between 0 and 1)
    ///
    /// If `self` is a [`ColorOrGradient::FixedColor`] the color is simply returned
    #[must_use]
    pub fn evaluate(&self, delta: f32) -> Color {
        match self {
            ColorOrGradient::FixedColor(c) => *c,
            ColorOrGradient::Gradient(g) => g.evaluate(delta),
        }
    }
}

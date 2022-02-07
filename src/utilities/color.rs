use bevy::prelude::{Color, Reflect, Vec4};

/// Defines a simple color gradient with only two keys
// TODO: Improve this a lot
#[derive(Copy, Clone, Default, Debug, Reflect)]
#[cfg_attr(feature = "inspector", derive(bevy_inspector_egui::Inspectable))]
pub struct ColorGradient {
    /// Start value
    pub start: Color,
    /// End value
    pub end: Color,
}

impl From<(Color, Color)> for ColorGradient {
    fn from((start, end): (Color, Color)) -> Self {
        Self { start, end }
    }
}

impl ColorGradient {
    /// Evaluates a color linearly
    #[must_use]
    pub fn evaluate_linear(&self, delta: f32) -> Color {
        let min = Vec4::from(self.start.as_rgba());
        let max = Vec4::from(self.end.as_rgba());
        Color::from(min + (max - min) * delta.clamp(0.0, 1.0))
    }
}

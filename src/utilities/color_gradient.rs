use bevy::prelude::{Color, Reflect, Vec4};
use bevy::reflect::FromReflect;

#[derive(Debug, Default, Copy, Clone, Reflect)]
#[cfg_attr(feature = "inspector", derive(bevy_inspector_egui::Inspectable))]
struct GradientPoint {
    pub pos: f32,
    pub color: Color,
}

/// Color gradient
#[derive(Clone, Debug, Reflect)]
#[cfg_attr(feature = "inspector", derive(bevy_inspector_egui::Inspectable))]
pub struct ColorGradient {
    points: Vec<GradientPoint>,
}

impl From<(Color, Color)> for ColorGradient {
    fn from((start, end): (Color, Color)) -> Self {
        Self {
            points: vec![
                GradientPoint {
                    pos: 0.0,
                    color: start,
                },
                GradientPoint {
                    pos: 1.0,
                    color: end,
                },
            ],
        }
    }
}

impl FromReflect for GradientPoint {
    fn from_reflect(reflect: &dyn Reflect) -> Option<Self> {
        reflect.any().downcast_ref::<Self>().copied()
    }
}
impl From<Color> for ColorGradient {
    fn from(color: Color) -> Self {
        Self::from((color, color))
    }
}

impl From<Vec<Color>> for ColorGradient {
    fn from(colors: Vec<Color>) -> Self {
        let delta = 1.0 / colors.len() as f32;
        let points = colors
            .into_iter()
            .enumerate()
            .map(|(i, color)| GradientPoint {
                pos: i as f32 * delta,
                color,
            })
            .collect();
        Self { points }
    }
}

impl Default for ColorGradient {
    fn default() -> Self {
        Self::from(Color::WHITE)
    }
}

impl ColorGradient {
    /// Initializes an empty gradient
    #[must_use]
    #[inline]
    pub const fn empty() -> Self {
        Self { points: vec![] }
    }

    /// Linearly interpolates between two colors
    #[must_use]
    pub fn sample_color(a: Color, b: Color, delta: f32) -> Color {
        let min = Vec4::from(a.as_rgba());
        let max = Vec4::from(b.as_rgba());
        Color::from(min + (max - min) * delta.clamp(0.0, 1.0))
    }

    /// Rainbow gradient
    #[must_use]
    pub fn rainbow() -> Self {
        Self::empty()
            .add_point(0.0, Color::RED)
            .add_point(0.2, Color::YELLOW)
            .add_point(0.4, Color::GREEN)
            .add_point(0.6, Color::CYAN)
            .add_point(0.8, Color::BLUE)
            .add_point(1.0, Color::FUCHSIA)
    }

    /// Solid White to transparent black gradient
    #[must_use]
    pub fn white_to_none() -> Self {
        Self::empty()
            .add_point(0.0, Color::WHITE)
            .add_point(1.0, Color::NONE)
    }

    /// Smooth White gradient with start and end with full transparency
    #[must_use]
    pub fn smooth_white() -> Self {
        Self::empty()
            .add_point(0.0, Color::rgba(1., 1., 1., 0.))
            .add_point(0.5, Color::WHITE)
            .add_point(1.0, Color::rgba(1., 1., 1., 0.))
    }

    /// Adds a point at `pos` with `color`.
    ///
    /// Note: If a point at `pos` already exists, it will be overwritten
    #[must_use]
    pub fn add_point(mut self, pos: f32, color: Color) -> Self {
        let point = GradientPoint { pos, color };
        if let Some(index) = self
            .points
            .iter()
            .position(|p| (p.pos - pos).abs() < f32::EPSILON)
        {
            self.points[index] = point;
        } else {
            match self.points.iter().position(|x| x.pos >= pos) {
                None => self.points.push(point),
                Some(index) => self.points.insert(index, point),
            }
        }
        self
    }

    /// Evaluates the color gradient at `pos` (will be clamped)
    ///
    /// # Panics
    ///
    /// Will panic if the gradient doesn't have at least 2 points
    #[must_use]
    pub fn evaluate(&self, pos: f32) -> Color {
        assert!(self.points.len() >= 2);

        let len = self.points.len();
        let clamped_pos = pos.clamp(self.points[0].pos, self.points[len - 1].pos);

        // Find the first element in the control point array that has a input
        // value larger than the output value from the source module
        let index2 = self
            .points
            .iter()
            .position(|x| (x.pos > clamped_pos))
            .unwrap_or(len - 1);

        let index1 = match index2.checked_sub(1) {
            None => return self.points[index2].color,
            Some(v) => v,
        };

        // Compute the alpha value used for linear interpolation
        let input0 = self.points[index1].pos;
        let input1 = self.points[index2].pos;
        let delta = (pos - input0) / (input1 - input0);

        Self::sample_color(self.points[index1].color, self.points[index2].color, delta)
    }
}

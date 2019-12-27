use std::ops::{Add, Div, Mul, Sub};

#[derive(Debug, Copy, Clone)]
pub struct Color(f32, f32, f32, f32);

impl Color {
    pub fn rgba(r: f32, g: f32, b: f32, a: f32) -> Self {
        Self(r.min(1.0), g.min(1.0), b.min(1.0), a.min(1.0))
    }

    /// create a solid color from RBG values
    pub fn rgb(r: f32, g: f32, b: f32) -> Self {
        Self::rgba(r, g, b, 1.0)
    }

    /// The red color channel value
    pub fn r(&self) -> f32 {
        self.0
    }

    /// The green color channel value
    pub fn g(&self) -> f32 {
        self.1
    }

    /// The blue color channel value
    pub fn b(&self) -> f32 {
        self.2
    }

    /// The alpha color (transparency) channel value
    pub fn a(&self) -> f32 {
        self.3
    }

    pub fn as_rgb_bytes(&self) -> [u8; 3] {
        [
            (self.r() * 255.0).min(255.0).round() as u8,
            (self.g() * 255.0).min(255.0).round() as u8,
            (self.b() * 255.0).min(255.0).round() as u8,
        ]
    }
}

/// Test the equality of each color element to 4 significant decimal places
const EPSILON: f32 = 0.0001;

impl PartialEq for Color {
    fn eq(&self, other: &Self) -> bool {
        self.r() - other.r() <= EPSILON
            && self.g() - other.g() <= EPSILON
            && self.b() - other.b() <= EPSILON
            && self.a() - other.a() <= EPSILON
    }
}
impl Add for Color {
    type Output = Self;

    fn add(self, rhs: Self) -> Self {
        Color::rgb(self.r() + rhs.r(), self.g() + rhs.g(), self.b() + rhs.b())
    }
}

impl Sub for Color {
    type Output = Self;

    fn sub(self, rhs: Self) -> Self {
        Color::rgb(self.r() - rhs.r(), self.g() - rhs.g(), self.b() - rhs.b())
    }
}

impl Mul for Color {
    type Output = Self;

    fn mul(self, rhs: Self) -> Self {
        Color::rgb(self.r() * rhs.r(), self.g() * rhs.g(), self.b() * rhs.b())
    }
}

impl Mul<f32> for Color {
    type Output = Self;

    fn mul(self, rhs: f32) -> Self {
        Color::rgb(self.r() * rhs, self.g() * rhs, self.b() * rhs)
    }
}

impl Div<f32> for Color {
    type Output = Self;

    fn div(self, rhs: f32) -> Self {
        Color::rgb(self.r() / rhs, self.g() / rhs, self.b() / rhs)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_similar_colors_are_equal() {
        let c1 = Color::rgb(0.3489778009, 1.097864356, 0.03747588);
        let c2 = Color::rgb(0.3489, 1.0978, 0.0374);
        assert_eq!(c1, c2);
    }

    #[test]
    fn test_colors_have_ordered_rgb_components() {
        let color = Color::rgb(0.34, 1.0, 0.0);
        assert!(0.34 - color.r() <= EPSILON);
        assert!(1.0 - color.g() <= EPSILON);
        assert!(0.0 - color.b() <= EPSILON);
    }

    #[test]
    fn test_can_add_two_colors() {
        assert_eq!(
            Color::rgb(0.75, 0.75, 0.75),
            Color::rgb(0.5, 0.5, 0.5) + Color::rgb(0.25, 0.25, 0.25)
        );
    }

    #[test]
    fn test_subtracting_two_colors_equals_a_different_color() {
        let c1 = Color::rgb(0.9, 0.6, 0.75);
        let c2 = Color::rgb(0.7, 0.1, 0.25);
        assert_eq!(Color::rgb(0.2, 0.5, 0.5), c1 - c2);
    }

    #[test]
    fn test_multiplying_two_colors_equals_hadamard_product() {
        assert_eq!(
            Color::rgb(0.1156, 0.56, 0.0),
            Color::rgb(0.34, 0.56, 0.0) * Color::rgb(0.34, 1.0, 0.0)
        );
    }

    #[test]
    fn test_multiplying_a_color_by_a_scalar_equals_a_color() {
        assert_eq!(Color::rgb(0.4, 0.6, 0.8), Color::rgb(0.2, 0.3, 0.4) * 2.0);
    }
}

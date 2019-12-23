#![allow(unused_macros, dead_code)]

use std::ops::{Add, Div, Mul, Neg, Sub};

macro_rules! point {
    ($x:expr, $y:expr, $z:expr) => {
        Tuple::point($x, $y, $z);
    };
}

macro_rules! vector {
    ($x:expr, $y:expr, $z:expr) => {
        Tuple::vector($x, $y, $z);
    };
}

macro_rules! color {
    ($r:expr, $g:expr, $b:expr) => {
        Tuple::color($r, $g, $b);
    };
}

/// Dim is a scalar dimension in space
pub type Dim = f32;

pub trait Vector {
    fn normalize(self) -> Self;
    fn dot(&self, other: Self) -> Dim;
    fn cross(&self, other: Self) -> Self;
    fn magnitude(&self) -> Dim;
}

pub trait Point {}

pub trait Color {
    fn red(&self) -> Dim;
    fn blue(&self) -> Dim;
    fn green(&self) -> Dim;
    fn as_rgb_bytes(&self) -> [u8;3];
}

pub trait Coordinate: Vector + Point {
    fn x(&self) -> Dim;
    fn y(&self) -> Dim;
    fn z(&self) -> Dim;
    fn w(&self) -> Dim;
    fn is_vector(&self) -> bool {
        self.w() == 0.0
    }
    fn is_point(&self) -> bool {
        self.w() == 1.0
    }
}

#[derive(Debug, Clone)]
pub struct Tuple {
    dimensions: Vec<Dim>,
}

impl Tuple {
    fn new(elements: &[Dim]) -> Self {
        Self {
            dimensions: elements.to_vec(),
        }
    }

    pub fn vector(x: Dim, y: Dim, z: Dim) -> Self {
        Self::new(&[x, y, z, 0.0])
    }

    pub fn point(x: Dim, y: Dim, z: Dim) -> Self {
        Self::new(&[x, y, z, 1.0])
    }

    pub fn color(red: Dim, green: Dim, blue: Dim) -> Self {
        Self::new(&[red, green, blue])
    }
}

const EPSILON: Dim = 0.0001;

impl PartialEq for Tuple {
    fn eq(&self, other: &Self) -> bool {
        for (a, b) in other.dimensions.iter().zip(self.dimensions.iter()) {
            if (a - b).abs() > EPSILON {
                return false;
            }
        }
        true
    }
}

impl Coordinate for Tuple {
    fn x(&self) -> Dim {
        self.dimensions[0]
    }

    fn y(&self) -> Dim {
        self.dimensions[1]
    }

    fn z(&self) -> Dim {
        self.dimensions[2]
    }

    fn w(&self) -> Dim {
        self.dimensions[3]
    }
}

impl Point for Tuple {}

impl Vector for Tuple {
    /// Compute the magnitude (scale) of a vector
    fn magnitude(&self) -> Dim {
        let summed_squares: Dim = self.dimensions.iter().map(|d| d * d).sum();
        summed_squares.sqrt()
    }

    /// Transform a non-zero magnitude vector into a unity vector
    fn normalize(self) -> Self {
        let magnitude = self.magnitude();
        Self {
            dimensions: self.dimensions.iter().map(|d| d / magnitude).collect(),
        }
    }

    /// Compute the dot product of a vector
    fn dot(&self, other: Self) -> Dim {
        self.x() * other.x() + self.y() * other.y() + self.z() * other.z() + self.w() * other.w()
    }

    /// Compute the cross product of a vector
    fn cross(&self, other: Self) -> Self {
        let x_x = self.y() * other.z() - self.z() * other.y();
        let x_y = self.z() * other.x() - self.x() * other.z();
        let x_z = self.x() * other.y() - self.y() * other.x();
        Self::vector(x_x, x_y, x_z)
    }
}

impl Color for Tuple {
    fn red(&self) -> Dim {
        self.dimensions[0]
    }

    fn green(&self) -> Dim {
        self.dimensions[1]
    }

    fn blue(&self) -> Dim {
        self.dimensions[2]
    }

    fn as_rgb_bytes(&self) -> [u8;3] {
        [(self.red() * 255.0).round() as u8, (self.green() * 255.0).round() as u8, (self.blue() * 255.0).round() as u8]
    }
}

impl Add for Tuple {
    type Output = Self;

    fn add(self, other: Self) -> Self {
        Self {
            dimensions: self
                .dimensions
                .iter()
                .zip(other.dimensions)
                .map(|(a, b)| a + b)
                .collect(),
        }
    }
}

impl Sub for Tuple {
    type Output = Self;

    fn sub(self, other: Self) -> Self {
        Self {
            dimensions: self
                .dimensions
                .iter()
                .zip(other.dimensions)
                .map(|(a, b)| a - b)
                .collect(),
        }
    }
}

impl Neg for Tuple {
    type Output = Self;

    fn neg(self) -> Self {
        Self {
            dimensions: self.dimensions.iter().map(|x| -x).collect(),
        }
    }
}

impl Mul for Tuple {
    type Output = Self;

    fn mul(self, other: Self) -> Self {
        Self {
            dimensions: self
                .dimensions
                .iter()
                .zip(other.dimensions.iter())
                .map(|(a, b)| a * b)
                .collect(),
        }
    }
}

impl Mul<Dim> for Tuple {
    type Output = Self;

    fn mul(self, rhs: Dim) -> Self {
        Self {
            dimensions: self.dimensions.iter().map(|x| x * rhs).collect(),
        }
    }
}

impl Div<Dim> for Tuple {
    type Output = Self;

    fn div(self, rhs: Dim) -> Self {
        Self {
            dimensions: self.dimensions.iter().map(|x| x / rhs).collect(),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_similar_tuples_are_equal() {
        assert_eq!(
            true,
            vector!(0.0, -0.0, 1.0) == vector!(0.00001, -0.00001, 1.00001)
        );
        assert_eq!(
            false,
            vector!(0.0, -0.0, 1.0) == vector!(0.9999, -0.9999, 1.9999)
        );
    }

    #[test]
    fn test_w_is_0_for_a_vector() {
        assert_eq!(Tuple::vector(0.0, 0.0, 0.0).w(), 0.0);
    }

    #[test]
    fn test_is_vector_is_true_for_vector() {
        let test_vector = Tuple::vector(0.0, 0.0, 0.0);
        assert!(test_vector.is_vector());
    }

    #[test]
    fn test_is_vector_is_false_if_point() {
        let test_point = Tuple::point(0.0, 0.0, 0.0);
        assert!(!test_point.is_vector());
    }

    #[test]
    fn test_vector_macro_creates_a_vector() {
        let test_vector = vector!(0.0, 0.0, 0.0);
        assert!(test_vector.is_vector());
    }

    #[test]
    fn test_w_is_1_for_a_point() {
        assert_eq!(Tuple::point(0.0, 0.0, 0.0).w(), 1.0);
    }

    #[test]
    fn test_is_point_is_true_if_point() {
        let test_point = Tuple::point(0.0, 0.0, 0.0);
        assert!(test_point.is_point());
    }

    #[test]
    fn test_is_point_is_false_if_vector() {
        let test_point = Tuple::vector(0.0, 0.0, 0.0);
        assert!(!test_point.is_point());
    }

    #[test]
    fn test_point_macro_creates_a_point() {
        let test_point = point!(0.0, 0.0, 0.0);
        assert!(test_point.is_point());
    }

    #[test]
    fn test_two_tuples_add_together() {
        assert!(point!(3.0, -2.0, 5.0) + vector!(-2.0, 3.0, 1.0) == point!(1.0, 1.0, 6.0));
    }

    #[test]
    fn test_can_subtract_two_tuples() {
        assert_eq!(
            point!(3.0, 2.0, 1.0) - point!(5.0, 6.0, 7.0),
            vector!(-2.0, -4.0, -6.0)
        );
        assert!(vector!(-2.3, 1.4, -32.6) != point!(3.0, 2.0, 1.0) - point!(5.0, 6.0, 7.0));
    }

    #[test]
    fn test_can_subtract_a_vector_from_a_point() {
        assert_eq!(
            point!(3.0, 2.0, 1.0) - vector!(5.0, 6.0, 7.0),
            point!(-2.0, -4.0, -6.0)
        );
    }

    #[test]
    fn test_can_subtract_two_vectors() {
        assert_eq!(
            vector!(3.0, 2.0, 1.0) - vector!(5.0, 6.0, 7.0),
            vector!(-2.0, -4.0, -6.0)
        );
    }

    #[test]
    fn test_can_negate_a_vector() {
        assert_eq!(
            -Tuple::new(&[1.0, -2.0, 3.0, -4.0]),
            Tuple::new(&[-1.0, 2.0, -3.0, 4.0])
        );
    }

    #[test]
    fn test_can_multiply_a_tuple_by_a_scalar() {
        assert_eq!(
            Tuple::new(&[1.0, -2.0, 3.0, -4.0]) * 3.5,
            Tuple::new(&[3.5, -7.0, 10.5, -14.0])
        );
    }

    #[test]
    fn test_can_multiply_by_a_fraction() {
        assert!(Tuple::new(&[0.5, -1.0, 1.5, -2.0]) == Tuple::new(&[1.0, -2.0, 3.0, -4.0]) * 0.5);
        assert!(Tuple::new(&[-5.0, 0.1, -5.1, 0.2]) != Tuple::new(&[1.0, -2.0, 3.0, -4.0]) * 0.5);
    }

    #[test]
    fn test_can_divide_by_a_scalar() {
        assert!(Tuple::new(&[0.5, -1.0, 1.5, -2.0]) == Tuple::new(&[1.0, -2.0, 3.0, -4.0]) / 2.0);
        assert!(Tuple::new(&[-0.5, 1.0, -1.5, 2.0]) != Tuple::new(&[1.0, -2.0, 3.0, -4.0]) / 2.0);
    }

    #[test]
    fn test_can_compute_magnatude_1_vector() {
        let unity_vectors = [
            vector!(1.0, 0.0, 0.0),
            vector!(0.0, 1.0, 0.0),
            vector!(0.0, 0.0, 1.0),
        ];
        for unity in unity_vectors.iter() {
            assert!(1.0 == unity.magnitude());
            assert!(0.1 != unity.magnitude());
        }
    }

    #[test]
    fn test_can_compute_non_unity_magnitude_vector() {
        assert!((14.0 as Dim).sqrt() == vector!(1.0, 2.0, 3.0).magnitude());
        assert!((14.0 as Dim).sqrt() == vector!(-1.0, -2.0, -3.0).magnitude());
        assert!((41.0 as Dim).sqrt() != vector!(1.0, 2.0, -3.0).magnitude());
    }

    #[test]
    fn test_can_normalize_a_vector() {
        assert!(vector!(1.0, 0.0, 0.0) == vector!(4.0, 0.0, 0.0).normalize());
        assert!(vector!(4.0, 0.0, 0.0) != vector!(4.0, 0.0, 0.0).normalize());
    }

    #[test]
    fn test_can_compute_the_dot_product_of_a_vector() {
        assert!(20.0 == vector!(1.0, 2.0, 3.0).dot(vector!(2.0, 3.0, 4.0)));
        assert!(-5.2 != vector!(1.0, 2.0, 3.0).dot(vector!(2.0, 3.0, 4.0)));
    }

    #[test]
    fn test_can_compute_cross_product_of_a_vector() {
        assert!(vector!(1.0, -2.0, 1.0) == vector!(2.0, 3.0, 4.0).cross(vector!(1.0, 2.0, 3.0)));
        assert!(vector!(-0.1, 0.2, -0.1) != vector!(2.0, 3.0, 4.0).cross(vector!(1.0, 2.0, 3.0)));
    }

    #[test]
    fn test_colors_have_red_green_and_blue_components() {
        let color = Tuple::color(0.43, 0.51, 1.1);
        assert!(0.43 == color.red());
        assert!(0.34 != color.red());
        assert!(0.51 == color.green());
        assert!(0.15 != color.green());
        assert!(1.1 == color.blue());
        assert!(0.0 != color.blue());
    }

    #[test]
    fn test_can_add_colors() {
        assert!(color!(1.6, 0.7, 1.0) == color!(0.9, 0.6, 0.75) + color!(0.7, 0.1, 0.25));
        assert!(color!(0.0, 0.5, 1.0) != color!(0.9, 0.6, 0.75) + color!(0.7, 0.1, 0.25));
    }

    #[test]
    fn test_can_subtract_colors() {
        assert!(color!(0.2, 0.5, 0.5) == color!(0.9, 0.6, 0.75) - color!(0.7, 0.1, 0.25));
        assert!(color!(1.6, 0.7, 1.0) != color!(0.9, 0.6, 0.75) - color!(0.7, 0.1, 0.25));
    }

    #[test]
    fn test_can_multiply_colors_for_hadamard_product() {
        assert!(color!(0.9, 0.2, 0.04) == color!(1.0, 0.2, 0.4) * color!(0.9, 1.0, 0.1));
        assert!(color!(1.9, 1.2, 1.04) != color!(1.0, 0.2, 0.4) * color!(0.9, 1.0, 0.1));
    }

    #[test]
    fn test_can_multiply_colors_by_a_scalar() {
        assert!(color!(0.4, 0.6, 0.8) == color!(0.2, 0.3, 0.4) * 2.0);
        assert!(color!(0.1, 1.5, 0.2) != color!(0.2, 0.3, 0.4) * 2.0);
    }
}

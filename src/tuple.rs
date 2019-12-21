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

/// Dim is a scalar dimension in space
pub type Dim = f32;

pub trait Vector {
    fn normalize(self) -> Self;
    fn dot(&self, other: Self) -> Dim;
    fn cross(&self, other: Self) -> Self;
}

pub trait Point {}

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

#[derive(PartialEq, Debug, Clone)]
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

    /// Calculate the magnitude (scale) of a tuple
    pub fn magnitude(&self) -> Dim {
        let summed_squares: Dim = self.dimensions.iter().map(|d| d * d).sum();
        summed_squares.sqrt()
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
    /// Transform a non-zero magnitude vector into a unity vector
    fn normalize(self) -> Self {
        let magnitude = self.magnitude();
        Self {
            dimensions: self.dimensions.iter().map(|d| d / magnitude).collect(),
        }
    }

    /// Calculate the dot product of a vector
    fn dot(&self, other: Self) -> Dim {
        self.x() * other.x() + self.y() * other.y() + self.z() * other.z() + self.w() * other.w()
    }

    /// Compute cross product of a vector
    fn cross(&self, other: Self) -> Self {
        let x_x = self.y() * other.z() - self.z() * other.y();
        let x_y = self.z() * other.x() - self.x() * other.z();
        let x_z = self.x() * other.y() - self.y() * other.x();
        Self::vector(x_x, x_y, x_z)
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
    fn test_can_subtract_two_points() {
        assert_eq!(
            point!(3.0, 2.0, 1.0) - point!(5.0, 6.0, 7.0),
            vector!(-2.0, -4.0, -6.0)
        );
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
        assert_eq!(
            Tuple::new(&[1.0, -2.0, 3.0, -4.0]) * 0.5,
            Tuple::new(&[0.5, -1.0, 1.5, -2.0])
        );
    }

    #[test]
    fn test_can_divide_by_a_scalar() {
        assert_eq!(
            Tuple::new(&[1.0, -2.0, 3.0, -4.0]) / 2.0,
            Tuple::new(&[0.5, -1.0, 1.5, -2.0])
        );
    }

    #[test]
    fn test_can_compute_magnatude_1_vector() {
        let unity_vectors = [
            vector!(1.0, 0.0, 0.0),
            vector!(0.0, 1.0, 0.0),
            vector!(0.0, 0.0, 1.0),
        ];
        for unity in unity_vectors.iter() {
            assert_eq!(unity.magnitude(), 1.0);
        }
    }

    #[test]
    fn test_can_compute_non_unity_magnitude_vector() {
        assert_eq!(vector!(1.0, 2.0, 3.0).magnitude(), (14.0 as Dim).sqrt());
        assert_eq!(vector!(-1.0, -2.0, -3.0).magnitude(), (14.0 as Dim).sqrt());
    }

    #[test]
    fn test_can_normalize_a_vector() {
        assert_eq!(vector!(4.0, 0.0, 0.0).normalize(), vector!(1.0, 0.0, 0.0));
    }

    #[test]
    fn test_can_compute_the_dot_product_of_a_vector() {
        assert_eq!(vector!(1.0, 2.0, 3.0).dot(vector!(2.0, 3.0, 4.0)), 20.0);
    }

    #[test]
    fn test_can_compute_cross_product_of_a_vector() {
        assert_eq!(
            vector!(1.0, 2.0, 3.0).cross(vector!(2.0, 3.0, 4.0)),
            vector!(-1.0, 2.0, -1.0)
        );
        assert_eq!(
            vector!(2.0, 3.0, 4.0).cross(vector!(1.0, 2.0, 3.0)),
            vector!(1.0, -2.0, 1.0)
        );
    }
}

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

#[derive(PartialEq, Debug)]
pub struct Tuple {
    x: f32,
    y: f32,
    z: f32,
    w: f32,
}

impl Tuple {
    pub fn new(x: f32, y: f32, z: f32, w: f32) -> Self {
        Self { x, y, z, w }
    }

    pub fn vector(x: f32, y: f32, z: f32) -> Self {
        Self::new(x, y, z, 0.0)
    }

    pub fn point(x: f32, y: f32, z: f32) -> Self {
        Self::new(x, y, z, 1.0)
    }

    pub fn x(&self) -> f32 {
        self.x
    }

    pub fn y(&self) -> f32 {
        self.y
    }

    pub fn z(&self) -> f32 {
        self.z
    }

    pub fn w(&self) -> f32 {
        self.w
    }

    pub fn is_vector(&self) -> bool {
        self.w == 0.0
    }

    pub fn is_point(&self) -> bool {
        self.w == 1.0
    }

    /// Calculate the magnitude (scale) of a vector
    pub fn magnitude(&self) -> f32 {
        (self.x() * self.x() + self.y() * self.y() + self.z() * self.z() + self.w() * self.w())
            .sqrt()
    }

    /// Transform a non-zero magnitude vector into a unity vector
    pub fn normalize(self) -> Self {
        let magnitude = self.magnitude();
        Self {
            x: self.x / magnitude,
            y: self.y / magnitude,
            z: self.z / magnitude,
            w: self.w / magnitude,
        }
    }
}

impl Add for Tuple {
    type Output = Self;

    fn add(self, other: Self) -> Self {
        Self {
            x: self.x + other.x,
            y: self.y + other.y,
            z: self.z + other.z,
            w: self.w + other.w,
        }
    }
}

impl Sub for Tuple {
    type Output = Self;

    fn sub(self, other: Self) -> Self {
        Self {
            x: self.x - other.x,
            y: self.y - other.y,
            z: self.z - other.z,
            w: self.w - other.w,
        }
    }
}

impl Neg for Tuple {
    type Output = Self;

    fn neg(self) -> Self {
        Self {
            x: -self.x,
            y: -self.y,
            z: -self.z,
            w: -self.w,
        }
    }
}

impl Mul<f32> for Tuple {
    type Output = Self;

    fn mul(self, rhs: f32) -> Self {
        Self {
            x: self.x * rhs,
            y: self.y * rhs,
            z: self.z * rhs,
            w: self.w * rhs,
        }
    }
}

impl Div<f32> for Tuple {
    type Output = Self;

    fn div(self, rhs: f32) -> Self {
        Self {
            x: self.x / rhs,
            y: self.y / rhs,
            z: self.z / rhs,
            w: self.w / rhs,
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
            -Tuple::new(1.0, -2.0, 3.0, -4.0),
            Tuple::new(-1.0, 2.0, -3.0, 4.0)
        );
    }

    #[test]
    fn test_can_multiply_a_tuple_by_a_scalar() {
        assert_eq!(
            Tuple::new(1.0, -2.0, 3.0, -4.0) * 3.5,
            Tuple::new(3.5, -7.0, 10.5, -14.0)
        );
    }

    #[test]
    fn test_can_multiply_by_a_fraction() {
        assert_eq!(
            Tuple::new(1.0, -2.0, 3.0, -4.0) * 0.5,
            Tuple::new(0.5, -1.0, 1.5, -2.0)
        );
    }

    #[test]
    fn test_can_divide_by_a_scalar() {
        assert_eq!(
            Tuple::new(1.0, -2.0, 3.0, -4.0) / 2.0,
            Tuple::new(0.5, -1.0, 1.5, -2.0)
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
        assert_eq!(vector!(1.0, 2.0, 3.0).magnitude(), 14.0_f32.sqrt());
        assert_eq!(vector!(-1.0, -2.0, -3.0).magnitude(), 14.0_f32.sqrt());
    }

    #[test]
    fn test_can_normalize_a_vector() {
        assert_eq!(vector!(4.0, 0.0, 0.0).normalize(), vector!(1.0, 0.0, 0.0));
    }
}

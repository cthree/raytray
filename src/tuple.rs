#![allow(unused_macros, dead_code)]

use std::ops::{Add, Neg, Sub};

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
}

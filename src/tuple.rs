#![allow(unused_macros, dead_code)]

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
    fn new(x: f32, y: f32, z: f32, w: f32) -> Self {
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
}

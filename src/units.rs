use std::fmt;
use std::ops::{Add, Div, Mul, Neg, Sub};

pub const EPSILON: f32 = 0.00001;

pub type Unit3D = f32;

pub trait Tuple {
    fn x(&self) -> Unit3D;
    fn y(&self) -> Unit3D;
    fn z(&self) -> Unit3D;
    fn w(&self) -> Unit3D;
}

pub trait TupleMut : Tuple {
    fn set_x(&mut self, x: Unit3D);
    fn set_y(&mut self, y: Unit3D);
    fn set_z(&mut self, z: Unit3D);
}

// ==========================================================================
// Vector3D

#[derive(Debug, Clone, Copy, PartialEq, PartialOrd)]
pub struct Vector3D(Unit3D, Unit3D, Unit3D);

impl Vector3D {
    pub fn new(x: Unit3D, y: Unit3D, z: Unit3D) -> Self {
        Self(x, y, z)
    }

    pub fn magnitude(&self) -> Unit3D {
        (self.0 * self.0 + self.1 * self.1 + self.2 * self.2).sqrt()
    }

    pub fn normalize(&self) -> Self {
        let magnitude = self.magnitude();
        Self(
            self.x() / magnitude,
            self.y() / magnitude,
            self.z() / magnitude,
        )
    }

    pub fn dot(&self, other: Self) -> Unit3D {
        self.x() * other.x() + self.y() * other.y() + self.z() * other.z()
    }

    pub fn cross(&self, other: Self) -> Self {
        let cross_x = self.y() * other.z() - self.z() * other.y();
        let cross_y = self.z() * other.x() - self.x() * other.z();
        let cross_z = self.x() * other.y() - self.y() * other.x();
        Self(cross_x, cross_y, cross_z)
    }
}

impl Tuple for Vector3D {
    fn x(&self) -> Unit3D {
        self.0
    }

    fn y(&self) -> Unit3D {
        self.1
    }

    fn z(&self) -> Unit3D {
        self.2
    }

    fn w(&self) -> Unit3D {
        0.0
    }
}

impl TupleMut for Vector3D {
    fn set_x(&mut self, x: Unit3D) {
        self.0 = x;
    }

    fn set_y(&mut self, y: Unit3D) {
        self.1 = y;
    }

    fn set_z(&mut self, z: Unit3D) {
        self.2 = z;
    }
}

impl Add for Vector3D {
    type Output = Self;

    // Add vector to a point to get a new point
    fn add(self, rhs: Self) -> Self {
        Self(self.x() + rhs.x(), self.y() + rhs.y(), self.z() + rhs.z())
    }
}

impl Add<Point3D> for Vector3D {
    type Output = Point3D;

    // Add a point to a vector to get a new point
    fn add(self, rhs: Point3D) -> Point3D {
        Point3D(self.x() + rhs.x(), self.y() + rhs.y(), self.z() + rhs.z())
    }
}

impl Sub for Vector3D {
    type Output = Self;

    // Subtract two vectors to get a new vector
    fn sub(self, rhs: Self) -> Self {
        Self(self.x() - rhs.x(), self.y() - rhs.y(), self.z() - rhs.z())
    }
}

impl Neg for Vector3D {
    type Output = Self;

    // Negatate a vector to get the opposite vector
    fn neg(self) -> Self {
        Self(-self.x(), -self.y(), -self.z())
    }
}

impl Mul<Unit3D> for Vector3D {
    type Output = Self;

    // Muliply a vector by a scalar to get a scaled vector
    fn mul(self, rhs: Unit3D) -> Self {
        Self(self.x() * rhs, self.y() * rhs, self.z() * rhs)
    }
}

impl Div<Unit3D> for Vector3D {
    type Output = Self;

    // Dividing a vector by a scalar to get a smaller vector
    fn div(self, rhs: Unit3D) -> Self {
        Self(self.x() / rhs, self.y() / rhs, self.z() / rhs)
    }
}

impl fmt::Display for Vector3D {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "<{:.4}, {:.4}, {:.4}>", self.x(), self.y(), self.z())
    }
}

// ==========================================================================
// Point3D

#[derive(Debug, Clone, Copy, PartialEq)]
pub struct Point3D(Unit3D, Unit3D, Unit3D);

impl Point3D {
    pub fn new(x: Unit3D, y: Unit3D, z: Unit3D) -> Self {
        Self(x, y, z)
    }
}

impl Tuple for Point3D {
    fn x(&self) -> Unit3D {
        self.0
    }

    fn y(&self) -> Unit3D {
        self.1
    }

    fn z(&self) -> Unit3D {
        self.2
    }

    fn w(&self) -> Unit3D {
        1.0
    }
}

impl TupleMut for Point3D {
    fn set_x(&mut self, x: Unit3D) {
        self.0 = x;
    }

    fn set_y(&mut self, y: Unit3D) {
        self.1 = y;
    }

    fn set_z(&mut self, z: Unit3D) {
        self.2 = z;
    }
}

impl Add<Vector3D> for Point3D {
    type Output = Self;

    // Add vector to a point to get a new point
    fn add(self, rhs: Vector3D) -> Self {
        Self(self.x() + rhs.x(), self.y() + rhs.y(), self.z() + rhs.z())
    }
}

impl Sub for Point3D {
    type Output = Vector3D;

    // Subtract two points to get a new vector
    fn sub(self, rhs: Self) -> Vector3D {
        Vector3D(self.x() - rhs.x(), self.y() - rhs.y(), self.z() - rhs.z())
    }
}

impl Sub<Vector3D> for Point3D {
    type Output = Self;

    // Subtract a vector from a point to get a new point
    fn sub(self, rhs: Vector3D) -> Self {
        Self(self.x() - rhs.x(), self.y() - rhs.y(), self.z() - rhs.z())
    }
}

impl fmt::Display for Point3D {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "[{:.4}, {:.4}, {:.4}]", self.x(), self.y(), self.z())
    }
}

// ==========================================================================
// Unit Tests

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_creates_a_point_of_floats() {
        let point = Point3D(0.0, -2.28, 0.0);
        assert_eq!(0.0, point.x());
        assert_eq!(-2.28, point.y());
    }

    #[test]
    fn test_creates_a_point_of_ints() {
        let point = Point3D(0.0, -2.0, 0.0);
        assert_eq!(0.0, point.x());
        assert_eq!(-2.0, point.y());
    }

    #[test]
    fn test_adding_two_vectors_equals_a_new_vector() {
        let a = Vector3D(1.0, 2.0, 3.0);
        let b = Vector3D(4.0, 5.0, 6.0);
        assert_eq!(Vector3D(5.0, 7.0, 9.0), a.clone() + b.clone());
        assert_eq!(Vector3D(5.0, 7.0, 9.0), b + a);
    }

    #[test]
    fn test_adding_a_vector_to_a_point_equals_a_point() {
        let point_a = Point3D(3.0, -2.0, 5.0);
        let vector_b = Vector3D(-2.0, 3.0, 1.0);
        assert_eq!(Point3D(1.0, 1.0, 6.0), point_a.clone() + vector_b.clone());
        assert_eq!(Point3D(1.0, 1.0, 6.0), vector_b + point_a);
    }

    #[test]
    fn test_subtracting_two_points_equals_a_vector() {
        assert_eq!(
            Vector3D(-2.0, -4.0, -6.0),
            Point3D(3.0, 2.0, 1.0) - Point3D(5.0, 6.0, 7.0)
        );
    }

    #[test]
    fn test_subtracting_a_vector_from_a_point_equals_a_point() {
        assert_eq!(
            Point3D(-2.0, -4.0, -6.0),
            Point3D(3.0, 2.0, 1.0) - Vector3D(5.0, 6.0, 7.0)
        );
    }

    #[test]
    fn test_subtracting_two_vectors_equals_a_vector() {
        assert_eq!(
            Vector3D(-2.0, -4.0, -6.0),
            Vector3D(3.0, 2.0, 1.0) - Vector3D(5.0, 6.0, 7.0)
        );
    }

    #[test]
    fn test_can_negate_a_vector() {
        assert_eq!(
            Vector3D(-1.0, 2.0, -3.0),
            Vector3D(0.0, 0.0, 0.0) - Vector3D(1.0, -2.0, 3.0)
        );
        assert_eq!(Vector3D(-1.0, 2.0, -3.0), -Vector3D(1.0, -2.0, 3.0));
    }

    #[test]
    fn test_multipling_a_vector_by_a_scalar_equals_a_vector() {
        assert_eq!(Vector3D(3.5, -7.0, 10.5), Vector3D(1.0, -2.0, 3.0) * 3.5);
    }

    #[test]
    fn test_dividing_a_vector_by_a_scalar_equals_a_vector() {
        assert_eq!(Vector3D(0.5, -1.0, 1.5), Vector3D(1.0, -2.0, 3.0) / 2.0);
    }

    #[test]
    fn test_computes_the_magnitude_of_a_vector() {
        assert_eq!(1.0, Vector3D(1.0, 0.0, 0.0).magnitude());
        assert_eq!(1.0, Vector3D(0.0, 1.0, 0.0).magnitude());
        assert_eq!(1.0, Vector3D(0.0, 0.0, 1.0).magnitude());
        assert!(14.0_f32.sqrt() - Vector3D(1.0, 2.0, 3.0).magnitude() <= EPSILON);
        assert!(14.0_f32.sqrt() - Vector3D(-1.0, -2.0, -3.0).magnitude() <= EPSILON);
        assert!(14.0_f64.sqrt() as f32 - Vector3D(1.0, 2.0, 3.0).magnitude() <= EPSILON);
    }

    #[test]
    fn test_can_normalize_a_vector() {
        assert_eq!(Vector3D(1.0, 0.0, 0.0), Vector3D(4.0, 0.0, 0.0).normalize());
        let vector = Vector3D(1.0, 2.0, 3.0).normalize();
        assert!(1.0 / 14.0_f32.sqrt() - vector.x() <= EPSILON);
        assert!(2.0 / 14.0_f32.sqrt() - vector.y() <= EPSILON);
        assert!(3.0 / 14.0_f32.sqrt() - vector.z() <= EPSILON);
    }

    #[test]
    fn test_the_magnitude_of_a_normalized_vector_is_1() {
        assert!(Vector3D(1.0, 0.0, 0.0).normalize().magnitude() - 1.0 <= EPSILON);
        assert!(Vector3D(0.0, 1.0, 0.0).normalize().magnitude() - 1.0 <= EPSILON);
        assert!(Vector3D(0.0, 0.0, 1.0).normalize().magnitude() - 1.0 <= EPSILON);
        assert!(Vector3D(0.0, 0.0, 4.0).normalize().magnitude() - 1.0 <= EPSILON);
        assert!(Vector3D(1.0, 2.0, 3.0).normalize().magnitude() - 1.0 <= EPSILON);
        assert!(Vector3D(-1.0, -2.0, -3.0).normalize().magnitude() - 1.0 <= EPSILON);
    }

    #[test]
    fn test_computes_the_dot_product_of_two_vectors() {
        let a = Vector3D(1.0, 2.0, 3.0);
        let b = Vector3D(2.0, 3.0, 4.0);
        assert_eq!(20.0, a.dot(b));
    }

    #[test]
    fn test_computes_the_cross_product_of_two_vectors() {
        let a = Vector3D(1.0, 2.0, 3.0);
        let b = Vector3D(2.0, 3.0, 4.0);
        assert_eq!(Vector3D(-1.0, 2.0, -1.0), a.cross(b))
    }

    #[test]
    fn test_can_display_point() {
        assert_eq!(
            "[1.0000, 2.0000, 3.0000]",
            format!("{}", Point3D(1.0, 2.0, 3.0))
        );
    }

    #[test]
    fn test_can_display_vector() {
        assert_eq!(
            "<1.0000, 2.0000, 3.0000>",
            format!("{}", Vector3D(1.0, 2.0, 3.0))
        );
    }
}

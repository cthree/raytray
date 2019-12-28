use crate::units::{Point3D, Vector3D};
use std::ops::{Index, IndexMut, Mul};

#[derive(Debug, Clone, Copy, Default)]
pub struct Matrix([[f32; 4]; 4]);

pub const IDENTITY: Matrix = Matrix([
    [1.0, 0.0, 0.0, 0.0],
    [0.0, 1.0, 0.0, 0.0],
    [0.0, 0.0, 1.0, 0.0],
    [0.0, 0.0, 0.0, 1.0],
]);

impl Matrix {
    pub fn transpose(self) -> Self {
        let mut transposed: Matrix = Default::default();
        for row in 0..4 {
            for col in 0..4 {
                transposed[row][col] = self[col][row];
            }
        }
        transposed
    }
}
impl PartialEq for Matrix {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}

impl Index<usize> for Matrix {
    type Output = [f32; 4];

    fn index(&self, i: usize) -> &Self::Output {
        &self.0[i]
    }
}

impl IndexMut<usize> for Matrix {
    fn index_mut(&mut self, i: usize) -> &mut Self::Output {
        &mut self.0[i]
    }
}

impl Mul for Matrix {
    type Output = Self;

    fn mul(self, rhs: Self) -> Self {
        let mut product: Matrix = Default::default();
        for row in 0..4 {
            for col in 0..4 {
                product[row][col] = self[row][0] * rhs[0][col]
                    + self[row][1] * rhs[1][col]
                    + self[row][2] * rhs[2][col]
                    + self[row][3] * rhs[3][col];
            }
        }
        product
    }
}

impl Mul<Point3D> for Matrix {
    type Output = Point3D;

    fn mul(self, rhs: Point3D) -> Self::Output {
        let mut product = [0.0; 4];
        for row in 0..4 {
            product[row] = self[row][0] * rhs.x()
                + self[row][1] * rhs.y()
                + self[row][2] * rhs.z()
                + self[row][3] * 1.0;
        }
        Point3D::new(product[0], product[1], product[2])
    }
}

impl Mul<Vector3D> for Matrix {
    type Output = Vector3D;

    fn mul(self, rhs: Vector3D) -> Self::Output {
        let mut product = [0.0; 4];
        for row in 0..4 {
            product[row] = self[row][0] * rhs.x()
                + self[row][1] * rhs.y()
                + self[row][2] * rhs.z()
                + self[row][3] * 0.0;
        }
        Vector3D::new(product[0], product[1], product[2])
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::units::Point3D;

    #[test]
    fn test_can_multiply_two_matrices() {
        let m1 = Matrix([
            [1.0, 2.0, 3.0, 4.0],
            [5.0, 6.0, 7.0, 8.0],
            [9.0, 8.0, 7.0, 6.0],
            [5.0, 4.0, 3.0, 2.0],
        ]);
        let m2 = Matrix([
            [-2.0, 1.0, 2.0, 3.0],
            [3.0, 2.0, 1.0, -1.0],
            [4.0, 3.0, 6.0, 5.0],
            [1.0, 2.0, 7.0, 8.0],
        ]);
        let m3 = Matrix([
            [20.0, 22.0, 50.0, 48.0],
            [44.0, 54.0, 114.0, 108.0],
            [40.0, 58.0, 110.0, 102.0],
            [16.0, 26.0, 46.0, 42.0],
        ]);
        assert_eq!(m3, m1 * m2);
    }

    #[test]
    fn test_multiplying_a_matrix_by_a_point_equals_a_point() {
        let m = Matrix([
            [1.0, 2.0, 3.0, 4.0],
            [2.0, 4.0, 4.0, 2.0],
            [8.0, 6.0, 4.0, 1.0],
            [0.0, 0.0, 0.0, 1.0],
        ]);
        let c = Point3D::new(1.0, 2.0, 3.0);
        assert_eq!(Point3D::new(18.0, 24.0, 33.0), m * c);
    }

    #[test]
    fn test_transposes_a_matrix() {
        let m1 = Matrix([
            [0.0, 9.0, 3.0, 0.0],
            [9.0, 8.0, 0.0, 8.0],
            [1.0, 8.0, 5.0, 3.0],
            [0.0, 0.0, 5.0, 8.0],
        ]);
        let m2 = Matrix([
            [0.0, 9.0, 1.0, 0.0],
            [9.0, 8.0, 8.0, 0.0],
            [3.0, 0.0, 5.0, 5.0],
            [0.0, 8.0, 3.0, 8.0],
        ]);
        assert_eq!(m2, m1.transpose());
    }

    #[test]
    fn test_transposing_the_identity_matrix_does_nothing() {
        assert_eq!(IDENTITY, IDENTITY.transpose());
    }
}

use crate::units::{Point3D, Unit3D, Vector3D, EPSILON};
use std::ops::{Index, IndexMut, Mul, Rem};

#[derive(Debug, Clone, Copy, Default)]
pub struct Matrix([[Unit3D; 4]; 4]);

pub const IDENTITY: Matrix = Matrix([
    [1.0, 0.0, 0.0, 0.0],
    [0.0, 1.0, 0.0, 0.0],
    [0.0, 0.0, 1.0, 0.0],
    [0.0, 0.0, 0.0, 1.0],
]);

impl Matrix {
    /// Transpose the rows and columns of the Matrix such that the element
    /// as `[2,3]` is at `[3,2]` in the resulting Matrix. The original Matrix
    /// is consumed and a new one returned in its place.
    pub fn transpose(self) -> Self {
        let mut transposed: Matrix = Default::default();
        for row in 0..4 {
            for col in 0..4 {
                transposed[row][col] = self[col][row];
            }
        }
        transposed
    }

    /// Calculate and return the determinate value of a Matrix.
    pub fn determinate(&self) -> Unit3D {
        let mut determinate = 0.0;
        for (col, v) in self[0].iter().enumerate() {
            determinate += self.cofactor(0, col) * v;
        }
        determinate
    }

    /// Calculate and return the cofactor of the sub-matrix obtained by
    /// removing row `irow` and column `icol` from the Matrix. Used to
    /// calculate the determinate of the Matrix.
    pub fn cofactor(&self, irow: usize, icol: usize) -> Unit3D {
        let sm = self.submatrix(irow, icol);
        let mut cofactor = 0.0;
        for (col, v) in sm[0].iter().enumerate() {
            cofactor += sm.cofactor(0, col) * v;
        }

        match (irow + icol).rem(2) {
            0 => cofactor,
            _ => -cofactor,
        }
    }

    /// Test the invertability of the Matrix. Returns true if the Matrix is
    /// invertible, false if not.
    pub fn is_invertible(&self) -> bool {
        self.determinate() != 0.0
    }

    /// Calculate and return a new Matrix which is the inverse of the original
    /// such that multiplying a Matrix by it's inverse is the same as dividing
    /// the product by the original.
    pub fn inverse(&self) -> Self {
        if !self.is_invertible() {
            panic!(
                "Cannot compute the inverse of a non-invertible matrix! {:?}",
                self
            );
        }

        // inverse is transposed matrix of cofactors / original determinate
        let determinate = self.determinate();
        let mut inverse: Matrix = Default::default();
        for row in 0..4 {
            for col in 0..4 {
                inverse[col][row] = self.cofactor(row, col) / determinate;
            }
        }
        inverse
    }

    fn submatrix(&self, irow: usize, icol: usize) -> SubMatrix {
        if irow > 3 {
            panic!("submatrix irow out-of-bounds! {} > 3", irow);
        }
        if icol > 3 {
            panic!("submatrix icol out-of-bounds! {} > 3", icol);
        }

        let mut sub_matrix = SubMatrix([[0.0; 3]; 3]);
        let mut sub_row = 0_usize;
        let mut sub_col = 0_usize;

        for row in 0..4 {
            if row == irow {
                continue;
            }
            for col in 0..4 {
                if col == icol {
                    continue;
                }
                sub_matrix[sub_row][sub_col] = self[row][col];
                sub_col += 1;
            }
            sub_row += 1;
            sub_col = 0;
        }
        sub_matrix
    }
}

impl PartialEq for Matrix {
    fn eq(&self, other: &Self) -> bool {
        for row in 0..4 {
            for col in 0..4 {
                if self[row][col] - other[row][col] > EPSILON {
                    return false;
                }
            }
        }
        true
    }
}

impl Index<usize> for Matrix {
    type Output = [Unit3D; 4];

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

#[derive(Debug, PartialEq, Clone, Copy)]
struct SubMatrix([[Unit3D; 3]; 3]);

impl SubMatrix {
    // FIXME: Leaving this commented out in case needed but I suspect
    // it isn't.

    /// Calculate and return the determinate of the SubMatrix
    // pub fn determinate(&self) -> Unit3D {
    //     let mut determinate = 0.0;
    //     for (col, sv) in self[0].iter().enumerate() {
    //         determinate += self.cofactor(0, col) * sv;
    //     }
    //     determinate
    // }

    /// Calculate and return the cofactor of a 2x2 matrix resulting
    /// from removing the row `irow` and the column `icol` from the
    /// orginal SubMatrix.
    pub fn cofactor(&self, irow: usize, icol: usize) -> Unit3D {
        if irow > 2 {
            panic!("submatrix irow out-of-bounds! {} > 3", irow);
        }
        if icol > 2 {
            panic!("submatrix icol out-of-bounds! {} > 3", icol);
        }

        let mut sub_matrix = [[0.0; 2]; 2];
        let mut sub_row = 0_usize;
        let mut sub_col = 0_usize;

        for row in 0..3 {
            if row == irow {
                continue;
            }
            for col in 0..3 {
                if col == icol {
                    continue;
                }
                sub_matrix[sub_row][sub_col] = self[row][col];
                sub_col += 1;
            }
            sub_row += 1;
            sub_col = 0;
        }
        let minor = sub_matrix[0][0] * sub_matrix[1][1] - sub_matrix[0][1] * sub_matrix[1][0];

        match (irow + icol).rem(2) {
            0 => minor,
            _ => -minor,
        }
    }
}

impl Index<usize> for SubMatrix {
    type Output = [Unit3D; 3];

    // Return the row `i` of the SubMatrix
    fn index(&self, i: usize) -> &Self::Output {
        &self.0[i]
    }
}

impl IndexMut<usize> for SubMatrix {
    // Set the value of the row `i` of the SubMatrix
    fn index_mut(&mut self, i: usize) -> &mut Self::Output {
        &mut self.0[i]
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    // Approximate equality, good enough for floating point primative comparisons
    // in tests.
    fn approx_eq(a: Unit3D, b: Unit3D) -> bool {
        a - b <= EPSILON
    }

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

    #[test]
    fn test_compute_a_submatrix_from_a_matrix() {
        let m = Matrix([
            [-6.0, 1.0, 1.0, 6.0],
            [-8.0, 5.0, 8.0, 6.0],
            [-1.0, 0.0, 8.0, 2.0],
            [-7.0, 1.0, -1.0, 1.0],
        ]);
        let sm = SubMatrix([[-6.0, 1.0, 6.0], [-8.0, 8.0, 6.0], [-7.0, -1.0, 1.0]]);
        assert_eq!(sm, m.submatrix(2, 1));
    }

    #[test]
    fn test_computes_the_cofactor_of_a_submatrix() {
        let sm = SubMatrix([[3.0, 5.0, 0.0], [2.0, -1.0, -7.0], [6.0, -1.0, 5.0]]);
        assert_eq!(-12.0, sm.cofactor(0, 0));
    }

    #[test]
    fn test_compute_the_determinant_of_a_submatrix() {
        let sm = SubMatrix([[1.0, 2.0, 6.0], [-5.0, 8.0, -4.0], [2.0, 6.0, 4.0]]);
        assert_eq!(56.0, sm.cofactor(0, 0));
        assert_eq!(12.0, sm.cofactor(0, 1));
        assert_eq!(-46.0, sm.cofactor(0, 2));
        // assert_eq!(-196.0, sm.determinate());
    }

    #[test]
    fn test_compute_the_determinant_of_a_matrix() {
        let m = Matrix([
            [-2.0, -8.0, 3.0, 5.0],
            [-3.0, 1.0, 7.0, 3.0],
            [1.0, 2.0, -9.0, 6.0],
            [-6.0, 7.0, 7.0, -9.0],
        ]);
        assert_eq!(690.0, m.cofactor(0, 0));
        assert_eq!(447.0, m.cofactor(0, 1));
        assert_eq!(210.0, m.cofactor(0, 2));
        assert_eq!(51.0, m.cofactor(0, 3));
        assert_eq!(-4071.0, m.determinate());
    }

    #[test]
    fn test_invertible_matrix_is_invertible() {
        let invertible = Matrix([
            [6.0, 4.0, 4.0, 4.0],
            [5.0, 5.0, 7.0, 6.0],
            [4.0, -9.0, 3.0, -7.0],
            [9.0, 1.0, 7.0, -6.0],
        ]);
        assert!(invertible.is_invertible());
    }

    #[test]
    fn test_non_invertible_maxtrix_is_not_invertible() {
        let non_invertible = Matrix([
            [-4.0, 2.0, -2.0, -3.0],
            [9.0, 6.0, 2.0, 6.0],
            [0.0, -5.0, 1.0, -5.0],
            [0.0, 0.0, 0.0, 0.0],
        ]);
        assert!(!non_invertible.is_invertible());
    }

    #[test]
    #[should_panic]
    fn test_inverting_a_non_invertible_matrix_panics() {
        let non_invertible = Matrix([
            [-4.0, 2.0, -2.0, -3.0],
            [9.0, 6.0, 2.0, 6.0],
            [0.0, -5.0, 1.0, -5.0],
            [0.0, 0.0, 0.0, 0.0],
        ]);
        // Should panic when we try vv
        let _ = non_invertible.inverse();
    }

    #[test]
    fn test_computes_the_inverse_of_an_invertible_matrix() {
        let m1 = Matrix([
            [-5.0, 2.0, 6.0, -8.0],
            [1.0, -5.0, 1.0, 8.0],
            [7.0, 7.0, -6.0, -7.0],
            [1.0, -3.0, 7.0, 4.0],
        ]);
        let m2 = Matrix([
            [0.21805, 0.45113, 0.24060, -0.04511],
            [-0.80827, -1.45677, -0.44361, 0.52068],
            [-0.07895, -0.22368, -0.05263, 0.19737],
            [-0.52256, -0.81391, -0.30075, 0.30639],
        ]);
        assert!(m1.is_invertible());
        assert!(approx_eq(532.0, m1.determinate()));
        assert!(approx_eq(-160.0, m1.cofactor(2, 3)));
        assert!(approx_eq(-160.0 / 532.0, m2[3][2]));
        assert!(approx_eq(105.0, m1.cofactor(3, 2)));
        assert!(approx_eq(105.0 / 532.0, m2[2][3]));
        assert_eq!(m2, m1.inverse());
    }

    #[test]
    fn test_computes_the_inverse_of_another_invertible_matrix() {
        let m1 = Matrix([
            [8.0, -5.0, 9.0, 2.0],
            [7.0, 5.0, 6.0, 1.0],
            [-6.0, 0.0, 9.0, 6.0],
            [-3.0, 0.0, -9.0, -4.0],
        ]);
        let m2 = Matrix([
            [-0.15385, -0.15385, -0.28205, -0.53846],
            [-0.07692, 0.12308, 0.02564, 0.03077],
            [0.35897, 0.35897, -0.43590, 0.92308],
            [-0.69231, -0.69231, -0.76923, -1.92308],
        ]);
        assert_eq!(m2, m1.inverse());
    }

    #[test]
    fn test_computes_the_inverse_of_yet_another_invertible_matrix() {
        let m1 = Matrix([
            [9.0, 3.0, 0.0, 9.0],
            [-5.0, -2.0, -6.0, -3.0],
            [-4.0, 9.0, 6.0, 4.0],
            [-7.0, 6.0, 6.0, 2.0],
        ]);
        let m2 = Matrix([
            [-0.04074, -0.07778, 0.14444, -0.22222],
            [-0.07778, 0.03333, 0.36667, -0.33333],
            [-0.02901, -0.14630, -0.10926, 0.12963],
            [0.17778, 0.06667, -0.26667, 0.33333],
        ]);
        assert_eq!(m2, m1.inverse());
    }

    #[test]
    fn test_multiplying_a_product_by_its_inverse_equals_original_matrix() {
        let m1 = Matrix([
            [3.0, -9.0, 7.0, 3.0],
            [3.0, -8.0, 2.0, -9.0],
            [-4.0, 4.0, 4.0, 1.0],
            [-6.0, 5.0, -1.0, 1.0],
        ]);
        let m2 = Matrix([
            [8.0, 2.0, 2.0, 2.0],
            [3.0, -1.0, 7.0, 0.0],
            [7.0, 0.0, 5.0, 4.0],
            [6.0, -2.0, 0.0, 5.0],
        ]);
        let m3 = m1 * m2;
        assert_eq!(m1, m3 * m2.inverse());
    }
}

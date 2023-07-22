use crate::{
    geometry::{Point, Vector},
    macros::AlmostEq,
};
use std::ops::Mul;

#[derive(Debug, Default, PartialEq, Clone, Copy)]
pub struct Matrix(pub [[f64; 4]; 4]);

impl Matrix {
    pub fn id() -> Self {
        Self([
            [1., 0., 0., 0.],
            [0., 1., 0., 0.],
            [0., 0., 1., 0.],
            [0., 0., 0., 1.],
        ])
    }

    pub fn transpose(self) -> Self {
        let Matrix(m) = self;
        Self([
            [m[0][0], m[1][0], m[2][0], m[3][0]],
            [m[0][1], m[1][1], m[2][1], m[3][1]],
            [m[0][2], m[1][2], m[2][2], m[3][2]],
            [m[0][3], m[1][3], m[2][3], m[3][3]],
        ])
    }
    pub fn inverse(self) -> Self {
        let mut indxc: [usize; 4] = [0; 4];
        let mut indxr: [usize; 4] = [0; 4];
        let mut ipiv: [usize; 4] = [0; 4];
        let Matrix(mut minv) = self.clone();
        for i in 0..4 {
            let mut irow: usize = 0;
            let mut icol: usize = 0;
            let mut big: f64 = 0.;
            for j in 0..4 {
                if ipiv[j] != 1 {
                    for k in 0..4 {
                        if ipiv[k] == 0 {
                            if f64::abs(minv[j][k]) >= big {
                                big = f64::abs(minv[j][k]);
                                irow = j;
                                icol = k;
                            } else if ipiv[k] > 1 {
                                panic!("Singular matrix");
                            }
                        }
                    }
                }
            }
            ipiv[icol] += 1;
            if irow != icol {
                for k in 0..4 {
                    let temp = minv[irow][k];
                    minv[irow][k] = minv[icol][k];
                    minv[icol][k] = temp;
                }
            }
            indxr[i] = irow;
            indxc[i] = icol;
            if minv[icol][icol] == 0. {
                panic!("Singular matrix");
            }
            let pivinv: f64 = 1. / minv[icol][icol];
            minv[icol][icol] = 1.;
            for j in 0..4 {
                minv[icol][j] *= pivinv;
            }

            for j in 0..4 {
                if j != icol {
                    let save: f64 = minv[j][icol];
                    minv[j][icol] = 0.;
                    for k in 0..4 {
                        minv[j][k] -= minv[icol][k] * save;
                    }
                }
            }
        }

        for j in (0..4).rev() {
            if indxr[j] != indxc[j] {
                for k in 0..4 {
                    let temp = minv[k][indxr[j]];
                    minv[k][indxr[j]] = minv[k][indxc[j]];
                    minv[k][indxc[j]] = temp;
                }
            }
        }
        Matrix(minv)
    }
}

impl Mul for Matrix {
    type Output = Matrix;
    fn mul(self, rhs: Self) -> Self::Output {
        let Matrix(m) = self;
        let Matrix(n) = rhs;
        let Matrix(mut r) = Matrix::default();
        for i in 0..4 {
            for j in 0..4 {
                r[i][j] =
                    m[i][0] * n[0][j] + m[i][1] * n[1][j] + m[i][2] * n[2][j] + m[i][3] * n[3][j]
            }
        }
        Matrix(r)
    }
}

impl Mul<Point> for Matrix {
    type Output = Point;

    fn mul(self, rhs: Point) -> Self::Output {
        let Matrix(a) = self;
        Point(
            a[0][0] * rhs.0 + a[0][1] * rhs.1 + a[0][2] * rhs.2 + a[0][3] * 1.,
            a[1][0] * rhs.0 + a[1][1] * rhs.1 + a[1][2] * rhs.2 + a[1][3] * 1.,
            a[2][0] * rhs.0 + a[2][1] * rhs.1 + a[2][2] * rhs.2 + a[2][3] * 1.,
        )
    }
}

impl Mul<Vector> for Matrix {
    type Output = Vector;

    fn mul(self, rhs: Vector) -> Self::Output {
        let Matrix(a) = self;
        Vector(
            a[0][0] * rhs.0 + a[0][1] * rhs.1 + a[0][2] * rhs.2 + a[0][3] * 0.,
            a[1][0] * rhs.0 + a[1][1] * rhs.1 + a[1][2] * rhs.2 + a[1][3] * 0.,
            a[2][0] * rhs.0 + a[2][1] * rhs.1 + a[2][2] * rhs.2 + a[2][3] * 0.,
        )
    }
}

impl AlmostEq for Matrix {
    fn almost_eq(self, other: Self, eps: f64) -> bool {
        let Matrix(a) = self;
        let Matrix(b) = other;
        for i in 0..4 {
            for j in 0..4 {
                if (a[i][j] - b[i][j]).abs() > eps {
                    return false;
                }
            }
        }
        true
    }
}

#[cfg(test)]
mod tests {
    use crate::{assert_almost_eq, geometry::Vector};

    use super::*;
    #[test]
    fn multiplying_two_matrices() {
        let a = Matrix([
            [1., 2., 3., 4.],
            [5., 6., 7., 8.],
            [9., 8., 7., 6.],
            [5., 4., 3., 2.],
        ]);
        let b = Matrix([
            [-2., 1., 2., 3.],
            [3., 2., 1., -1.],
            [4., 3., 6., 5.],
            [1., 2., 7., 8.],
        ]);
        assert_eq!(
            a * b,
            Matrix([
                [20., 22., 50., 48.],
                [44., 54., 114., 108.],
                [40., 58., 110., 102.],
                [16., 26., 46., 42.]
            ])
        );
    }
    #[test]
    fn a_matrix_multipied_by_a_vector() {
        let a = Matrix([
            [1., 2., 3., 4.],
            [5., 6., 7., 8.],
            [9., 8., 7., 6.],
            [5., 4., 3., 2.],
        ]);

        let v = Vector(1., 2., 3.);
        assert_eq!(a * v, Vector(14., 38., 46.))
    }
    #[test]
    fn multiplying_a_matrix_by_the_identity_matrix() {
        let a = Matrix([
            [1., 2., 3., 4.],
            [5., 6., 7., 8.],
            [9., 8., 7., 6.],
            [5., 4., 3., 2.],
        ]);

        assert_eq!(a * Matrix::id(), a);
        assert_eq!(Matrix::id() * a, a);
    }
    #[test]
    fn transposing_a_matrix() {
        let a = Matrix([
            [1., 2., 3., 4.],
            [5., 6., 7., 8.],
            [9., 8., 7., 6.],
            [5., 4., 3., 2.],
        ]);

        let b = Matrix([
            [1., 5., 9., 5.],
            [2., 6., 8., 4.],
            [3., 7., 7., 3.],
            [4., 8., 6., 2.],
        ]);

        assert_eq!(a.transpose(), b)
    }

    #[test]
    fn calculating_the_inverse_of_a_matrix() {
        let a = Matrix([
            [-5., 2., 6., -8.],
            [1., -5., 1., 8.],
            [7., 7., -6., -7.],
            [1., -3., 7., 4.],
        ]);
        let b = Matrix([
            [0.21805, 0.45113, 0.24060, -0.04511],
            [-0.80827, -1.45677, -0.44361, 0.52068],
            [-0.07895, -0.22368, -0.05263, 0.19737],
            [-0.52256, -0.81391, -0.30075, 0.30639],
        ]);
        assert_almost_eq!(a.inverse(), b);
    }
    #[test]
    fn calculating_the_inverse_of_another_matrix() {
        let a = Matrix([
            [21., 14., 8., 3.],
            [27., 19., 11., 4.],
            [19., 14., 9., 4.],
            [4., 3., 2., 1.],
        ]);
        let b = Matrix([
            [1., -1., 1., -3.],
            [-2., 3., -5., 14.],
            [1., -3., 8., -23.],
            [0., 1., -5., 17.],
        ]);
        assert_almost_eq!(a.inverse(), b);
    }

    #[test]
    fn calculating_the_inverse_of_a_thid_matrix() {
        let a = Matrix([
            [8., -5., 9., 2.],
            [7., 5., 6., 1.],
            [-6., 0., 9., 6.],
            [-3., 0., -9., -4.],
        ]);
        let b = Matrix([
            [-0.15385, -0.15385, -0.28205, -0.53846],
            [-0.07692, 0.12308, 0.02564, 0.03077],
            [0.35897, 0.35897, 0.43590, 0.92308],
            [-0.69231, -0.69231, -0.76923, -1.92308],
        ]);
        assert_almost_eq!(a.inverse(), b);
    }
}

use crate::geometry::Tuple;
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

    fn transpose(self) -> Self {
        let Matrix(m) = self;
        Self([
            [m[0][0], m[1][0], m[2][0], m[3][0]],
            [m[0][1], m[1][1], m[2][1], m[3][1]],
            [m[0][2], m[1][2], m[2][2], m[3][2]],
            [m[0][3], m[1][3], m[2][3], m[3][3]],
        ])
    }
    fn inverse(self) -> Self {
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

impl<T> Mul<T> for Matrix
where
    T: Tuple,
{
    type Output = T;

    fn mul(self, rhs: T) -> Self::Output {
        let Matrix(a) = self;
        Self::Output::new(
            a[0][0] * rhs.x() + a[0][1] * rhs.y() + a[0][2] * rhs.z() + a[0][3] * rhs.w(),
            a[1][0] * rhs.x() + a[1][1] * rhs.y() + a[1][2] * rhs.z() + a[1][3] * rhs.w(),
            a[2][0] * rhs.x() + a[2][1] * rhs.y() + a[2][2] * rhs.z() + a[2][3] * rhs.w(),
        )
    }
}

#[cfg(test)]
mod tests {
    use crate::geometry::Vector;

    use super::*;
    fn almost_eq(a: Matrix, b: Matrix) -> bool {
        let Matrix(a) = a;
        let Matrix(b) = b;
        for i in 0..4 {
            for j in 0..4 {
                if (a[i][j] - b[i][j]).abs() > 1e6 {
                    return false;
                }
            }
        }
        true
    }
    #[test]
    fn mul_matrices() {
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
    fn mul_tuple() {
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
    fn mul_id() {
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
    fn test_transpose() {
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
    fn test_inverse() {
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
        assert!(almost_eq(a.inverse(), b));
    }
}

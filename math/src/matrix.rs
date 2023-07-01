use std::ops::{
    Add, AddAssign, Div, DivAssign, Index, IndexMut, Mul, MulAssign, Neg, Sub, SubAssign,
};

pub type Dimensions = (usize, usize);

mod internal {
    pub(super) fn identity<const N: usize>() -> [[f32; N]; N] {
        scalar(1.)
    }

    pub(super) fn scalar<const N: usize>(value: f32) -> [[f32; N]; N] {
        let mut data = [[0.; N]; N];
        for i in 0..data.len() {
            data[i][i] = value;
        }

        data
    }
}

/// Вспомогательный трейт чтобы не забыть чо вообще надо реализовывать
pub trait MatrixMethods
where
    Self: Sized,
{
    /// То что матрица будет возвращать в качестве строки
    type Column;
    /// То что матрица будет возвращать в качестве минора

    fn dim(&self) -> Dimensions;
    fn zero() -> Self;
    fn identity() -> Self;
    fn scalar(value: f32) -> Self;
    fn inverse(&self) -> Self;
    fn adjugate(&self) -> Self;
    fn det(&self) -> f32;
    fn minor(&self, row: usize, column: usize) -> f32;
    fn transpose(&self) -> Self;
    fn row(&self, idx: usize) -> Self::Column;
    fn column(&self, idx: usize) -> Self::Column;
}

#[derive(Debug, Clone, PartialEq, Default)]
pub struct Mat2 {
    data: [[f32; 2]; 2],
}

impl From<[[f32; 2]; 2]> for Mat2 {
    fn from(value: [[f32; 2]; 2]) -> Self {
        Self { data: value }
    }
}

impl MatrixMethods for Mat2 {
    type Column = [f32; 2];

    fn dim(&self) -> Dimensions {
        (2, 2)
    }

    fn zero() -> Self {
        Self::default()
    }

    fn identity() -> Self {
        Self {
            data: internal::identity(),
        }
    }

    fn scalar(value: f32) -> Self {
        Self {
            data: internal::scalar(value),
        }
    }

    fn inverse(&self) -> Self {
        dbg!(self.adjugate().transpose()) / dbg!(self.det())
    }

    fn adjugate(&self) -> Self {
        let mut res = Mat2::zero();
        for i in 0..2 {
            for j in 0..2 {
                res[i][j] = (-1_i32).pow((i + j) as _) as f32 * self.minor(i, j);
            }
        }

        res
    }

    fn minor(&self, row: usize, column: usize) -> f32 {
        let mut res = 0.;
        let mut degree = 1.;
        for i in 0..2 {
            for j in 0..2 {
                if i == row || column == j {
                    continue;
                }

                res += self.data[i][j] * degree;
                degree *= -1.;
            }
        }

        res
    }

    fn det(&self) -> f32 {
        self.data[0][0] * self.data[1][1] - self.data[1][0] * self.data[0][1]
    }

    fn transpose(&self) -> Self {
        let mut data = [[0.; 2]; 2];
        for i in 0..2 {
            data[i] = self.column(i);
        }

        Self::from(data)
    }

    fn row(&self, idx: usize) -> Self::Column {
        dbg!(self.data[idx])
    }

    fn column(&self, idx: usize) -> Self::Column {
        let data = [self.data[0][idx], self.data[1][idx]];

        dbg!(data)
    }
}

// impl<const DIM: usize> Mat<DIM, DIM> {
// TODO: СДЕЛАЙТЕ ПОТОМ ОБЯЗАТЕЛЬНО
// pub fn lu_decomposition(&self) -> (Self, Self) {
//     let mut u = Mat::<DIM, DIM>::zero();
//     let mut l = Mat::<DIM, DIM>::identity();
//     for i in 0..DIM {
//         for j in 0..DIM {
//             let mut sum = self[i][j];
//             for k in 0..min(i, j) {
//                 sum -= l[i][k] * u[k][j]
//             }
//             if i <= j {
//                 u[i][j] = sum
//             } else {
//                 l[i][j] = sum / u[j][j]
//             }
//         }
//     }

//     (l, u)
// }

// pub fn det(&self) -> f32 {
//     if DIM == 2 {
//         self[0][0] * self[1][1] - self[0][1] * self[1][0]
//     } else {
//         let (l, u) = self.lu_decomposition();
//         let (mut det_l, mut det_u) = (1., 1.);

//         for i in 0..DIM {
//             det_l *= l[i][i];
//             det_u *= u[i][i];
//         }

//         det_l * det_u
//     }
// }

//     pub fn adjugate(&self) -> Self {
//     }

//     pub fn invertible(&self) -> Result<Self, String> {
//     }
// }

impl Index<usize> for Mat2 {
    type Output = [f32; 2];
    fn index(&self, index: usize) -> &Self::Output {
        &self.data[index]
    }
}

impl IndexMut<usize> for Mat2 {
    fn index_mut(&mut self, index: usize) -> &mut Self::Output {
        &mut self.data[index]
    }
}

impl Neg for Mat2 {
    type Output = Self;
    fn neg(self) -> Self::Output {
        self * -1.
    }
}

impl Add for Mat2 {
    type Output = Self;
    fn add(mut self, rhs: Self) -> Self::Output {
        for i in 0..2 {
            for j in 0..2 {
                self[i][j] += rhs[i][j]
            }
        }
        self
    }
}

impl AddAssign for Mat2 {
    fn add_assign(&mut self, rhs: Self) {
        *self = self.clone() + rhs;
    }
}

impl Sub for Mat2 {
    type Output = Self;
    fn sub(self, rhs: Self) -> Self::Output {
        self + (-rhs)
    }
}

impl SubAssign for Mat2 {
    fn sub_assign(&mut self, rhs: Self) {
        *self = self.clone() - rhs
    }
}

impl Mul for Mat2 {
    type Output = Mat2;
    fn mul(self, rhs: Mat2) -> Self::Output {
        let mut result = Mat2::zero();
        for i in 0..2 {
            for j in 0..2 {
                for k in 0..2 {
                    result[i][j] += self[i][k] * rhs[k][j];
                }
            }
        }
        result
    }
}

impl Mul<f32> for Mat2 {
    type Output = Self;
    fn mul(mut self, rhs: f32) -> Self::Output {
        for i in 0..2 {
            for j in 0..2 {
                self[i][j] *= rhs
            }
        }
        self
    }
}

impl MulAssign<f32> for Mat2 {
    fn mul_assign(&mut self, rhs: f32) {
        *self = self.clone() * rhs
    }
}

impl Div<f32> for Mat2 {
    type Output = Self;
    fn div(self, rhs: f32) -> Self::Output {
        self * (1. / rhs)
    }
}

impl DivAssign<f32> for Mat2 {
    fn div_assign(&mut self, rhs: f32) {
        *self = self.clone() / rhs
    }
}

#[derive(Debug, Clone, PartialEq, Default)]
pub struct Mat3 {
    data: [[f32; 3]; 3],
}

impl From<[[f32; 3]; 3]> for Mat3 {
    fn from(value: [[f32; 3]; 3]) -> Self {
        Self { data: value }
    }
}

impl MatrixMethods for Mat3 {
    type Column = [f32; 3];

    fn dim(&self) -> Dimensions {
        (3, 3)
    }

    fn zero() -> Self {
        Self::default()
    }

    fn identity() -> Self {
        Self {
            data: internal::identity(),
        }
    }

    fn scalar(value: f32) -> Self {
        Self {
            data: internal::scalar(value),
        }
    }

    fn inverse(&self) -> Self {
        dbg!(self.adjugate().transpose()) / dbg!(self.det())
    }

    fn adjugate(&self) -> Self {
        let mut res = Self::zero();
        let (n, _) = self.dim();
        for i in 0..n {
            for j in 0..n {
                res[i][j] = (-1_i32).pow((i + j) as _) as f32 * self.minor(i, j);
            }
        }

        res
    }

    fn minor(&self, row: usize, column: usize) -> f32 {
        let mut res = Mat2::zero();
        let mut skip_row = false;
        let mut skip_col;
        let mut degree = 1.;
        let n = self.data.len();
        for i in 0..n {
            skip_col = false;
            for j in 0..n {
                if i == row {
                    skip_row = true;
                    continue;
                }

                if column == j {
                    skip_col = true;
                    continue;
                }

                let idx_row = if skip_row { i + 1 } else { i };
                let idx_col = if skip_col { j + 1 } else { j };

                res[idx_row][idx_col] += self.data[i][j] * degree;
                degree *= -1.;
            }
        }

        res.det()
    }

    fn det(&self) -> f32 {
        let (mut res, mut degree) = (0., 1.);
        for i in 0..self.data.len() {
            res += degree * self.minor(i, i);
            degree *= -1.;
        }

        res
    }

    fn transpose(&self) -> Self {
        let n = self.data.len();
        let mut data = [[0.; 3]; 3];
        for i in 0..n {
            data[i] = self.column(i);
        }

        Self::from(data)
    }

    fn row(&self, idx: usize) -> Self::Column {
        self.data[idx]
    }

    fn column(&self, idx: usize) -> Self::Column {
        let data = [self.data[0][idx], self.data[1][idx], self.data[2][idx]];

        data
    }
}

impl Index<usize> for Mat3 {
    type Output = [f32; 3];
    fn index(&self, index: usize) -> &Self::Output {
        &self.data[index]
    }
}

impl IndexMut<usize> for Mat3 {
    fn index_mut(&mut self, index: usize) -> &mut Self::Output {
        &mut self.data[index]
    }
}

impl Neg for Mat3 {
    type Output = Self;
    fn neg(self) -> Self::Output {
        self * -1.
    }
}

impl Add for Mat3 {
    type Output = Self;
    fn add(mut self, rhs: Self) -> Self::Output {
        let (n, _) = self.dim();

        for i in 0..n {
            for j in 0..n {
                self[i][j] += rhs[i][j]
            }
        }
        self
    }
}

impl AddAssign for Mat3 {
    fn add_assign(&mut self, rhs: Self) {
        *self = self.clone() + rhs;
    }
}

impl Sub for Mat3 {
    type Output = Self;
    fn sub(self, rhs: Self) -> Self::Output {
        self + (-rhs)
    }
}

impl SubAssign for Mat3 {
    fn sub_assign(&mut self, rhs: Self) {
        *self = self.clone() - rhs
    }
}

impl Mul for Mat3 {
    type Output = Self;
    fn mul(self, rhs: Self) -> Self::Output {
        let mut result = Self::zero();
        let (n, _) = self.dim();
        for i in 0..n {
            for j in 0..n {
                for k in 0..n {
                    result[i][j] += self[i][k] * rhs[k][j];
                }
            }
        }
        result
    }
}

impl Mul<f32> for Mat3 {
    type Output = Self;
    fn mul(mut self, rhs: f32) -> Self::Output {
        let (n, _) = self.dim();

        for i in 0..n {
            for j in 0..n {
                self[i][j] *= rhs
            }
        }
        self
    }
}

impl MulAssign<f32> for Mat3 {
    fn mul_assign(&mut self, rhs: f32) {
        *self = self.clone() * rhs
    }
}

impl Div<f32> for Mat3 {
    type Output = Self;
    fn div(self, rhs: f32) -> Self::Output {
        self * (1. / rhs)
    }
}

impl DivAssign<f32> for Mat3 {
    fn div_assign(&mut self, rhs: f32) {
        *self = self.clone() / rhs
    }
}

#[derive(Debug, Clone, PartialEq, Default)]
pub struct Mat4 {
    data: [[f32; 4]; 4],
}

impl From<[[f32; 4]; 4]> for Mat4 {
    fn from(value: [[f32; 4]; 4]) -> Self {
        Self { data: value }
    }
}

impl MatrixMethods for Mat4 {
    type Column = [f32; 4];

    fn dim(&self) -> Dimensions {
        (4, 4)
    }

    fn zero() -> Self {
        Self::default()
    }

    fn identity() -> Self {
        Self {
            data: internal::identity(),
        }
    }

    fn scalar(value: f32) -> Self {
        Self {
            data: internal::scalar(value),
        }
    }

    fn inverse(&self) -> Self {
        dbg!(self.adjugate().transpose()) / dbg!(self.det())
    }

    fn adjugate(&self) -> Self {
        let mut res = Self::zero();
        let (n, _) = self.dim();
        for i in 0..n {
            for j in 0..n {
                res[i][j] = (-1_i32).pow((i + j) as _) as f32 * self.minor(i, j);
            }
        }

        res
    }

    fn minor(&self, row: usize, column: usize) -> f32 {
        let mut res = Mat3::zero();
        let mut skip_row = false;
        let mut skip_col;
        let mut degree = 1.;
        let n = self.data.len();
        for i in 0..n {
            skip_col = false;
            for j in 0..n {
                if i == row {
                    skip_row = true;
                    continue;
                }

                if column == j {
                    skip_col = true;
                    continue;
                }

                let idx_row = if skip_row { i + 1 } else { i };
                let idx_col = if skip_col { j + 1 } else { j };

                res[idx_row][idx_col] += self.data[i][j] * degree;
                degree *= -1.;
            }
        }

        res.det()
    }

    fn det(&self) -> f32 {
        let (mut res, mut degree) = (0., 1.);
        for i in 0..self.data.len() {
            res += degree * self.minor(i, i);
            degree *= -1.;
        }

        res
    }

    fn transpose(&self) -> Self {
        let n = self.data.len();
        let mut data = [[0.; 4]; 4];
        for i in 0..n {
            data[i] = self.column(i);
        }

        Self::from(data)
    }

    fn row(&self, idx: usize) -> Self::Column {
        self.data[idx]
    }

    fn column(&self, idx: usize) -> Self::Column {
        let data = [self.data[0][idx], self.data[1][idx], self.data[2][idx], self.data[3][idx]];

        data
    }
}

impl Index<usize> for Mat4 {
    type Output = [f32; 4];
    fn index(&self, index: usize) -> &Self::Output {
        &self.data[index]
    }
}

impl IndexMut<usize> for Mat4 {
    fn index_mut(&mut self, index: usize) -> &mut Self::Output {
        &mut self.data[index]
    }
}

impl Neg for Mat4 {
    type Output = Self;
    fn neg(self) -> Self::Output {
        self * -1.
    }
}

impl Add for Mat4 {
    type Output = Self;
    fn add(mut self, rhs: Self) -> Self::Output {
        let (n, _) = self.dim();

        for i in 0..n {
            for j in 0..n {
                self[i][j] += rhs[i][j]
            }
        }
        self
    }
}

impl AddAssign for Mat4 {
    fn add_assign(&mut self, rhs: Self) {
        *self = self.clone() + rhs;
    }
}

impl Sub for Mat4 {
    type Output = Self;
    fn sub(self, rhs: Self) -> Self::Output {
        self + (-rhs)
    }
}

impl SubAssign for Mat4 {
    fn sub_assign(&mut self, rhs: Self) {
        *self = self.clone() - rhs
    }
}

impl Mul for Mat4 {
    type Output = Self;
    fn mul(self, rhs: Self) -> Self::Output {
        let mut result = Self::zero();
        let (n, _) = self.dim();
        for i in 0..n {
            for j in 0..n {
                for k in 0..n {
                    result[i][j] += self[i][k] * rhs[k][j];
                }
            }
        }
        result
    }
}

impl Mul<f32> for Mat4 {
    type Output = Self;
    fn mul(mut self, rhs: f32) -> Self::Output {
        let (n, _) = self.dim();

        for i in 0..n {
            for j in 0..n {
                self[i][j] *= rhs
            }
        }
        self
    }
}

impl MulAssign<f32> for Mat4 {
    fn mul_assign(&mut self, rhs: f32) {
        *self = self.clone() * rhs
    }
}

impl Div<f32> for Mat4 {
    type Output = Self;
    fn div(self, rhs: f32) -> Self::Output {
        self * (1. / rhs)
    }
}

impl DivAssign<f32> for Mat4 {
    fn div_assign(&mut self, rhs: f32) {
        *self = self.clone() / rhs
    }
}


#[test]
fn test() {
    let m: Mat2 = Mat2::from([[1., -2.], [4., 0.]]);

    println!("{m:?}");
    println!("{det}", det = m.det());
    let n = m.transpose();
    println!("{n:?}");

    let inv = m.inverse();
    println!("{inv:?} {res:?}", res = inv.clone() * m);
}

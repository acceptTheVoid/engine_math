use std::{
    cmp::min,
    ops::{Add, AddAssign, Div, DivAssign, Index, IndexMut, Mul, MulAssign, Neg, Sub, SubAssign},
};

#[derive(Debug, Clone, PartialEq)]
pub struct Mat<const ROWS: usize, const COLUMNS: usize> {
    data: [[f32; COLUMNS]; ROWS],
}

impl<const ROWS: usize, const COLUMNS: usize> Mat<ROWS, COLUMNS> {
    pub fn zero() -> Self {
        Self {
            data: [[0.; COLUMNS]; ROWS],
        }
    }
}

impl<const DIM: usize> Mat<DIM, DIM> {
    pub fn identity() -> Self {
        Self::scalar(1.)
    }

    pub fn scalar(value: f32) -> Self {
        let mut data = [[0.; DIM]; DIM];
        for i in 0..DIM {
            data[i][i] = value;
        }
        Self { data }
    }
}

impl<const ROWS: usize, const COLUMNS: usize> From<[[f32; COLUMNS]; ROWS]> for Mat<ROWS, COLUMNS> {
    fn from(data: [[f32; COLUMNS]; ROWS]) -> Self {
        Self { data }
    }
}

impl<const ROWS: usize, const COLUMNS: usize> Mat<ROWS, COLUMNS> {
    pub const fn dim(&self) -> (usize, usize) {
        (ROWS, COLUMNS)
    }

    pub fn row(&self, idx: usize) -> [f32; COLUMNS] {
        self[idx].clone()
    }

    pub fn column(&self, idx: usize) -> [f32; ROWS] {
        let mut column = [0.; ROWS];
        for i in 0..ROWS {
            column[i] = self[i][idx]
        }
        column
    }

    pub fn transpose(&self) -> Mat<COLUMNS, ROWS> {
        let mut data = [[0.; ROWS]; COLUMNS];
        for i in 0..COLUMNS {
            data[i] = self.column(i);
        }
        
        Mat::from(data)
    }
}

impl<const DIM: usize> Mat<DIM, DIM> {
    pub fn lu_decomposition(&self) -> (Self, Self) {
        let mut u = Mat::<DIM, DIM>::zero();
        let mut l = Mat::<DIM, DIM>::identity();
        for i in 0..DIM {
            for j in 0..DIM {
                let mut sum = self[i][j];
                for k in 0..min(i, j) {
                    sum -= l[i][k] * u[k][j]
                }
                if i <= j {
                    u[i][j] = sum
                } else {
                    l[i][j] = sum / u[j][j]
                }
            }
        }

        (l, u)
    }

    pub fn det(&self) -> f32 {
        if DIM == 2 {
            self[0][0] * self[1][1] - self[0][1] * self[1][0]
        } else {
            let (l, u) = self.lu_decomposition();
            let (mut det_l, mut det_u) = (1., 1.);
            
            for i in 0..DIM {
                det_l *= l[i][i];
                det_u *= u[i][i];
            }

            det_l * det_u
        }
    }

    fn minor(&self, i: usize, j: usize) -> f32 {
        // let mut data = [[0.; DIM - 1]; DIM - 1];
        todo!()
    }
    
    pub fn adjugate(&self) -> Self {

        todo!()
    }

    pub fn invertible(&self) -> Result<Self, String> {

        todo!()
    }

}

impl<const ROWS: usize, const COLUMNS: usize> Index<usize> for Mat<ROWS, COLUMNS> {
    type Output = [f32; COLUMNS];
    fn index(&self, index: usize) -> &Self::Output {
        &self.data[index]
    }
}

impl<const ROWS: usize, const COLUMNS: usize> IndexMut<usize> for Mat<ROWS, COLUMNS> {
    fn index_mut(&mut self, index: usize) -> &mut Self::Output {
        &mut self.data[index]
    }
}

impl<const ROWS: usize, const COLUMNS: usize> Neg for Mat<ROWS, COLUMNS> {
    type Output = Self;
    fn neg(self) -> Self::Output {
        self * -1.
    }
}

impl<const ROWS: usize, const COLUMNS: usize> Add for Mat<ROWS, COLUMNS> {
    type Output = Self;
    fn add(mut self, rhs: Self) -> Self::Output {
        for i in 0..ROWS {
            for j in 0..COLUMNS {
                self[i][j] += rhs[i][j]
            }
        }
        self
    }
}

impl<const ROWS: usize, const COLUMNS: usize> AddAssign for Mat<ROWS, COLUMNS> {
    fn add_assign(&mut self, rhs: Self) {
        *self = self.clone() + rhs;
    }
}

impl<const ROWS: usize, const COLUMNS: usize> Sub for Mat<ROWS, COLUMNS> {
    type Output = Self;
    fn sub(self, rhs: Self) -> Self::Output {
        self + (-rhs)
    }
}

impl<const ROWS: usize, const COLUMNS: usize> SubAssign for Mat<ROWS, COLUMNS> {
    fn sub_assign(&mut self, rhs: Self) {
        *self = self.clone() - rhs
    }
}

impl<const ROWS_L: usize, const COLUMNS_R: usize, const COMMON: usize> Mul<Mat<COMMON, COLUMNS_R>>
    for Mat<ROWS_L, COMMON>
{
    type Output = Mat<ROWS_L, COLUMNS_R>;
    fn mul(self, rhs: Mat<COMMON, COLUMNS_R>) -> Self::Output {
        let mut result = Mat::zero();
        for i in 0..ROWS_L {
            for j in 0..COLUMNS_R {
                for k in 0..COMMON {
                    result[i][j] += self[i][k] * rhs[k][j];
                }
            }
        }
        result
    }
}

impl<const ROWS: usize, const COLUMNS: usize> Mul<f32> for Mat<ROWS, COLUMNS> {
    type Output = Self;
    fn mul(mut self, rhs: f32) -> Self::Output {
        for i in 0..3 {
            for j in 0..3 {
                self[i][j] *= rhs
            }
        }
        self
    }
}

impl<const ROWS: usize, const COLUMNS: usize> MulAssign<f32> for Mat<ROWS, COLUMNS> {
    fn mul_assign(&mut self, rhs: f32) {
        *self = self.clone() * rhs
    }
}

impl<const ROWS: usize, const COLUMNS: usize> Div<f32> for Mat<ROWS, COLUMNS> {
    type Output = Self;
    fn div(self, rhs: f32) -> Self::Output {
        self * (1. / rhs)
    }
}

impl<const ROWS: usize, const COLUMNS: usize> DivAssign<f32> for Mat<ROWS, COLUMNS> {
    fn div_assign(&mut self, rhs: f32) {
        *self = self.clone() / rhs
    }
}

#[test]
fn test() {
    let m: Mat<3, 3> = Mat::from([
        [1., -2., 3.],
        [4., 0., 6.],
        [-7., 8., 9.],
    ]);

    println!("{m:?}");
    println!("{det}", det=m.det());
    let n = m.transpose();
    println!("{n:?}")
}

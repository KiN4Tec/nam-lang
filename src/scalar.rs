use crate::matrix::Matrix;

use std::ops::{Add, Div, Mul, Neg, Sub};

pub type Scalar = f64;

impl Add<Matrix> for Scalar {
	type Output = Matrix;
	fn add(self, rhs: Matrix) -> Self::Output {
		rhs.add_scalar(self)
	}
}

impl Sub<Matrix> for Scalar {
	type Output = Matrix;
	fn sub(self, rhs: Matrix) -> Self::Output {
		rhs.neg().add_scalar(self)
	}
}

impl Mul<Matrix> for Scalar {
	type Output = Matrix;
	fn mul(self, rhs: Matrix) -> Self::Output {
		rhs.mul_scalar(self)
	}
}

impl Div<Matrix> for Scalar {
	type Output = Matrix;
	fn div(self, rhs: Matrix) -> Self::Output {
		let mut res = rhs.clone();
		for cell in res.iter_mut() {
			*cell = self / *cell;
		}
		res
	}
}

use crate::errors::EvaluationError;
use crate::scalar::Scalar;

use std::{
	fmt::Display,
	ops::{Add, Div, Index, IndexMut, Mul, Neg, Sub},
	slice::{Iter, IterMut},
};

// TODO: Maybe use SIMD

#[derive(Debug, Clone)]
pub struct Matrix {
	data: Vec<Scalar>,
	shape: (usize, usize), // (rows/height, columns/width)
}

//////////////////////////
////// Constructors //////
//////////////////////////

impl Default for Matrix {
	fn default() -> Self {
		Self::new()
	}
}

impl Matrix {
	pub fn new() -> Self {
		Self {
			data: Vec::new(),
			shape: (0, 0),
		}
	}

	pub fn try_from_rows(data: Vec<Vec<Scalar>>) -> Result<Self, EvaluationError> {
		let height = data.len();
		let width = match data.first() {
			Some(row) => row.len(),
			None => {
				return Ok(Self {
					data: Vec::new(),
					shape: (0, 0),
				});
			},
		};

		for row in data.iter() {
			if row.len() != width {
				return Err(EvaluationError::InconsistantMatrixWidth(width, row.len()));
			}
		}

		Ok(Self {
			data: data.into_iter().flatten().collect(),
			shape: (height, width),
		})
	}

	/// Creates an identitiy matrix where the width and height are the same
	///
	/// # Parameters
	/// - size: the width or height of the matrix
	///
	/// See also `Matrix::identity_rect()`
	///
	pub fn identity_square(size: usize) -> Self {
		let mut data = vec![0.0; size * size];

		for i in 0..size {
			data[i * size + i] = 1.0;
		}

		Self {
			data,
			shape: (size, size),
		}
	}

	pub fn identity_rect(nrows: usize, ncols: usize) -> Self {
		let mut data = vec![0.0; nrows * ncols];

		for i in 0..nrows {
			data[i * ncols + i] = 1.0;
		}

		Self {
			data,
			shape: (nrows, ncols),
		}
	}

	/// Creates a matrix of zeros where the width and height are the same
	///
	/// # Parameters
	/// - size: the width or height of the matrix
	///
	/// See also `Matrix::zeros_rect()`
	///
	pub fn zeros_square(size: usize) -> Self {
		Self {
			data: vec![0.0; size * size],
			shape: (size, size),
		}
	}

	pub fn zeros_rect(nrows: usize, ncolumns: usize) -> Self {
		Self {
			data: vec![0.0; nrows * ncolumns],
			shape: (nrows, ncolumns),
		}
	}

	/// Creates a matrix of ones where the width and height are the same
	///
	/// # Parameters
	/// - size: the width or height of the matrix
	///
	/// See also `Matrix::ones_rect()`
	///
	pub fn ones_square(size: usize) -> Self {
		Self {
			data: vec![1.0; size * size],
			shape: (size, size),
		}
	}

	pub fn ones_rect(nrows: usize, ncolumns: usize) -> Self {
		Self {
			data: vec![1.0; nrows * ncolumns],
			shape: (nrows, ncolumns),
		}
	}

	pub fn from_permutations_vector(input: Vec<usize>) -> Self {
		let mut output = Self::zeros_square(input.len());
		for (col, row) in input.into_iter().enumerate() {
			output[(row, col)] = 1.0;
		}
		output
	}
}

////////////////////////////
////// Math Operation //////
////////////////////////////

////// Basic

impl Add for Matrix {
	type Output = Self;
	fn add(self, rhs: Self) -> Self::Output {
		assert_eq!(
			self.shape, rhs.shape,
			"LHS and RHS matrices shapes do not match"
		);

		let mut res = self.data.clone();
		for (idx, cell) in rhs.data.iter().enumerate() {
			res[idx] += cell;
		}

		Self {
			data: res,
			shape: self.shape,
		}
	}
}

impl Sub for Matrix {
	type Output = Self;
	fn sub(self, rhs: Self) -> Self::Output {
		assert_eq!(
			self.shape, rhs.shape,
			"LHS and RHS matrices shapes do not match"
		);

		let mut res = self.data.clone();
		for (idx, cell) in rhs.data.iter().enumerate() {
			res[idx] -= cell;
		}

		Self {
			data: res,
			shape: self.shape,
		}
	}
}

impl Mul for Matrix {
	type Output = Self;
	fn mul(self, rhs: Self) -> Self::Output {
		assert_eq!(
			self.width(),
			rhs.height(),
			"LHS matrix width must match RHS matrix height"
		);

		let mut res = vec![vec![0.0; rhs.width()]; self.height()];
		for (row_idx, row) in res.iter_mut().enumerate() {
			for (col_idx, col) in row.iter_mut().enumerate() {
				for idx in 0..self.width() {
					*col += self[(row_idx, idx)] * rhs[(idx, col_idx)];
				}
			}
		}

		Self::try_from_rows(res).unwrap()
	}
}

impl Div for Matrix {
	type Output = Self;

	#[allow(
		clippy::suspicious_arithmetic_impl,
		reason = "This is needed because clippy is suspecting the usage of the multiplication operator (*)
	    inside the division trait. And this is because there is no direct definition of matrix
	    division in mathematics, other than multiplying by the inverse of the right hand side."
	)]
	fn div(self, rhs: Self) -> Self::Output {
		self * rhs.try_invert().unwrap()
	}
}

impl Neg for Matrix {
	type Output = Self;
	fn neg(self) -> Self::Output {
		Self {
			data: self.data.iter().map(|&cell| -cell).collect(),
			shape: self.shape,
		}
	}
}

////// Scalar

impl Matrix {
	pub fn add_scalar(&self, scalar: Scalar) -> Self {
		let mut res = self.clone();
		for cell in res.data.iter_mut() {
			*cell += scalar;
		}
		res
	}

	pub fn sub_scalar(&self, scalar: Scalar) -> Self {
		let mut res = self.clone();
		for cell in res.data.iter_mut() {
			*cell -= scalar;
		}
		res
	}

	pub fn mul_scalar(&self, scalar: Scalar) -> Self {
		let mut res = self.clone();
		for cell in res.data.iter_mut() {
			*cell *= scalar;
		}
		res
	}

	pub fn div_scalar(&self, scalar: Scalar) -> Self {
		let mut res = self.clone();
		for cell in res.data.iter_mut() {
			*cell /= scalar;
		}
		res
	}
}

impl Add<Scalar> for Matrix {
	type Output = Self;
	fn add(self, rhs: Scalar) -> Self::Output {
		self.add_scalar(rhs)
	}
}

impl Sub<Scalar> for Matrix {
	type Output = Self;
	fn sub(self, rhs: Scalar) -> Self::Output {
		self.sub_scalar(rhs)
	}
}

impl Mul<Scalar> for Matrix {
	type Output = Self;
	fn mul(self, rhs: Scalar) -> Self::Output {
		self.mul_scalar(rhs)
	}
}

impl Div<Scalar> for Matrix {
	type Output = Self;
	fn div(self, rhs: Scalar) -> Self::Output {
		self.div_scalar(rhs)
	}
}

////// Other

impl Matrix {
	/// # Returns
	/// A tuple of (L, U, P, R) where:
	/// - L is the Lower triangular matrix
	/// - U is the Upper triangular matrix
	/// - P is the Permutations vector which can be converted into a permutation matrix
	/// - R is the rank of the matrix
	///
	/// such that PA = LU where A is the input matrix
	///
	/// See `Matrix::from_permutations_vector`
	pub fn lu_decomp(&self) -> (Self, Self, Vec<usize>, usize) {
		let mut lower = Self::zeros_rect(self.nrows(), self.ncols());
		let mut upper = self.clone();
		let mut permutations: Vec<usize> = (0..self.height()).collect();

		let mut shift = 0;
		let mut pivot = 0;
		while pivot < upper.height() && pivot + shift < upper.width() {
			let pivot_row = pivot;
			let pivot_col = pivot + shift;

			// Replace with a row that has a non-zero cell in the current column if needed (and available)
			if upper[(pivot_row, pivot_col)] == 0.0 {
				for row in pivot_row..upper.height() {
					if upper[(row, pivot_col)] == 0.0 {
						continue;
					}
					upper.swap_rows_starting_from(pivot_row, row, pivot_col);
					lower.swap_rows_ending_at(pivot_row, row, pivot_col - 1);
					permutations.swap(pivot_row, row);
					break;
				}
			}

			// Skip column if no non-zero cell is found
			if upper[(pivot_row, pivot_col)] == 0.0 {
				shift += 1;
				continue;
			}

			// Do the subtraction
			for row in (pivot_row + 1)..upper.height() {
				if upper[(row, pivot_col)] == 0.0 {
					continue;
				}

				let factor = upper[(row, pivot_col)] / upper[(pivot_row, pivot_col)];
				for col in 0..upper.width() {
					upper[(row, col)] -= upper[(pivot_row, col)] * factor;
				}

				lower[(row, pivot_col)] = factor;
			}

			pivot += 1;
		}

		for i in 0..lower.height() {
			lower[(i, i)] = 1.0;
		}

		let rank = (upper.width() - shift).min(upper.height());

		(lower, upper, permutations, rank)
	}

	pub fn row_echelon_form(&self) -> Self {
		let (_, upper, _, _) = self.lu_decomp();
		upper
	}

	pub fn rank(&self) -> usize {
		let (_, _, _, rank) = self.lu_decomp();
		rank
	}

	pub fn try_det(&self) -> Option<Scalar> {
		if !self.is_square() {
			return None;
		}

		let ref_mat = self.row_echelon_form();
		let mut res = 1.0;
		for i in 0..ref_mat.height() {
			res *= ref_mat[(i, i)];
		}
		Some(res)
	}

	pub fn try_invert(mut self) -> Option<Self> {
		if !self.is_square() {
			return None;
		}

		let mut res = Self::identity_square(self.height());

		for prim in 0..self.ncols() {
			// Primary Diagonal Element (where row index = column index)
			if self[(prim, prim)] == 0.0 {
				let mut is_non_zero_row_found = false;

				for row in prim..self.height() {
					if self[(row, prim)] != 0.0 {
						self.swap_rows_starting_from(prim, row, prim); // Because everything before should be 0
						is_non_zero_row_found = true;
						break;
					}
				}

				if !is_non_zero_row_found {
					return None;
				}
			}

			// Divide the row by the element of the primary diagonal
			{
				let factor = 1.0 / self[(prim, prim)];
				for cell in prim..self.ncols() {
					self[(prim, cell)] *= factor;
				}

				res[(prim, prim)] = factor; // Multiplied by `res[(prim, prim)]` which is 1
			}

			// Then subtract that row from the other rows
			for row in 0..self.nrows() {
				if row == prim {
					continue;
				}

				let factor = self[(row, prim)]; // Divided by `self[(prim, prim)]` which is 1
				for cell in 0..self.ncols() {
					self[(row, cell)] -= self[(prim, cell)] * factor;
					res[(row, cell)] -= res[(prim, cell)] * factor;
				}
			}
		}

		// Check if was full-rank and therefore invertable
		if self.last_cell() != Some(&1.0) {
			return None;
		}
		for i in 0..(self.width() - 1) {
			if self[(self.height() - 1, i)] != 0.0 {
				return None;
			}
		}

		Some(res)
	}
}

////////////////////////////////////
////// Indexing and Iteration //////
////////////////////////////////////

impl Index<usize> for Matrix {
	type Output = Scalar;
	fn index(&self, index: usize) -> &Self::Output {
		self.data.get(index).unwrap()
	}
}

impl IndexMut<usize> for Matrix {
	fn index_mut(&mut self, index: usize) -> &mut Self::Output {
		self.data.get_mut(index).unwrap()
	}
}

impl Index<(usize, usize)> for Matrix {
	type Output = Scalar;
	fn index(&self, index: (usize, usize)) -> &Self::Output {
		assert!(index.0 < self.nrows(), "Matrix was indexed out of bounds");
		assert!(index.1 < self.ncols(), "Matrix was indexed out of bounds");
		self.get(index.0, index.1).unwrap()
	}
}

impl IndexMut<(usize, usize)> for Matrix {
	fn index_mut(&mut self, index: (usize, usize)) -> &mut Self::Output {
		assert!(index.0 < self.nrows(), "Matrix was indexed out of bounds");
		assert!(index.1 < self.ncols(), "Matrix was indexed out of bounds");
		self.get_mut(index.0, index.1).unwrap()
	}
}

impl Matrix {
	pub fn get(&self, row: usize, col: usize) -> Option<&Scalar> {
		assert!(row < self.nrows(), "Matrix was indexed out of bounds");
		assert!(col < self.ncols(), "Matrix was indexed out of bounds");
		self.data.get(row * self.width() + col)
	}

	pub fn get_mut(&mut self, row: usize, col: usize) -> Option<&mut Scalar> {
		assert!(row < self.nrows(), "Matrix was indexed out of bounds");
		assert!(col < self.ncols(), "Matrix was indexed out of bounds");
		let row_start = row * self.width(); // Isolated to get around the borrow checker
		self.data.get_mut(row_start + col)
	}

	#[inline]
	pub fn iter(&self) -> Iter<'_, f64> {
		self.data.iter()
	}

	#[inline]
	pub fn iter_mut(&mut self) -> IterMut<'_, f64> {
		self.data.iter_mut()
	}

	#[inline]
	pub fn last_cell(&self) -> Option<&Scalar> {
		self.data.last()
	}
}

//////////////////
////// Misc //////
//////////////////

impl Matrix {
	#[inline]
	pub fn get_shape(&self) -> (usize, usize) {
		self.shape
	}

	#[inline]
	pub fn width(&self) -> usize {
		self.shape.1
	}

	#[inline]
	pub fn height(&self) -> usize {
		self.shape.0
	}

	#[inline]
	pub fn nrows(&self) -> usize {
		self.shape.0
	}

	#[inline]
	pub fn ncols(&self) -> usize {
		self.shape.1
	}

	#[inline]
	pub fn is_square(&self) -> bool {
		self.width() == self.height()
	}

	pub fn swap_rows(&mut self, row1: usize, row2: usize) {
		assert!(
			row1 < self.nrows() && row2 < self.nrows(),
			"Matrix was indexed out of bounds"
		);

		for col in 0..self.ncols() {
			let temp = self[(row1, col)];
			self[(row1, col)] = self[(row2, col)];
			self[(row2, col)] = temp;
		}
	}

	pub fn swap_rows_starting_from(&mut self, row1: usize, row2: usize, start_col: usize) {
		assert!(
			row1 < self.nrows() && row2 < self.nrows(),
			"Matrix was indexed out of bounds"
		);

		// If `start_col` was larger than `self.ncols` this function won't execute
		for col in start_col..self.ncols() {
			let temp = self[(row1, col)];
			self[(row1, col)] = self[(row2, col)];
			self[(row2, col)] = temp;
		}
	}

	pub fn swap_rows_ending_at(&mut self, row1: usize, row2: usize, end_col: usize) {
		assert!(
			row1 < self.nrows() && row2 < self.nrows() && end_col < self.ncols(),
			"Matrix was indexed out of bounds"
		);

		for col in 0..=end_col {
			let temp = self[(row1, col)];
			self[(row1, col)] = self[(row2, col)];
			self[(row2, col)] = temp;
		}
	}

	pub fn swap_rows_ranged(&mut self, row1: usize, row2: usize, range: std::ops::Range<usize>) {
		assert!(
			row1 < self.nrows() && row2 < self.nrows() && range.end <= self.ncols(),
			"Matrix was indexed out of bounds"
		);

		for col in range {
			let temp = self[(row1, col)];
			self[(row1, col)] = self[(row2, col)];
			self[(row2, col)] = temp;
		}
	}
}

impl Display for Matrix {
	fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
		let mut buffer = String::new();

		buffer.push('\n');
		for row in 0..self.height() {
			buffer.push('\n');
			for col in 0..self.width() {
				buffer.push_str("   ");
				buffer.push_str(&self.get(row, col).unwrap().to_string());
			}
		}
		buffer.push('\n');

		write!(f, "{buffer}")
	}
}

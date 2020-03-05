use std::cmp::PartialEq;
use std::f32;
use auto_ops::*;
use super::vector2::Vector2;

#[derive(Debug, Clone)]
pub struct Matrix3(
	f32, f32, f32,
	f32, f32, f32,
	f32, f32, f32
);

impl Matrix3 {
	pub fn new() -> Matrix3 {
		Matrix3(1.0, 0.0, 0.0, 0.0, 1.0, 0.0, 0.0, 0.0, 1.0)
	}
	
	pub fn identity(&mut self) -> &Matrix3 {
		self.0 = 1.0;
		self.1 = 0.0;
		self.2 = 0.0;
		self.3 = 0.0;
		self.4 = 1.0;
		self.5 = 0.0;
		self.6 = 0.0;
		self.7 = 0.0;
		self.8 = 1.0;
		self
	}

	pub fn transpose(&mut self) -> &Matrix3 {
		let mut t = self.1;
		self.1 = self.3;
		self.3 = t;
		t = self.2;
		self.2 = self.6;
		self.6 = t;
		t = self.5;
		self.5 = self.7;
		self.7 = t;
		self
	}

	pub fn scale(&mut self, sx: f32, sy: f32) -> &Matrix3 {
		self.0 *= sx;
		self.3 *= sx;
		self.6 *= sx;
		self.1 *= sy;
		self.4 *= sy;
		self.7 *= sy;
		self
	}

	pub fn rotate(&mut self, theta: f32) -> &Matrix3 {
		let c = theta.cos();
		let s = theta.sin();
		let a11 = self.0;
		let a12 = self.3;
		let a13 = self.6;
		let a21 = self.1;
		let a22 = self.4;
		let a23 = self.7;
		self.0 = c * a11 + s * a21;
		self.3 = c * a12 + s * a22;
		self.6 = c * a13 + s * a23;
		self.1 = -s * a11 + c * a21;
		self.4 = -s * a12 + c * a22;
		self.7 = -s * a13 + c * a23;
		self
	}

	pub fn translate(&mut self, x: f32, y: f32) -> &Matrix3 {
		self.0 += x * self.2;
		self.3 += x * self.5;
		self.6 += x * self.8;
		self.1 += y * self.2;
		self.4 += y * self.5;
		self.7 += y * self.8;
		self
	}
	
	pub fn determinant(mat: &Matrix3) -> f32 {
		mat.0 * mat.4 * mat.8 - mat.0 * mat.5 * mat.7 - mat.1 * mat.3 * mat.8 + mat.1 * mat.5 * mat.6 + mat.2 * mat.3 * mat.7 - mat.2 * mat.4 * mat.6
	}
}

impl PartialEq for Matrix3 {
	fn eq(&self, other: &Self) -> bool {
		self.0 == other.0 &&
		self.1 == other.1 &&
		self.2 == other.2 &&
		self.3 == other.3 &&
		self.4 == other.4 &&
		self.5 == other.5 &&
		self.6 == other.6 &&
		self.7 == other.7 &&
		self.8 == other.8
	}
}

macro_rules! matrix4mul {
	($lhs:expr, $rhs:expr) => (Matrix3(
		$lhs.0 * $rhs.0 + $lhs.1 * $rhs.3 + $lhs.2 * $rhs.6,
		$lhs.3 * $rhs.0 + $lhs.4 * $rhs.3 + $lhs.5 * $rhs.6,
		$lhs.6 * $rhs.0 + $lhs.7 * $rhs.3 + $lhs.8 * $rhs.6,
		$lhs.0 * $rhs.1 + $lhs.1 * $rhs.4 + $lhs.2 * $rhs.7,
		$lhs.3 * $rhs.1 + $lhs.4 * $rhs.4 + $lhs.5 * $rhs.7,
		$lhs.6 * $rhs.1 + $lhs.7 * $rhs.4 + $lhs.8 * $rhs.7,
		$lhs.0 * $rhs.2 + $lhs.1 * $rhs.5 + $lhs.2 * $rhs.8,
		$lhs.3 * $rhs.2 + $lhs.4 * $rhs.5 + $lhs.5 * $rhs.8,
		$lhs.6 * $rhs.2 + $lhs.7 * $rhs.5 + $lhs.8 * $rhs.8,
	));
}

impl_op_ex!(* |lhs: &Matrix3, rhs: &Matrix3| -> Matrix3 { 
	matrix4mul!(lhs, rhs)
});
impl_op_ex!(*= |lhs: &mut Matrix3, rhs: &Matrix3| {
	let t = matrix4mul!(lhs, rhs);
	lhs.0 = t.0;
	lhs.1 = t.1;
	lhs.2 = t.2;
	lhs.3 = t.3;
	lhs.4 = t.4;
	lhs.5 = t.5;
	lhs.6 = t.6;
	lhs.7 = t.7;
	lhs.8 = t.8;
});
impl_op_ex!(* |lhs: &Vector2, rhs: &Matrix3| -> Vector2 { 
	Vector2::new(rhs.0 * lhs.x + rhs.3 * lhs.y + rhs.6, rhs.1 * lhs.x + rhs.4 * lhs.y + rhs.7)
});
impl_op_ex!(*= |lhs: &mut Vector2, rhs: &Matrix3| {
	let x = rhs.0 * lhs.x + rhs.3 * lhs.y + rhs.6;
	let y = rhs.1 * lhs.x + rhs.4 * lhs.y + rhs.7;
	lhs.x = x;
	lhs.y = y;
});
impl_op_ex!(* |lhs: &Matrix3, rhs: &Vector2| -> Vector2 { 
	Vector2::new(lhs.0 * rhs.x + lhs.3 * rhs.y + lhs.6, lhs.1 * rhs.x + lhs.4 * rhs.y + lhs.7)
});

#[cfg(test)]
mod tests {
	use super::*;

	#[test]
	pub fn it_identity() {
		assert_eq!(Matrix3::new(), Matrix3(1.0, 0.0, 0.0, 0.0, 1.0, 0.0, 0.0, 0.0, 1.0));
	}
}
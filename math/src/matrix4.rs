use std::cmp::PartialEq;
use std::f32;
use auto_ops::*;
use super::vector3::Vector3;

#[derive(Debug, Clone)]
pub struct Matrix4(
	f32, f32, f32, f32,
	f32, f32, f32, f32,
	f32, f32, f32, f32,
	f32, f32, f32, f32
);

impl Matrix4 {
	pub fn new() -> Matrix4 {
		Matrix4(1.0, 0.0, 0.0, 0.0, 0.0, 1.0, 0.0, 0.0, 0.0, 0.0, 1.0, 0.0, 0.0, 0.0, 0.0, 1.0)
	}
	
	pub fn identity(&mut self) -> &Matrix4 {
		self.0 = 1.0;
		self.1 = 0.0;
		self.2 = 0.0;
		self.3 = 0.0;
		self.4 = 0.0;
		self.5 = 1.0;
		self.6 = 0.0;
		self.7 = 0.0;
		self.8 = 0.0;
		self.9 = 0.0;
		self.10 = 1.0;
		self.11 = 0.0;
		self.12 = 0.0;
		self.13 = 0.0;
		self.14 = 0.0;
		self.15 = 1.0;
		self
	}

	pub fn transpose(&mut self) -> &Matrix4 {
		let mut t = self.1;
		self.1 = self.4;
		self.4 = t;
		t = self.2;
		self.2 = self.8;
		self.8 = t;
		t = self.6;
		self.6 = self.9;
		self.9 = t;
		t = self.3;
		self.3 = self.12;
		self.12 = t;
		t = self.7;
		self.7 = self.13;
		self.13 = t;
		t = self.11;
		self.11 = self.14;
		self.14 = t;
		self
	}

	pub fn position(&mut self, position: &Vector3) -> &Matrix4 {
		self.12 = position.x;
		self.13 = position.y;
		self.14 = position.z;
		self
	}

	pub fn scale(&mut self, sx: f32, sy: f32, sz: f32) -> &Matrix4 {
		self.0 *= sx;
		self.4 *= sy;
		self.8 *= sz;
		self.1 *= sx;
		self.5 *= sy;
		self.9 *= sz;
		self.2 *= sx;
		self.6 *= sy;
		self.10 *= sz;
		self.3 *= sx;
		self.7 *= sy;
		self.11 *= sz;
		self
	}

	// pub fn rotate(&mut self, theta: f32) -> &Matrix4 {
	// 	let c = theta.cos();
	// 	let s = theta.sin();
	// 	let a11 = self.0;
	// 	let a12 = self.3;
	// 	let a13 = self.6;
	// 	let a21 = self.1;
	// 	let a22 = self.4;
	// 	let a23 = self.7;
	// 	self.0 = c * a11 + s * a21;
	// 	self.3 = c * a12 + s * a22;
	// 	self.6 = c * a13 + s * a23;
	// 	self.1 = -s * a11 + c * a21;
	// 	self.4 = -s * a12 + c * a22;
	// 	self.7 = -s * a13 + c * a23;
	// 	self
	// }

	// pub fn translate(&mut self, x: f32, y: f32) -> &Matrix4 {
	// 	self.0 += x * self.2;
	// 	self.3 += x * self.5;
	// 	self.6 += x * self.8;
	// 	self.1 += y * self.2;
	// 	self.4 += y * self.5;
	// 	self.7 += y * self.8;
	// 	self
	// }
	
	// pub fn determinant(mat: &Matrix4) -> f32 {
	// 	mat.0 * mat.4 * mat.8 - mat.0 * mat.5 * mat.7 - mat.1 * mat.3 * mat.8 + mat.1 * mat.5 * mat.6 + mat.2 * mat.3 * mat.7 - mat.2 * mat.4 * mat.6
	// }
}

impl PartialEq for Matrix4 {
	fn eq(&self, other: &Self) -> bool {
		self.0 == other.0 &&
		self.1 == other.1 &&
		self.2 == other.2 &&
		self.3 == other.3 &&
		self.4 == other.4 &&
		self.5 == other.5 &&
		self.6 == other.6 &&
		self.7 == other.7 &&
		self.8 == other.8 &&
		self.9 == other.9 &&
		self.10 == other.10 &&
		self.11 == other.11 &&
		self.12 == other.12 &&
		self.13 == other.13 &&
		self.14 == other.14 &&
		self.15 == other.15
	}
}

macro_rules! matrix4mul {
	($lhs:expr, $rhs:expr) => (Matrix4(
		$lhs.0 * $rhs.0 + $lhs.4 * $rhs.1 + $lhs.8 * $rhs.2 + $lhs.12 * $rhs.3,
		$lhs.1 * $rhs.0 + $lhs.5 * $rhs.1 + $lhs.9 * $rhs.2 + $lhs.13 * $rhs.3,
		$lhs.2 * $rhs.0 + $lhs.6 * $rhs.1 + $lhs.10 * $rhs.2 + $lhs.14 * $rhs.3,
		$lhs.3 * $rhs.0 + $lhs.7 * $rhs.1 + $lhs.11 * $rhs.2 + $lhs.15 * $rhs.3,
		$lhs.0 * $rhs.4 + $lhs.4 * $rhs.5 + $lhs.8 * $rhs.6 + $lhs.12 * $rhs.7,
		$lhs.1 * $rhs.4 + $lhs.5 * $rhs.5 + $lhs.9 * $rhs.6 + $lhs.13 * $rhs.7,
		$lhs.2 * $rhs.4 + $lhs.6 * $rhs.5 + $lhs.10 * $rhs.6 + $lhs.14 * $rhs.7,
		$lhs.3 * $rhs.4 + $lhs.7 * $rhs.5 + $lhs.11 * $rhs.6 + $lhs.15 * $rhs.7,
		$lhs.0 * $rhs.8 + $lhs.4 * $rhs.9 + $lhs.8 * $rhs.10 + $lhs.12 * $rhs.11,
		$lhs.1 * $rhs.8 + $lhs.5 * $rhs.9 + $lhs.9 * $rhs.10 + $lhs.13 * $rhs.11,
		$lhs.2 * $rhs.8 + $lhs.6 * $rhs.9 + $lhs.10 * $rhs.10 + $lhs.14 * $rhs.11,
		$lhs.3 * $rhs.8 + $lhs.7 * $rhs.9 + $lhs.11 * $rhs.10 + $lhs.15 * $rhs.11,
		$lhs.0 * $rhs.12 + $lhs.4 * $rhs.13 + $lhs.8 * $rhs.14 + $lhs.12 * $rhs.15,
		$lhs.1 * $rhs.12 + $lhs.5 * $rhs.13 + $lhs.9 * $rhs.14 + $lhs.13 * $rhs.15,
		$lhs.2 * $rhs.12 + $lhs.6 * $rhs.13 + $lhs.10 * $rhs.14 + $lhs.14 * $rhs.15,
		$lhs.3 * $rhs.12 + $lhs.7 * $rhs.13 + $lhs.11 * $rhs.14 + $lhs.15 * $rhs.15
	));
}

impl_op_ex!(* |lhs: &Matrix4, rhs: &Matrix4| -> Matrix4 { 
	matrix4mul!(lhs, rhs)
});
impl_op_ex!(*= |lhs: &mut Matrix4, rhs: &Matrix4| {
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
	lhs.9 = t.9;
	lhs.10 = t.10;
	lhs.11 = t.11;
	lhs.12 = t.12;
	lhs.13 = t.13;
	lhs.14 = t.14;
	lhs.15 = t.15;
});

#[cfg(test)]
mod tests {
	use super::*;

	#[test]
	pub fn it_identity() {
		assert_eq!(Matrix4::new(), Matrix4(1.0, 0.0, 0.0, 0.0, 0.0, 1.0, 0.0, 0.0, 0.0, 0.0, 1.0, 0.0, 0.0, 0.0, 0.0, 1.0));
	}
}
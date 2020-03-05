use std::cmp::PartialEq;
use std::f32;
use auto_ops::*;

#[derive(Debug, Clone)]
pub struct Vector3 {
	pub x: f32,
	pub y: f32,
	pub z: f32,
}

impl Vector3 {
	pub fn new(x: f32, y: f32, z: f32) -> Vector3 {
		Vector3 { x, y, z }
	}

	pub fn set(&mut self, x: f32, y: f32, z: f32) -> &Vector3 {
		self.x = x;
		self.y = y;
		self.z = z;
		self
	}

	pub fn normalize(&mut self) -> &Vector3 {
		let length = Vector3::length(self);
		self.x /= length;
		self.y /= length;
		self.z /= length;
		self
	}

	pub fn min(&mut self, other: &Vector3) -> &Vector3 {
		self.x = self.x.min(other.x);
		self.y = self.y.min(other.y);
		self.z = self.z.min(other.z);
		self
	}

	pub fn max(&mut self, other: &Vector3) -> &Vector3 {
		self.x = self.x.max(other.x);
		self.y = self.y.max(other.y);
		self.z = self.z.max(other.z);
		self
	}

	pub fn clamp(&mut self, min: &Vector3, max: &Vector3) -> &Vector3 {
		self.x = self.x.max(min.x).min(max.x);
		self.y = self.y.max(min.y).min(max.y);
		self.z = self.z.max(min.z).min(max.z);
		self
	}

	pub fn clamp_length(&mut self, min: f32, max: f32) -> &Vector3 {
		let length = Vector3::length(self);
		let clamped = length.min(max).max(min);
		self.x /= length;
		self.y /= length;
		self.z /= length;
		self.x *= clamped;
		self.y *= clamped;
		self.z *= clamped;
		self
	}

	pub fn project(&mut self, normal: &Vector3) -> &Vector3 {
		let d = Vector3::dot(normal, self);
		let l = Vector3::length_squared(normal);
		self.x *= d / l;
		self.y *= d / l;
		self.z *= d / l;
		self
	}

	pub fn reflect(&mut self, normal: &Vector3) -> &Vector3 {
		let d = Vector3::dot(self, normal) * 2.0;
		self.x -= normal.x * d;
		self.y -= normal.y * d;
		self.z -= normal.z * d;
		self
	}

	#[inline]
	pub fn length_squared(vec: &Vector3) -> f32 {
		vec.x * vec.x + vec.y * vec.y + vec.z * vec.z
	}

	#[inline]
	pub fn length(vec: &Vector3) -> f32 {
		Vector3::length_squared(vec).sqrt()
	}

	#[inline]
	pub fn length_manhattan(vec: &Vector3) -> f32 {
		vec.x.abs() + vec.y.abs() + vec.z.abs()
	}

	#[inline]
	pub fn dot(lhs: &Vector3, rhs: &Vector3) -> f32 {
		lhs.x * rhs.x + lhs.y * rhs.y + lhs.z * rhs.z
	}

	pub fn cross(&mut self, lhs: &Vector3, rhs: &Vector3) -> &Vector3 {
		self.x = lhs.y * rhs.z - lhs.z * rhs.y;
		self.y = lhs.z * rhs.x - lhs.x * rhs.z;
		self.z = lhs.x * rhs.y - lhs.y * rhs.x;
		self
	}
	
	pub fn angle_between(lhs: &Vector3, rhs: &Vector3) -> f32 {
		let t = Vector3::dot(lhs, rhs) / (Vector3::length_squared(lhs) * Vector3::length_squared(rhs)).sqrt();
		t.max(-1.0).min(1.0).acos()
	}

	pub fn distance(lhs: &Vector3, rhs: &Vector3) -> f32 {
		Vector3::distance_squared(lhs, rhs).sqrt()
	}

	pub fn distance_squared(lhs: &Vector3, rhs: &Vector3) -> f32 {
		let dx = lhs.x - rhs.x;
		let dy = lhs.y - rhs.y;
		let dz = lhs.z - rhs.z;
		dx * dx + dy * dy + dz * dz
	}

	pub fn distance_manhattan(lhs: &Vector3, rhs: &Vector3) -> f32 {
		let dx = lhs.x - rhs.x;
		let dy = lhs.y - rhs.y;
		let dz = lhs.z - rhs.z;
		dx.abs() + dy.abs() + dz.abs()
	}
}

impl PartialEq for Vector3 {
	fn eq(&self, other: &Self) -> bool {
		self.x == other.x && self.y == other.y
	}
}

impl_op_ex!(+ |lhs: &Vector3, rhs: &Vector3| -> Vector3 { Vector3::new(lhs.x + rhs.x, lhs.y + rhs.y, lhs.z + rhs.z) });
impl_op_ex!(- |lhs: &Vector3, rhs: &Vector3| -> Vector3 { Vector3::new(lhs.x - rhs.x, lhs.y - rhs.y, lhs.z - rhs.z) });
impl_op_ex!(* |lhs: &Vector3, rhs: &Vector3| -> Vector3 { Vector3::new(lhs.x * rhs.x, lhs.y * rhs.y, lhs.z * rhs.z) });
impl_op_ex!(/ |lhs: &Vector3, rhs: &Vector3| -> Vector3 { Vector3::new(lhs.x / rhs.x, lhs.y / rhs.y, lhs.z / rhs.z) });
impl_op_ex!(+ |lhs: &Vector3, rhs: &f32| -> Vector3 { Vector3::new(lhs.x + rhs, lhs.y + rhs, lhs.z + rhs) });
impl_op_ex!(- |lhs: &Vector3, rhs: &f32| -> Vector3 { Vector3::new(lhs.x - rhs, lhs.y - rhs, lhs.z - rhs) });
impl_op_ex!(* |lhs: &Vector3, rhs: &f32| -> Vector3 { Vector3::new(lhs.x * rhs, lhs.y * rhs, lhs.z * rhs) });
impl_op_ex!(/ |lhs: &Vector3, rhs: &f32| -> Vector3 { Vector3::new(lhs.x / rhs, lhs.y / rhs, lhs.z / rhs) });
impl_op_ex!(- |lhs: &Vector3| -> Vector3 { Vector3::new(-lhs.x, -lhs.x, -lhs.z) });
impl_op_ex!(-= |lhs: &mut Vector3, rhs: &Vector3| { lhs.x -= rhs.x; lhs.y -= rhs.y; lhs.z -= rhs.z });
impl_op_ex!(+= |lhs: &mut Vector3, rhs: &Vector3| { lhs.x += rhs.x; lhs.y += rhs.y; lhs.z += rhs.z });
impl_op_ex!(*= |lhs: &mut Vector3, rhs: &Vector3| { lhs.x *= rhs.x; lhs.y *= rhs.y; lhs.z *= rhs.z });
impl_op_ex!(/= |lhs: &mut Vector3, rhs: &Vector3| { lhs.x /= rhs.x; lhs.y /= rhs.y; lhs.z /= rhs.z });
impl_op_ex!(+= |lhs: &mut Vector3, rhs: &f32| { lhs.x += rhs; lhs.y += rhs; lhs.z += rhs; });
impl_op_ex!(-= |lhs: &mut Vector3, rhs: &f32| { lhs.x -= rhs; lhs.y -= rhs; lhs.z -= rhs; });
impl_op_ex!(*= |lhs: &mut Vector3, rhs: &f32| { lhs.x *= rhs; lhs.y *= rhs; lhs.z *= rhs; });
impl_op_ex!(/= |lhs: &mut Vector3, rhs: &f32| { lhs.x /= rhs; lhs.y /= rhs; lhs.z /= rhs; });

#[cfg(test)]
mod tests {
	use super::*;

	#[test]
	fn it_adds() {
		assert_eq!(Vector3::new(1.0, 1.0, 1.0) + Vector3::new(1.0, 1.0, 1.0), Vector3::new(2.0, 2.0, 2.0));
		assert_eq!(Vector3::new(0.0, 0.0, 0.0) + 2.0, Vector3::new(2.0, 2.0, 2.0));
		let mut v1 = Vector3::new(0.0, 0.0, 0.0);
		v1 += 2.0;
		assert_eq!(v1, Vector3::new(2.0, 2.0, 2.0));
	}
	
	#[test]
	fn it_substracts() {
		assert_eq!(Vector3::new(1.0, 1.0, 1.0) - Vector3::new(1.0, 1.0, 1.0), Vector3::new(0.0, 0.0, 0.0));
		assert_eq!(Vector3::new(0.0, 0.0, 0.0) - 2.0, Vector3::new(-2.0, -2.0, -2.0));
		let mut v1 = Vector3::new(2.0, 2.0, 2.0);
		v1 -= 2.0;
		assert_eq!(v1, Vector3::new(0.0, 0.0, 0.0));
	}

	#[test]
	fn it_multiplies() {
		assert_eq!(Vector3::new(1.0, 1.0, 1.0) * Vector3::new(1.0, 1.0, 1.0), Vector3::new(1.0, 1.0, 1.0));
		assert_eq!(Vector3::new(1.0, 0.0, 2.0) * 2.0, Vector3::new(2.0, 0.0, 4.0));
		let mut v1 = Vector3::new(2.0, 2.0, 2.0);
		v1 *= 2.0;
		assert_eq!(v1, Vector3::new(4.0, 4.0, 4.0));
	}

	#[test]
	fn it_divides() {
		assert_eq!(Vector3::new(4.0, 4.0, 4.0) / Vector3::new(2.0, 1.0, 2.0), Vector3::new(2.0, 4.0, 2.0));
		assert_eq!(Vector3::new(4.0, 2.0, 2.0) / 2.0, Vector3::new(2.0, 1.0, 1.0));
		let mut v1 = Vector3::new(4.0, 4.0, 4.0);
		v1 /= 2.0;
		assert_eq!(v1, Vector3::new(2.0, 2.0, 2.0));
	}
}

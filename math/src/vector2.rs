use std::cmp::PartialEq;
use std::f32;
use auto_ops::*;

#[derive(Debug, Clone)]
pub struct Vector2 {
	pub x: f32,
	pub y: f32,
}

impl Vector2 {
	pub fn new(x: f32, y: f32) -> Vector2 {
		Vector2 { x, y }
	}

	pub fn set(&mut self, x: f32, y: f32) -> &Vector2 {
		self.x = x;
		self.y = y;
		self
	}

	pub fn normalize(&mut self) -> &Vector2 {
		let length = Vector2::length(self);
		self.x /= length;
		self.y /= length;
		self
	}

	pub fn min(&mut self, other: &Vector2) -> &Vector2 {
		self.x = self.x.min(other.x);
		self.y = self.y.min(other.y);
		self
	}

	pub fn max(&mut self, other: &Vector2) -> &Vector2 {
		self.x = self.x.max(other.x);
		self.y = self.y.max(other.y);
		self
	}

	pub fn clamp(&mut self, min: &Vector2, max: &Vector2) -> &Vector2 {
		self.x = self.x.max(min.x).min(max.x);
		self.y = self.y.max(min.y).min(max.y);
		self
	}

	pub fn clamp_length(&mut self, min: f32, max: f32) -> &Vector2 {
		let length = Vector2::length(self);
		let clamped = length.min(max).max(min);
		self.x /= length;
		self.y /= length;
		self.x *= clamped;
		self.y *= clamped;
		self
	}

	pub fn project(&mut self, normal: &Vector2) -> &Vector2 {
		let d = Vector2::dot(normal, self);
		let l = Vector2::length_squared(normal);
		self.x *= d / l;
		self.y *= d / l;
		self
	}

	pub fn reflect(&mut self, normal: &Vector2) -> &Vector2 {
		let d = Vector2::dot(self, normal) * 2.0;
		self.x -= normal.x * d;
		self.y -= normal.y * d;
		self
	}

	#[inline]
	pub fn length_squared(vec: &Vector2) -> f32 {
		vec.x * vec.x + vec.y * vec.y
	}

	#[inline]
	pub fn length(vec: &Vector2) -> f32 {
		Vector2::length_squared(vec).sqrt()
	}

	#[inline]
	pub fn length_manhattan(vec: &Vector2) -> f32 {
		vec.x.abs() + vec.y.abs()
	}

	#[inline]
	pub fn angle(vec: &Vector2) -> f32 {
		let angle = vec.y.atan2(vec.x);
		if angle < 0.0 {
			angle + 2.0 * f32::consts::PI
		} else {
			angle
		}
	}

	#[inline]
	pub fn dot(lhs: &Vector2, rhs: &Vector2) -> f32 {
		lhs.x * rhs.x + lhs.y * rhs.y
	}

	#[inline]
	pub fn cross(lhs: &Vector2, rhs: &Vector2) -> f32 {
		lhs.x * rhs.x - lhs.y * rhs.y
	}

	pub fn angle_between(lhs: &Vector2, rhs: &Vector2) -> f32 {
		let t = Vector2::dot(lhs, rhs) / (Vector2::length_squared(lhs) * Vector2::length_squared(rhs)).sqrt();
		t.max(-1.0).min(1.0).acos()
	}

	pub fn distance(lhs: &Vector2, rhs: &Vector2) -> f32 {
		Vector2::distance_squared(lhs, rhs).sqrt()
	}

	pub fn distance_squared(lhs: &Vector2, rhs: &Vector2) -> f32 {
		let dx = lhs.x - rhs.x;
		let dy = lhs.y - rhs.y;
		dx * dx + dy * dy
	}

	pub fn distance_manhattan(lhs: &Vector2, rhs: &Vector2) -> f32 {
		let dx = lhs.x - rhs.x;
		let dy = lhs.y - rhs.y;
		dx.abs() + dy.abs()
	}
}

impl PartialEq for Vector2 {
	fn eq(&self, other: &Self) -> bool {
		self.x == other.x && self.y == other.y
	}
}

impl_op_ex!(+ |lhs: &Vector2, rhs: &Vector2| -> Vector2 { Vector2::new(lhs.x + rhs.x, lhs.y + rhs.y) });
impl_op_ex!(- |lhs: &Vector2, rhs: &Vector2| -> Vector2 { Vector2::new(lhs.x - rhs.x, lhs.y - rhs.y) });
impl_op_ex!(* |lhs: &Vector2, rhs: &Vector2| -> Vector2 { Vector2::new(lhs.x * rhs.x, lhs.y * rhs.y) });
impl_op_ex!(/ |lhs: &Vector2, rhs: &Vector2| -> Vector2 { Vector2::new(lhs.x / rhs.x, lhs.y / rhs.y) });
impl_op_ex!(+ |lhs: &Vector2, rhs: &f32| -> Vector2 { Vector2::new(lhs.x + rhs, lhs.y + rhs) });
impl_op_ex!(- |lhs: &Vector2, rhs: &f32| -> Vector2 { Vector2::new(lhs.x - rhs, lhs.y - rhs) });
impl_op_ex!(* |lhs: &Vector2, rhs: &f32| -> Vector2 { Vector2::new(lhs.x * rhs, lhs.y * rhs) });
impl_op_ex!(/ |lhs: &Vector2, rhs: &f32| -> Vector2 { Vector2::new(lhs.x / rhs, lhs.y / rhs) });
impl_op_ex!(- |lhs: &Vector2| -> Vector2 { Vector2::new(-lhs.x, -lhs.x) });
impl_op_ex!(+= |lhs: &mut Vector2, rhs: &Vector2| { lhs.x += rhs.x; lhs.y += rhs.y; });
impl_op_ex!(-= |lhs: &mut Vector2, rhs: &Vector2| { lhs.x -= rhs.x; lhs.y -= rhs.y; });
impl_op_ex!(*= |lhs: &mut Vector2, rhs: &Vector2| { lhs.x *= rhs.x; lhs.y *= rhs.y; });
impl_op_ex!(/= |lhs: &mut Vector2, rhs: &Vector2| { lhs.x /= rhs.x; lhs.y /= rhs.y; });
impl_op_ex!(+= |lhs: &mut Vector2, rhs: &f32| { lhs.x += rhs; lhs.y += rhs; });
impl_op_ex!(-= |lhs: &mut Vector2, rhs: &f32| { lhs.x -= rhs; lhs.y -= rhs; });
impl_op_ex!(*= |lhs: &mut Vector2, rhs: &f32| { lhs.x *= rhs; lhs.y *= rhs; });
impl_op_ex!(/= |lhs: &mut Vector2, rhs: &f32| { lhs.x /= rhs; lhs.y /= rhs; });

#[cfg(test)]
mod tests {
	use super::*;

	#[test]
	fn it_adds() {
		assert_eq!(Vector2::new(1.0, 1.0) + Vector2::new(1.0, 1.0), Vector2::new(2.0, 2.0));
		assert_eq!(Vector2::new(0.0, 0.0) + 2.0, Vector2::new(2.0, 2.0));
		let mut v1 = Vector2::new(0.0, 0.0);
		v1 += 2.0;
		assert_eq!(v1, Vector2::new(2.0, 2.0));
	}
	
	#[test]
	fn it_substracts() {
		assert_eq!(Vector2::new(1.0, 1.0) - Vector2::new(1.0, 1.0), Vector2::new(0.0, 0.0));
		assert_eq!(Vector2::new(0.0, 0.0) - 2.0, Vector2::new(-2.0, -2.0));
		let mut v1 = Vector2::new(2.0, 2.0);
		v1 -= 2.0;
		assert_eq!(v1, Vector2::new(0.0, 0.0));
	}

	#[test]
	fn it_multiplies() {
		assert_eq!(Vector2::new(1.0, 1.0) * Vector2::new(1.0, 1.0), Vector2::new(1.0, 1.0));
		assert_eq!(Vector2::new(1.0, 0.0) * 2.0, Vector2::new(2.0, 0.0));
		let mut v1 = Vector2::new(2.0, 2.0);
		v1 *= 2.0;
		assert_eq!(v1, Vector2::new(4.0, 4.0));
	}

	#[test]
	fn it_divides() {
		assert_eq!(Vector2::new(4.0, 4.0) / Vector2::new(2.0, 1.0), Vector2::new(2.0, 4.0));
		assert_eq!(Vector2::new(4.0, 2.0) / 2.0, Vector2::new(2.0, 1.0));
		let mut v1 = Vector2::new(4.0, 4.0);
		v1 /= 2.0;
		assert_eq!(v1, Vector2::new(2.0, 2.0));
	}
}

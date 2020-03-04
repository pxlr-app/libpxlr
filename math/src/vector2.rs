use std::cmp::PartialEq;
use std::f32;
use auto_ops::*;

#[derive(Debug, Clone)]
pub struct Vector2 {
	x: f32,
	y: f32,
}

impl Vector2 {
	fn new(x: f32, y: f32) -> Vector2 {
		Vector2 { x, y }
	}

	#[inline]
	fn length_squared(&self) -> f32 {
		self.x * self.x + self.y * self.y
	}

	#[inline]
	fn length(&self) -> f32 {
		self.length_squared().sqrt()
	}

	#[inline]
	fn length_manhattan(&self) -> f32 {
		self.x.abs() + self.y.abs()
	}

	#[inline]
	fn angle(&self) -> f32 {
		let angle = self.y.atan2(self.x);
		if angle < 0.0 {
			angle + 2.0 * f32::consts::PI
		} else {
			angle
		}
	}

	fn set(&mut self, x: f32, y: f32) -> &Vector2 {
		self.x = x;
		self.y = y;
		self
	}

	fn normalize(&mut self) -> &Vector2 {
		let length = self.length();
		self.x /= length;
		self.y /= length;
		self
	}

	fn min(&mut self, other: &Vector2) -> &Vector2 {
		self.x = self.x.min(other.x);
		self.y = self.y.min(other.y);
		self
	}

	fn max(&mut self, other: &Vector2) -> &Vector2 {
		self.x = self.x.max(other.x);
		self.y = self.y.max(other.y);
		self
	}

	fn clamp(&mut self, min: &Vector2, max: &Vector2) -> &Vector2 {
		self.x = self.x.max(min.x).min(max.x);
		self.y = self.y.max(min.y).min(max.y);
		self
	}

	fn clamp_length(&mut self, min: f32, max: f32) -> &Vector2 {
		let length = self.length();
		let clamped = length.min(max).max(min);
		self.x /= length;
		self.y /= length;
		self.x *= clamped;
		self.y *= clamped;
		self
	}

	fn dot(lhs: &Vector2, rhs: &Vector2) -> f32 {
		lhs.x * rhs.x + lhs.y * rhs.y
	}

	fn cross(lhs: &Vector2, rhs: &Vector2) -> f32 {
		lhs.x * rhs.x - lhs.y * rhs.y
	}

	fn angle_between(lhs: &Vector2, rhs: &Vector2) -> f32 {
		let t = Vector2::dot(lhs, rhs) / (lhs.length_squared() * rhs.length_squared()).sqrt();
		t.max(-1.0).min(1.0).acos()
	}

	fn project(&mut self, normal: &Vector2) -> &Vector2 {
		let d = Vector2::dot(normal, self);
		let l = normal.length_squared();
		self.x *= d / l;
		self.y *= d / l;
		self
	}

	fn reflect(&mut self, normal: &Vector2) -> &Vector2 {
		let d = Vector2::dot(self, normal) * 2.0;
		self.x -= normal.x * d;
		self.y -= normal.y * d;
		self
	}
}

impl PartialEq for Vector2 {
	fn eq(&self, other: &Self) -> bool {
        self.x == other.x && self.y == other.y
    }
}

impl_op_ex!(+ |a: &Vector2, b: &Vector2| -> Vector2 { Vector2::new(a.x + b.x, a.y + b.y) });
impl_op_ex!(- |a: &Vector2, b: &Vector2| -> Vector2 { Vector2::new(a.x - b.x, a.y - b.y) });
impl_op_ex!(* |a: &Vector2, b: &Vector2| -> Vector2 { Vector2::new(a.x * b.x, a.y * b.y) });
impl_op_ex!(/ |a: &Vector2, b: &Vector2| -> Vector2 { Vector2::new(a.x / b.x, a.y / b.y) });
impl_op_ex!(+ |a: &Vector2, b: &f32| -> Vector2 { Vector2::new(a.x + b, a.y + b) });
impl_op_ex!(- |a: &Vector2, b: &f32| -> Vector2 { Vector2::new(a.x - b, a.y - b) });
impl_op_ex!(* |a: &Vector2, b: &f32| -> Vector2 { Vector2::new(a.x * b, a.y * b) });
impl_op_ex!(/ |a: &Vector2, b: &f32| -> Vector2 { Vector2::new(a.x / b, a.y / b) });
impl_op_ex!(- |a: &Vector2| -> Vector2 { Vector2::new(-a.x, -a.x) });
impl_op_ex!(+= |a: &mut Vector2, b: &Vector2| { a.x += b.x; a.y += b.y; });
impl_op_ex!(-= |a: &mut Vector2, b: &Vector2| { a.x -= b.x; a.y -= b.y; });
impl_op_ex!(*= |a: &mut Vector2, b: &Vector2| { a.x *= b.x; a.y *= b.y; });
impl_op_ex!(/= |a: &mut Vector2, b: &Vector2| { a.x /= b.x; a.y /= b.y; });
impl_op_ex!(+= |a: &mut Vector2, b: &f32| { a.x += b; a.y += b; });
impl_op_ex!(-= |a: &mut Vector2, b: &f32| { a.x -= b; a.y -= b; });
impl_op_ex!(*= |a: &mut Vector2, b: &f32| { a.x *= b; a.y *= b; });
impl_op_ex!(/= |a: &mut Vector2, b: &f32| { a.x /= b; a.y /= b; });

#[cfg(test)]
mod tests {
	use super::Vector2;

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

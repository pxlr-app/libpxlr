use std::cmp::PartialEq;
use std::f32;
use std::ops;

#[derive(Debug)]
pub struct Vector2 {
	x: f32,
	y: f32,
}

impl Vector2 {
	fn new(x: f32, y: f32) -> Vector2 {
		Vector2 { x, y }
	}

	#[inline]
	fn lengthSquared(&self) -> f32 {
		self.x * self.x + self.y * self.y
	}

	#[inline]
	fn length(&self) -> f32 {
		self.lengthSquared().sqrt()
	}

	#[inline]
	fn lengthManhattan(&self) -> f32 {
		self.x.abs() + self.y.abs()
	}

	#[inline]
	fn angle(&self) -> f32 {
		let angle = self.y.atan2(self.x);
		if (angle < 0.0) {
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

	fn normalize(&self) -> &Vector2 {
		self /= self.length();
		self
	}

	fn min(&self, other: &Vector2) -> Vector2 {
		Vector2::new(self.x.min(other.x), self.y.min(other.y))
	}

	fn max(&self, other: &Vector2) -> Vector2 {
		Vector2::new(self.x.max(other.x), self.y.max(other.y))
	}

	fn clamp(&mut self, min: &Vector2, max: &Vector2) -> &Vector2 {
		self.x = self.x.max(min.x).min(max.x);
		self.y = self.y.max(min.y).min(max.y);
		self
	}

	fn clampLength(&mut self, min: f32, max: f32) -> &Vector2 {
		let length = self.length();
		self /= length;
		self *= length.min(max).max(min);
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
impl_op_ex!(+ |a: &f32, b: &Vector2| -> Vector2 { Vector2::new(b.x + a, b.y + a) });
impl_op_ex!(- |a: &f32, b: &Vector2| -> Vector2 { Vector2::new(b.x - a, b.y - a) });
impl_op_ex!(* |a: &f32, b: &Vector2| -> Vector2 { Vector2::new(b.x * a, b.y * a) });
impl_op_ex!(/ |a: &f32, b: &Vector2| -> Vector2 { Vector2::new(b.x / a, b.y / a) });
impl_op_ex!(- |a: &Vector2| -> Vector2 { Vector2::new(-a.x, -a.x) });
impl_op_ex!(+= |a: &mut Vector2, b: &Vector2| { a.x += b.x; a.y += b.y });
impl_op_ex!(-= |a: &mut Vector2, b: &Vector2| { a.x -= b.x; a.y -= b.y });
impl_op_ex!(*= |a: &mut Vector2, b: &Vector2| { a.x *= b.x; a.y *= b.y });
impl_op_ex!(/= |a: &mut Vector2, b: &Vector2| { a.x /= b.x; a.y /= b.y });
impl_op_ex!(+= |a: &mut Vector2, b: &f32| { a.x += b; a.y += b });
impl_op_ex!(-= |a: &mut Vector2, b: &f32| { a.x -= b; a.y -= b });
impl_op_ex!(*= |a: &mut Vector2, b: &f32| { a.x *= b; a.y *= b });
impl_op_ex!(/= |a: &mut Vector2, b: &f32| { a.x /= b; a.y /= b });

#[cfg(test)]
mod tests {
	use super::Vector2;

    #[test]
    fn it_adds() {
		assert_eq!(Vector2::new(1.0, 1.0) + Vector2::new(1.0, 1.0), Vector2::new(2.0, 2.0));
		assert_eq!(Vector2::new(1.0, 1.0) - Vector2::new(1.0, 1.0), Vector2::new(0.0, 0.0));
		assert_eq!(Vector2::new(0.0, 0.0) + 2.0, Vector2::new(2.0, 2.0));
		assert_eq!(2.0 + Vector2::new(0.0, 0.0), Vector2::new(2.0, 2.0));
    }
}
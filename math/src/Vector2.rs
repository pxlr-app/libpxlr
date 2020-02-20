use std::ops::{ Add };
use std::cmp::PartialEq;
use std::f32;

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
	fn lengthSquared(self) -> f32 {
		self.x * self.x + self.y * self.y
	}

	#[inline]
	fn length(self) -> f32 {
		self.lengthSquared().sqrt()
	}

	#[inline]
	fn lengthManhattan(self) -> f32 {
		self.x.abs() + self.y.abs()
	}

	fn angle(self) -> f32 {
		let angle = self.y.atan2(self.x);
		if (angle < 0.0) {
			angle + 2.0 * f32::consts::PI
		} else {
			angle
		}
	}
}

impl PartialEq for Vector2 {
	fn eq(&self, other: &Self) -> bool {
        self.x == other.x && self.y == other.y
    }
}

impl Add<Vector2> for Vector2 {
	type Output = Vector2;

	fn add(self, other: Vector2) -> Vector2 {
		Vector2::new(self.x + other.x, self.y + other.y)
	}
}

impl Add<f32> for Vector2 {
	type Output = Vector2;

	fn add(self, other: f32) -> Vector2 {
		Vector2::new(self.x + other, self.y + other)
	}
}

impl Add<Vector2> for f32 {
	type Output = Vector2;

	fn add(self, other: Vector2) -> Vector2 {
		Vector2::new(other.x + self, other.y + self)
	}
}

#[cfg(test)]
mod tests {
	use super::Vector2;

    #[test]
    fn it_adds() {
		assert_eq!(Vector2::new(0.0, 0.0) + Vector2::new(1.0, 1.0), Vector2::new(1.0, 1.0));
		assert_eq!(Vector2::new(0.0, 0.0) + 2.0, Vector2::new(2.0, 2.0));
		assert_eq!(2.0 + Vector2::new(0.0, 0.0), Vector2::new(2.0, 2.0));
    }
}
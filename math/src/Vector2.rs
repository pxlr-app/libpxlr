use std::ops::{ Add };
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

	fn lengthSquared(self) -> f32 {
		self.x * self.x + self.y * self.y
	}

	fn length(self) -> f32 {
		self.lengthSquared().sqrt()
	}

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

impl Add for Vector2 {
	type Output = Self;

	fn add(self, other: Self) -> Vector2 {
		Vector2::new(self.x + other.x, self.y + other.y)
	}
}

impl Add<f32> for Vector2 {
	type Output = Self;

	fn add(self, other: f32) -> Vector2 {
		Vector2::new(self.x + other, self.y + other)
	}
}

#[cfg(test)]
mod tests {
    #[test]
    fn it_adds() {
        assert_eq!(Vector2::new(0, 0) + Vector2::new(1, 1), Vector2::new(1, 1));
    }
}
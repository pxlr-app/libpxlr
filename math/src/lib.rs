pub use nalgebra_glm::*;

#[derive(PartialEq, Clone)]
pub struct Rect {
	pub min: Vec2,
	pub max: Vec2
}

impl Rect {
	pub fn new(min: Vec2, max: Vec2) -> Rect {
		Rect { min: min, max: max }
	}

	pub fn center(&self) -> Vec2 {
		self.min + self.max / 2.
	}
}
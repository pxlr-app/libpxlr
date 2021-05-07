use crate::{HasBounds, HasColors, Node};
use color::Rgba;
use serde::{Deserialize, Serialize};
use std::sync::Arc;
use uuid::Uuid;
use vek::{geom::repr_c::Rect, vec::repr_c::vec2::Vec2};

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct Palette {
	id: Uuid,
	name: String,
	position: Arc<Vec2<i32>>,
	colors: Vec<Rgba>,
}

impl Palette {
	pub unsafe fn construct(
		id: Uuid,
		name: String,
		position: Vec2<i32>,
		colors: Vec<Rgba>,
	) -> Self {
		Palette {
			id: id,
			name: name,
			position: Arc::new(position),
			colors: colors,
		}
	}
	pub fn new<S: Into<String>, V: Into<Vec2<i32>>, C: Into<Vec<Rgba>>>(
		name: S,
		position: V,
		colors: C,
	) -> Self {
		unsafe { Palette::construct(Uuid::new_v4(), name.into(), position.into(), colors.into()) }
	}
}

impl Default for Palette {
	fn default() -> Self {
		Palette::new("Palette", Vec2::new(0, 0), vec![])
	}
}

impl Node for Palette {
	fn id(&self) -> &Uuid {
		&self.id
	}
	fn set_id(&mut self, id: Uuid) {
		self.id = id;
	}
	fn name(&self) -> &str {
		&self.name
	}
	fn set_name(&mut self, name: String) {
		self.name = name;
	}
}

impl HasBounds for Palette {
	fn bounds(&self) -> Rect<i32, i32> {
		Rect::new(self.position.x, self.position.y, 0, 0)
	}
	fn set_position(&mut self, position: Vec2<i32>) {
		self.position = Arc::new(position);
	}
}

impl HasColors for Palette {
	fn colors(&self) -> &Vec<Rgba> {
		&self.colors
	}
	fn set_colors(&mut self, colors: Vec<Rgba>) {
		self.colors = colors;
	}
}

#[cfg(test)]
mod tests {
	use super::*;

	#[test]
	fn impl_default() {
		let note = Palette::default();
		assert_eq!(note.name(), "Palette");
		assert_eq!(note.bounds().into_aabr().min, Vec2::new(0, 0));
	}
}

use crate::{HasBounds, Node};
use std::sync::Arc;
use uuid::Uuid;
use vek::{geom::repr_c::Rect, vec::repr_c::vec2::Vec2};

#[derive(Debug, Clone, Eq, PartialEq)]
pub struct Unloaded {
	id: Uuid,
	name: String,
	bounds: Arc<Rect<i32, i32>>,
}

impl Unloaded {
	pub unsafe fn construct(id: Uuid, name: String, bounds: Rect<i32, i32>) -> Self {
		Unloaded {
			id: id,
			name: name,
			bounds: Arc::new(bounds),
		}
	}
	pub fn new<S: Into<String>, B: Into<Rect<i32, i32>>>(name: S, bounds: B) -> Self {
		unsafe { Self::construct(Uuid::new_v4(), name.into(), bounds.into()) }
	}
}

impl Default for Unloaded {
	fn default() -> Self {
		Unloaded::new("Unloaded", Rect::new(0, 0, 0, 0))
	}
}

impl Node for Unloaded {
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

impl HasBounds for Unloaded {
	fn bounds(&self) -> Rect<i32, i32> {
		*self.bounds
	}
	fn set_position(&mut self, position: Vec2<i32>) {
		self.bounds = Arc::new(Rect::new(
			position.x,
			position.y,
			self.bounds.w,
			self.bounds.h,
		));
	}
}

#[cfg(test)]
mod tests {
	use super::*;

	#[test]
	fn impl_default() {
		let note = Unloaded::default();
		assert_eq!(note.name(), "Unloaded");
		assert_eq!(note.bounds().into_aabr().min, Vec2::new(0, 0));
	}
}

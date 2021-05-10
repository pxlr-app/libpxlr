use crate::{HasBounds, HasContent, Node};
use std::sync::Arc;
use uuid::Uuid;
use vek::{geom::repr_c::Rect, vec::repr_c::vec2::Vec2};

#[derive(Debug, Clone, Eq, PartialEq)]
pub struct Note {
	id: Uuid,
	name: String,
	position: Arc<Vec2<i32>>,
	content: String,
}

impl Note {
	pub unsafe fn construct(id: Uuid, name: String, position: Vec2<i32>, content: String) -> Self {
		Note {
			id: id,
			name: name,
			position: Arc::new(position),
			content: content,
		}
	}
	pub fn new<S: Into<String>, V: Into<Vec2<i32>>>(name: S, position: V, content: S) -> Self {
		unsafe { Self::construct(Uuid::new_v4(), name.into(), position.into(), content.into()) }
	}
}

impl Default for Note {
	fn default() -> Self {
		Note::new("Note", Vec2::new(0, 0), "")
	}
}

impl Node for Note {
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

impl HasBounds for Note {
	fn bounds(&self) -> Rect<i32, i32> {
		Rect::new(self.position.x, self.position.y, 0, 0)
	}
	fn set_position(&mut self, position: Vec2<i32>) {
		self.position = Arc::new(position);
	}
}

impl HasContent for Note {
	fn content(&self) -> &str {
		&self.content
	}
	fn set_content(&mut self, content: String) {
		self.content = content;
	}
}

#[cfg(test)]
mod tests {
	use super::*;

	#[test]
	fn impl_default() {
		let note = Note::default();
		assert_eq!(note.name(), "Note");
		assert_eq!(note.bounds().into_aabr().min, Vec2::new(0, 0));
	}
}

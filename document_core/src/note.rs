use crate::Node;
use serde::{Deserialize, Serialize};
use std::sync::Arc;
use uuid::Uuid;
use vek::vec::repr_c::vec2::Vec2;

#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct Note {
	pub id: Uuid,
	pub name: Arc<String>,
	pub position: Arc<Vec2<u32>>,
}

impl Node for Note {
	fn id(&self) -> &Uuid {
		&self.id
	}
	fn display(&self) -> &str {
		&self.name
	}
}

#[cfg(test)]
mod tests {
	use super::Note;
	use vek::vec::repr_c::vec2::Vec2;

	#[test]
	fn impl_default() {
		let note = Note::default();
		assert_eq!(*note.name, "");
		assert_eq!(*note.position, Vec2::new(0, 0));
	}
}

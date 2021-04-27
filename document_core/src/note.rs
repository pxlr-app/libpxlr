use crate::Node;
use serde::{Deserialize, Serialize};
use std::sync::Arc;
use uuid::Uuid;
use vek::vec::repr_c::vec2::Vec2;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Note {
	pub id: Uuid,
	pub name: Arc<String>,
	pub position: Arc<Vec2<u32>>,
	pub content: Arc<String>,
}

impl Default for Note {
	fn default() -> Self {
		Note {
			id: Uuid::new_v4(),
			name: Arc::new("Note".into()),
			position: Arc::new(Vec2::new(0, 0)),
			content: Arc::new("".into()),
		}
	}
}

impl Node for Note {
	fn id(&self) -> &Uuid {
		&self.id
	}
	fn name(&self) -> &str {
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
		assert_eq!(*note.name, "Note");
		assert_eq!(*note.position, Vec2::new(0, 0));
	}
}

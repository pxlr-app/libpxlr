use crate::Node;
use color::Rgba;
use serde::{Deserialize, Serialize};
use std::sync::Arc;
use uuid::Uuid;
use vek::vec::repr_c::vec2::Vec2;

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct Palette {
	pub id: Uuid,
	pub name: Arc<String>,
	pub position: Arc<Vec2<i32>>,
	pub colors: Arc<Vec<Rgba>>,
}

impl Default for Palette {
	fn default() -> Self {
		Palette {
			id: Uuid::new_v4(),
			name: Arc::new("Palette".into()),
			position: Arc::new(Vec2::new(0, 0)),
			colors: Arc::new(vec![]),
		}
	}
}

impl Node for Palette {
	fn id(&self) -> &Uuid {
		&self.id
	}
	fn name(&self) -> &str {
		&self.name
	}
}

#[cfg(test)]
mod tests {
	use super::Palette;
	use vek::vec::repr_c::vec2::Vec2;

	#[test]
	fn impl_default() {
		let note = Palette::default();
		assert_eq!(*note.name, "Palette");
		assert_eq!(*note.position, Vec2::new(0, 0));
	}
}

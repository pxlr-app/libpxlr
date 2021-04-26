use crate::{Node, NodeType, NonLeafNode};
use serde::{Deserialize, Serialize};
use std::sync::Arc;
use uuid::Uuid;
use vek::vec::repr_c::vec2::Vec2;

#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct Group {
	pub id: Uuid,
	pub name: Arc<String>,
	pub position: Arc<Vec2<u32>>,
	pub children: Arc<Vec<Arc<NodeType>>>,
}

impl Node for Group {
	fn id(&self) -> &Uuid {
		&self.id
	}
	fn display(&self) -> &str {
		&self.name
	}
}

impl NonLeafNode for Group {
	fn children(&self) -> &Arc<Vec<Arc<NodeType>>> {
		&self.children
	}
}

#[cfg(test)]
mod tests {
	use super::Group;
	use vek::vec::repr_c::vec2::Vec2;

	#[test]
	fn impl_default() {
		let group = Group::default();
		assert_eq!(*group.name, "");
		assert_eq!(*group.position, Vec2::new(0, 0));
		assert_eq!(group.children.len(), 0);
	}
}

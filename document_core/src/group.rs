use crate::{Node, NodeType, NonLeafNode};
use serde::{Deserialize, Serialize};
use std::sync::Arc;
use uuid::Uuid;
use vek::vec::repr_c::vec2::Vec2;

#[derive(Debug, Clone, Eq, PartialEq, Serialize, Deserialize)]
pub struct Group {
	pub id: Uuid,
	pub name: Arc<String>,
	pub position: Arc<Vec2<u32>>,
	pub children: Arc<Vec<Arc<NodeType>>>,
}

impl Default for Group {
	fn default() -> Self {
		Group {
			id: Uuid::new_v4(),
			name: Arc::new("Group".into()),
			position: Arc::new(Vec2::new(0, 0)),
			children: Arc::new(vec![]),
		}
	}
}

impl Node for Group {
	fn id(&self) -> &Uuid {
		&self.id
	}
	fn name(&self) -> &str {
		&self.name
	}
}

#[allow(unreachable_patterns)]
impl NonLeafNode for Group {
	fn is_child_valid(&self, node: &NodeType) -> bool {
		match node {
			NodeType::Group(_) | NodeType::Note(_) => true,
			_ => false,
		}
	}
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
		assert_eq!(*group.name, "Group");
		assert_eq!(*group.position, Vec2::new(0, 0));
		assert_eq!(group.children.len(), 0);
	}
}

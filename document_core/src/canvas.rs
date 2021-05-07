use crate::{Node, NodeType, NonLeafNode};
use color::Channel;
use serde::{Deserialize, Serialize};
use std::sync::Arc;
use uuid::Uuid;
use vek::vec::repr_c::vec2::Vec2;

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct CanvasGroup {
	pub id: Uuid,
	pub name: Arc<String>,
	pub position: Arc<Vec2<i32>>,
	pub channel: Channel,
	pub children: Arc<Vec<Arc<NodeType>>>,
}

impl Default for CanvasGroup {
	fn default() -> Self {
		CanvasGroup {
			id: Uuid::new_v4(),
			name: Arc::new("CanvasGroup".into()),
			position: Arc::new(Vec2::new(0, 0)),
			channel: Channel::default(),
			children: Arc::new(vec![]),
		}
	}
}

impl Node for CanvasGroup {
	fn id(&self) -> &Uuid {
		&self.id
	}
	fn name(&self) -> &str {
		&self.name
	}
}

#[allow(unreachable_patterns)]
impl NonLeafNode for CanvasGroup {
	fn is_child_valid(&self, node: &NodeType) -> bool {
		match node {
			NodeType::CanvasGroup(_) => true,
			_ => false,
		}
	}
	fn children(&self) -> &Arc<Vec<Arc<NodeType>>> {
		&self.children
	}
}

#[cfg(test)]
mod tests {
	use super::CanvasGroup;
	use vek::vec::repr_c::vec2::Vec2;

	#[test]
	fn impl_default() {
		let group = CanvasGroup::default();
		assert_eq!(*group.name, "CanvasGroup");
		assert_eq!(*group.position, Vec2::new(0, 0));
		assert_eq!(group.children.len(), 0);
	}
}

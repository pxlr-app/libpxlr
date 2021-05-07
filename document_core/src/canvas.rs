use crate::{HasBounds, HasChannel, HasChildren, Node, NodeType};
use color::Channel;
use serde::{Deserialize, Serialize};
use std::sync::Arc;
use uuid::Uuid;
use vek::{geom::repr_c::Rect, vec::repr_c::vec2::Vec2};

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct CanvasGroup {
	id: Uuid,
	name: String,
	position: Arc<Vec2<i32>>,
	channel: Channel,
	children: Vec<Arc<NodeType>>,
}

impl CanvasGroup {
	pub unsafe fn construct(
		id: Uuid,
		name: String,
		position: Vec2<i32>,
		channel: Channel,
		children: Vec<Arc<NodeType>>,
	) -> Self {
		CanvasGroup {
			id: id,
			name: name,
			position: Arc::new(position),
			channel: channel,
			children: children,
		}
	}
	pub fn new<
		S: Into<String>,
		V: Into<Vec2<i32>>,
		C: Into<Channel>,
		L: Into<Vec<Arc<NodeType>>>,
	>(
		name: S,
		position: V,
		channel: C,
		children: L,
	) -> Self {
		unsafe {
			CanvasGroup::construct(
				Uuid::new_v4(),
				name.into(),
				position.into(),
				channel.into(),
				children.into(),
			)
		}
	}
}

impl Default for CanvasGroup {
	fn default() -> Self {
		CanvasGroup::new("CanvasGroup", Vec2::new(0, 0), Channel::default(), vec![])
	}
}

impl Node for CanvasGroup {
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

impl HasBounds for CanvasGroup {
	fn bounds(&self) -> Rect<i32, i32> {
		// TODO extend children bounds
		Rect::new(self.position.x, self.position.y, 0, 0)
	}
	fn set_position(&mut self, position: Vec2<i32>) {
		self.position = Arc::new(position);
	}
}

impl HasChildren for CanvasGroup {
	fn is_child_valid(&self, node: &NodeType) -> bool {
		match node {
			NodeType::CanvasGroup(_) => true,
			_ => false,
		}
	}
	fn children(&self) -> &Vec<Arc<NodeType>> {
		&self.children
	}
	fn set_children(&mut self, children: Vec<Arc<NodeType>>) {
		self.children = children;
	}
}

impl HasChannel for CanvasGroup {
	fn channel(&self) -> Channel {
		self.channel
	}
	fn set_channel(&mut self, channel: Channel) {
		self.channel = channel;
	}
}

#[cfg(test)]
mod tests {
	use super::*;

	#[test]
	fn impl_default() {
		let group = CanvasGroup::default();
		assert_eq!(group.name(), "CanvasGroup");
		assert_eq!(group.bounds().into_aabr().min, Vec2::new(0, 0));
		assert_eq!(group.children().len(), 0);
	}
}

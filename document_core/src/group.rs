use crate::{HasBounds, HasChildren, Node, NodeType};
use serde::{Deserialize, Serialize};
use std::{convert::TryInto, sync::Arc};
use uuid::Uuid;
use vek::{geom::repr_c::Rect, vec::repr_c::vec2::Vec2};

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct Group {
	id: Uuid,
	name: String,
	position: Arc<Vec2<i32>>,
	children: Vec<Arc<NodeType>>,
}

impl Group {
	pub unsafe fn construct(
		id: Uuid,
		name: String,
		position: Vec2<i32>,
		children: Vec<Arc<NodeType>>,
	) -> Self {
		Group {
			id: id,
			name: name,
			position: Arc::new(position),
			children: children,
		}
	}
	pub fn new<S: Into<String>, V: Into<Vec2<i32>>, C: Into<Vec<Arc<NodeType>>>>(
		name: S,
		position: V,
		children: C,
	) -> Self {
		unsafe {
			Group::construct(
				Uuid::new_v4(),
				name.into(),
				position.into(),
				children.into(),
			)
		}
	}
}

impl Default for Group {
	fn default() -> Self {
		Group::new("Group", Vec2::new(0, 0), vec![])
	}
}

impl Node for Group {
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

impl HasBounds for Group {
	fn bounds(&self) -> Rect<i32, i32> {
		let mut bounds: Option<Rect<i32, i32>> = None;
		for child in self.children.iter() {
			if let Ok(child_bounds) = TryInto::<&dyn HasBounds>::try_into(&**child) {
				let new_bounds = bounds.take().map_or_else(
					|| child_bounds.bounds(),
					|prev_bounds| prev_bounds.union(child_bounds.bounds()),
				);
				bounds.replace(new_bounds);
			};
		}
		bounds.take().map_or_else(
			|| Rect::new(self.position.x, self.position.y, 0, 0),
			|bounds| {
				Rect::new(
					self.position.x + bounds.x,
					self.position.y + bounds.y,
					bounds.w,
					bounds.h,
				)
			},
		)
	}
	fn set_position(&mut self, position: Vec2<i32>) {
		self.position = Arc::new(position);
	}
}

impl HasChildren for Group {
	fn is_child_valid(&self, node: &NodeType) -> bool {
		match node {
			NodeType::Group(_) | NodeType::Note(_) => true,
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

#[cfg(test)]
mod tests {
	use super::*;

	#[test]
	fn impl_default() {
		let group = Group::default();
		assert_eq!(group.name(), "Group");
		assert_eq!(group.bounds().into_aabr().min, Vec2::new(0, 0));
		assert_eq!(group.children().len(), 0);
	}
}

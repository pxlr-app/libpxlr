use std::rc::Rc;
use uuid::Uuid;
use math::{Vec2, Extent2};

use crate::document::*;
use crate::node::*;
use crate::patch::*;
use crate::sprite::{Layer, ILayer};

pub struct Group {
	pub id: Uuid,
	pub name: Rc<String>,
	pub children: Rc<Vec<Rc<Layer>>>,
	pub position: Rc<Vec2<f32>>,
	pub size: Rc<Extent2<u16>>,
}

impl Group {
	pub fn new(id: Option<Uuid>, name: &str, children: Vec<Rc<Layer>>, position: Vec2<f32>, size: Extent2<u16>) -> Group {
		Group {
			id: id.or(Some(Uuid::new_v4())).unwrap(),
			name: Rc::new(name.to_owned()),
			children: Rc::new(children),
			position: Rc::new(position),
			size: Rc::new(size),
		}
	}
}

impl INode for Group {
	fn id(&self) -> Uuid {
		self.id
	}
	fn display(&self) -> String {
		self.name.to_string()
	}
}

macro_rules! patch_children {
	($children:expr, $mutated:expr, $patch:expr) => {{
		$children.iter().map(|child| {
			macro_rules! patch_layer {
				($x:expr) => {{
					if let Some(doc) = ILayer::patch($x, $patch) {
						$mutated = true;
						Rc::new(doc)
					} else {
						child.clone()
					}
				}};
			}
			match &**child {
				Layer::Group(group) => patch_layer!(group),
				_ => child.clone()
			}
		}).collect::<Vec<_>>()
	}}
}

macro_rules! patch_group {
	($group:expr, $enum:expr, $patch:expr) => {{
		if $patch.target == $group.id {
			match &$patch.payload {
				PatchAction::Rename(new_name) => Some($enum(Group {
					id: $group.id,
					name: Rc::new(new_name.to_string()),
					children: Rc::clone(&$group.children),
					position: Rc::clone(&$group.position),
					size: Rc::clone(&$group.size),
				})),
				_ => None,
			}
		} else {
			let mut mutated = false;
			let children = patch_children!($group.children, mutated, $patch);
			
			if mutated {
				Some($enum(Group {
					id: $group.id,
					name: Rc::clone(&$group.name),
					children: Rc::new(children),
					position: Rc::clone(&$group.position),
					size: Rc::clone(&$group.size),
				}))
			} else {
				None
			}
		}
	}}
}

impl ILayer for Group {
	fn patch(&self, patch: &Patch) -> Option<Layer> {
		patch_group!(self, Layer::Group, patch)
	}
}

impl IDocument for Group {
	fn position(&self) -> Vec2<f32> {
		*(self.position).clone()
	}
	fn patch(&self, patch: &Patch) -> Option<Document> {
		patch_group!(self, Document::Sprite, patch)
	}
}
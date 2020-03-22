use std::rc::Rc;
use uuid::Uuid;
use math::{Vec2};

use crate::document::*;
use crate::node::*;
use crate::patch::*;
use crate::sprite::{Layer, ILayer};

pub struct Group {
	pub id: Uuid,
	pub name: Rc<String>,
	pub children: Rc<Vec<Rc<Layer>>>,
	pub position: Rc<Vec2>,
}

impl Group {
	pub fn new(id: Option<Uuid>, name: &str, position: Vec2, children: Vec<Rc<Layer>>) -> Group {
		Group {
			id: id.or(Some(Uuid::new_v4())).unwrap(),
			name: Rc::new(name.to_owned()),
			position: Rc::new(position),
			children: Rc::new(children)
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

impl ILayer for Group {
	fn patch(&self, patch: &Patch) -> Option<Layer> {
		if patch.target == self.id {
			match &patch.payload {
				PatchAction::Rename(new_name) => Some(Layer::Group(Group {
					id: self.id,
					name: Rc::new(new_name.to_string()),
					children: Rc::clone(&self.children),
					position: Rc::clone(&self.position),
				})),
				_ => None,
			}
		} else {
			let mut mutated = false;
			let children = self.children.iter().map(|child| {
				macro_rules! patch_layer {
					($x:expr) => {{
						if let Some(doc) = ILayer::patch($x, patch) {
							mutated = true;
							Rc::new(doc)
						} else {
							child.clone()
						}
					}};
				}
				match &**child {
					Layer::Group(group) => patch_layer!(group),
				}
			}).collect::<Vec<_>>();
			
			if mutated {
				Some(Layer::Group(Group {
					id: self.id,
					name: Rc::clone(&self.name),
					children: Rc::new(children),
					position: Rc::clone(&self.position),
				}))
			} else {
				None
			}
		}
	}
}

impl IDocument for Group {
	fn position(&self) -> Vec2 {
		*(self.position).clone()
	}
	fn patch(&self, patch: &Patch) -> Option<Document> {
		if patch.target == self.id {
			match &patch.payload {
				PatchAction::Rename(new_name) => Some(Document::Sprite(Group {
					id: self.id,
					name: Rc::new(new_name.to_string()),
					children: Rc::clone(&self.children),
					position: Rc::clone(&self.position),
				})),
				_ => None,
			}
		} else {
			let mut mutated = false;
			let children = self.children.iter().map(|child| {
				macro_rules! patch_doc {
					($x:expr) => {{
						if let Some(doc) = ILayer::patch($x, patch) {
							mutated = true;
							Rc::new(doc)
						} else {
							child.clone()
						}
					}};
				}
				match &**child {
					Layer::Group(group) => patch_doc!(group),
				}
			}).collect::<Vec<_>>();
			
			if mutated {
				Some(Document::Sprite(Group {
					id: self.id,
					name: Rc::clone(&self.name),
					children: Rc::new(children),
					position: Rc::clone(&self.position),
				}))
			} else {
				None
			}
		}
	}
}
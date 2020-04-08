use std::any::Any;
use std::rc::Rc;

use math::{Extent2, Vec2};
use uuid::Uuid;

use crate::document::Document;
use crate::node::*;
use crate::patch::*;
use crate::sprite::Layer;

pub trait GroupLayer: Layer {
	fn patch_rc(&self, patch: &dyn PatchImpl) -> Option<Rc<dyn GroupLayer + 'static>>;
	fn as_any(&self) -> &dyn Any;
}
impl<T> GroupLayer for T
where
	T: Patchable + Layer + Any,
{
	fn patch_rc(&self, patch: &dyn PatchImpl) -> Option<Rc<dyn GroupLayer + 'static>> {
		match self.patch(patch) {
			Some(new_self) => Some(Rc::new(*new_self)),
			None => None,
		}
	}
	fn as_any(&self) -> &dyn Any {
		self
	}
}

pub struct Group {
	pub id: Uuid,
	pub name: Rc<String>,
	pub children: Rc<Vec<Rc<dyn GroupLayer>>>,
	pub position: Rc<Vec2<f32>>,
	pub size: Rc<Extent2<u32>>,
}

impl Group {
	pub fn new(
		id: Option<Uuid>,
		name: &str,
		children: Vec<Rc<dyn GroupLayer>>,
		position: Vec2<f32>,
		size: Extent2<u32>,
	) -> Group {
		Group {
			id: id.or(Some(Uuid::new_v4())).unwrap(),
			name: Rc::new(name.to_owned()),
			children: Rc::new(children),
			position: Rc::new(position),
			size: Rc::new(size),
		}
	}
}

impl Node for Group {
	fn id(&self) -> Uuid {
		self.id
	}
}

impl Layer for Group {
	fn crop(&self, offset: Vec2<u32>, size: Extent2<u32>) -> (CropPatch, Box<dyn PatchImpl>) {
		(
			CropPatch {
				target: self.id,
				offset: offset,
				size: size,
			},
			Box::new(RestoreGroupPatch {
				target: self.id,
				name: (*self.name).to_owned(),
				position: (*self.position).clone(),
				size: (*self.size).clone(),
				children: self
					.children
					.iter()
					.map(|child| child.crop(offset, size).1)
					.collect::<Vec<_>>(),
			}),
		)
	}
}

impl Document for Group {
	fn position(&self) -> Vec2<f32> {
		*(self.position).clone()
	}
}

impl<'a> Renamable<'a> for Group {
	fn rename(&self, new_name: &'a str) -> (RenamePatch, RenamePatch) {
		(
			RenamePatch {
				target: self.id,
				name: new_name.to_owned(),
			},
			RenamePatch {
				target: self.id,
				name: (*self.name).to_owned(),
			},
		)
	}
}

pub struct AddLayerPatch {
	pub target: Uuid,
	pub child: Rc<dyn GroupLayer>,
	pub position: usize,
}

impl Patch for AddLayerPatch {
	fn target(&self) -> Uuid {
		self.target
	}
}

pub struct RemoveLayerPatch {
	pub target: Uuid,
	pub child_id: Uuid,
}

impl Patch for RemoveLayerPatch {
	fn target(&self) -> Uuid {
		self.target
	}
}

pub struct MoveLayerPatch {
	pub target: Uuid,
	pub child_id: Uuid,
	pub position: usize,
}

impl Patch for MoveLayerPatch {
	fn target(&self) -> Uuid {
		self.target
	}
}

pub struct RestoreGroupPatch {
	pub target: Uuid,
	pub name: String,
	pub position: Vec2<f32>,
	pub size: Extent2<u32>,
	pub children: Vec<Box<dyn PatchImpl>>,
}

impl Patch for RestoreGroupPatch {
	fn target(&self) -> Uuid {
		self.target
	}
}

impl Patchable for Group {
	fn patch(&self, patch: &dyn PatchImpl) -> Option<Box<Self>> {
		if patch.target() == self.id {
			if let Some(rename) = patch.as_any().downcast_ref::<RenamePatch>() {
				return Some(Box::new(Group {
					id: self.id,
					name: Rc::new(rename.name.clone()),
					position: self.position.clone(),
					size: self.size.clone(),
					children: self.children.clone(),
				}));
			} else if let Some(patch) = patch.as_any().downcast_ref::<AddLayerPatch>() {
				let mut children = self
					.children
					.iter()
					.map(|child| child.clone())
					.collect::<Vec<_>>();
				if patch.position > children.len() {
					children.push(patch.child.clone());
				} else {
					children.insert(patch.position, patch.child.clone());
				}
				return Some(Box::new(Group {
					id: self.id,
					name: self.name.clone(),
					position: self.position.clone(),
					size: self.size.clone(),
					children: Rc::new(children),
				}));
			} else if let Some(patch) = patch.as_any().downcast_ref::<RemoveLayerPatch>() {
				let children = self
					.children
					.iter()
					.filter_map(|child| {
						if child.id() == patch.child_id {
							None
						} else {
							Some(child.clone())
						}
					})
					.collect::<Vec<_>>();
				return Some(Box::new(Group {
					id: self.id,
					name: self.name.clone(),
					position: self.position.clone(),
					size: self.size.clone(),
					children: Rc::new(children),
				}));
			} else if let Some(patch) = patch.as_any().downcast_ref::<MoveLayerPatch>() {
				let mut children = self
					.children
					.iter()
					.map(|child| child.clone())
					.collect::<Vec<_>>();
				let index = children
					.iter()
					.position(|child| child.id() == patch.child_id)
					.unwrap();
				let child = children.remove(index);
				if patch.position > children.len() {
					children.push(child);
				} else {
					children.insert(patch.position, child);
				}
				return Some(Box::new(Group {
					id: self.id,
					name: self.name.clone(),
					position: self.position.clone(),
					size: self.size.clone(),
					children: Rc::new(children),
				}));
			}
		} else {
			let mut mutated = false;
			let children = self
				.children
				.iter()
				.map(|child| match child.patch_rc(patch) {
					Some(new_child) => {
						mutated = true;
						new_child
					}
					None => child.clone(),
				})
				.collect::<Vec<_>>();
			if mutated {
				return Some(Box::new(Group {
					id: self.id,
					name: Rc::clone(&self.name),
					position: Rc::clone(&self.position),
					size: Rc::clone(&self.size),
					children: Rc::new(children),
				}));
			}
		}
		return None;
	}
}

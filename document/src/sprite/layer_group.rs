use math::{Extent2, Vec2};
use std::rc::Rc;
use uuid::Uuid;

use crate::document::Document;
use crate::node::*;
use crate::patch::*;
use crate::sprite::*;

pub struct LayerGroup {
	pub id: Uuid,
	pub name: Rc<String>,
	pub children: Rc<Vec<Rc<LayerNode>>>,
	pub position: Rc<Vec2<f32>>,
	pub size: Rc<Extent2<u32>>,
}

#[derive(Debug)]
pub enum LayerGroupError {
	LayerFound,
	LayerNotFound,
}

impl std::fmt::Display for LayerGroupError {
	fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
		match *self {
			LayerGroupError::LayerFound => write!(f, "Child already exists in this group."),
			LayerGroupError::LayerNotFound => write!(f, "Child not found in this group."),
		}
	}
}

impl std::error::Error for LayerGroupError {
	fn cause(&self) -> Option<&dyn std::error::Error> {
		None
	}
}

impl LayerGroup {
	pub fn new(
		id: Option<Uuid>,
		name: &str,
		children: Vec<Rc<LayerNode>>,
		position: Vec2<f32>,
		size: Extent2<u32>,
	) -> LayerGroup {
		LayerGroup {
			id: id.or(Some(Uuid::new_v4())).unwrap(),
			name: Rc::new(name.to_owned()),
			children: Rc::new(children),
			position: Rc::new(position),
			size: Rc::new(size),
		}
	}

	pub fn add_child(
		&self,
		add_child: Rc<LayerNode>,
	) -> Result<(AddLayerPatch, RemoveLayerPatch), LayerGroupError> {
		let index = self
			.children
			.iter()
			.position(|child| Rc::ptr_eq(&child, &add_child));
		if index.is_some() {
			Err(LayerGroupError::LayerFound)
		} else {
			Ok((
				AddLayerPatch {
					target: self.id,
					child: add_child.clone(),
					position: self.children.len(),
				},
				RemoveLayerPatch {
					target: self.id,
					child_id: add_child.id(),
				},
			))
		}
	}

	pub fn remove_child(
		&self,
		child_id: Uuid,
	) -> Result<(RemoveLayerPatch, AddLayerPatch), LayerGroupError> {
		let index = self
			.children
			.iter()
			.position(|child| child.id() == child_id);
		if index.is_none() {
			Err(LayerGroupError::LayerNotFound)
		} else {
			let index = index.unwrap();
			Ok((
				RemoveLayerPatch {
					target: self.id,
					child_id: child_id,
				},
				AddLayerPatch {
					target: self.id,
					child: self.children.get(index).unwrap().clone(),
					position: index,
				},
			))
		}
	}

	pub fn move_child(
		&self,
		child_id: Uuid,
		position: usize,
	) -> Result<(MoveLayerPatch, MoveLayerPatch), LayerGroupError> {
		let index = self
			.children
			.iter()
			.position(|child| child.id() == child_id);
		if index.is_none() {
			Err(LayerGroupError::LayerNotFound)
		} else {
			let index = index.unwrap();
			Ok((
				MoveLayerPatch {
					target: self.id,
					child_id: child_id,
					position: position,
				},
				MoveLayerPatch {
					target: self.id,
					child_id: child_id,
					position: index,
				},
			))
		}
	}
}

impl Node for LayerGroup {
	fn id(&self) -> Uuid {
		self.id
	}
}

impl Layer for LayerGroup {
	fn crop(&self, offset: Vec2<u32>, size: Extent2<u32>) -> (Patch, Patch) {
		(
			Patch::CropLayer(CropLayerPatch {
				target: self.id,
				offset: offset,
				size: size,
			}),
			Patch::RestoreLayerGroup(RestoreLayerGroupPatch {
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

	fn resize(
		&self,
		size: Extent2<u32>,
		interpolation: Interpolation,
	) -> (Patch, Patch) {
		(
			Patch::ResizeLayer(ResizeLayerPatch {
				target: self.id,
				size: size,
				interpolation: interpolation,
			}),
			Patch::RestoreLayerGroup(RestoreLayerGroupPatch {
				target: self.id,
				name: (*self.name).to_owned(),
				position: (*self.position).clone(),
				size: (*self.size).clone(),
				children: self
					.children
					.iter()
					.map(|child| child.resize(size, interpolation).1)
					.collect::<Vec<_>>(),
			}),
		)
	}
}

impl Document for LayerGroup {
	fn position(&self) -> Vec2<f32> {
		*(self.position).clone()
	}
}

impl<'a> Renamable<'a> for LayerGroup {
	fn rename(&self, new_name: &'a str) -> (Patch, Patch) {
		(
			Patch::Rename(RenamePatch {
				target: self.id,
				name: new_name.to_owned(),
			}),
			Patch::Rename(RenamePatch {
				target: self.id,
				name: (*self.name).to_owned(),
			}),
		)
	}
}

impl Patchable for LayerGroup {
	fn patch(&self, patch: &Patch) -> Option<Box<Self>> {
		if patch.target() == self.id {
			return match patch {
				Patch::Rename(patch) => Some(Box::new(LayerGroup {
					id: self.id,
					name: Rc::new(patch.name.clone()),
					position: self.position.clone(),
					size: self.size.clone(),
					children: self.children.clone(),
				})),
				Patch::AddLayer(patch) => {
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
					Some(Box::new(LayerGroup {
						id: self.id,
						name: self.name.clone(),
						position: self.position.clone(),
						size: self.size.clone(),
						children: Rc::new(children),
					}))
				},
				Patch::RemoveLayer(patch) => {
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
					Some(Box::new(LayerGroup {
						id: self.id,
						name: self.name.clone(),
						position: self.position.clone(),
						size: self.size.clone(),
						children: Rc::new(children),
					}))
				},
				Patch::MoveLayer(patch) => {
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
					Some(Box::new(LayerGroup {
						id: self.id,
						name: self.name.clone(),
						position: self.position.clone(),
						size: self.size.clone(),
						children: Rc::new(children),
					}))
				},
				Patch::CropLayer(patch) => {
					let children = self
						.children
						.iter()
						.map(|child| {
							match child.patch(&Patch::CropLayer(CropLayerPatch {
								target: child.id(),
								..*patch
							})) {
								Some(new_child) => Rc::new(new_child),
								None => child.clone(),
							}
						})
						.collect::<Vec<_>>();
					Some(Box::new(LayerGroup {
						id: self.id,
						name: Rc::clone(&self.name),
						position: Rc::clone(&self.position),
						size: Rc::new(patch.size),
						children: Rc::new(children),
					}))
				},
				Patch::ResizeLayer(patch) => {
					let children = self
						.children
						.iter()
						.map(|child| {
							match child.patch(&Patch::ResizeLayer(ResizeLayerPatch {
								target: child.id(),
								..*patch
							})) {
								Some(new_child) => Rc::new(new_child),
								None => child.clone(),
							}
						})
						.collect::<Vec<_>>();
					Some(Box::new(LayerGroup {
						id: self.id,
						name: Rc::clone(&self.name),
						position: Rc::clone(&self.position),
						size: Rc::new(patch.size),
						children: Rc::new(children),
					}))
				},
				_ => None
			};
		} else {
			let mut mutated = false;
			let children = self
				.children
				.iter()
				.map(|child| match child.patch(patch) {
					Some(new_child) => {
						mutated = true;
						Rc::new(new_child)
					}
					None => child.clone(),
				})
				.collect::<Vec<_>>();
			if mutated {
				return Some(Box::new(LayerGroup {
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

#[cfg(test)]
mod tests {
	use super::*;
	use math::{Extent2, Vec2};
	use std::rc::Rc;

	#[test]
	fn it_adds_child() {
		let g1 = LayerGroup::new(
			None,
			"group",
			vec![],
			Vec2::new(0., 0.),
			Extent2::new(4u32, 4u32),
		);
		let c1 = Rc::new(Canvas::new(
			None,
			"canvas",
			Extent2::new(2u32, 2u32),
			vec![Pixel::I(255), Pixel::I(128), Pixel::I(64), Pixel::I(32)],
		));

		let (patch, _) = g1.add_child(c1.clone()).unwrap();
		let g2 = g1.patch(&patch).unwrap();

		assert_eq!(g2.children.len(), 1);
		assert_eq!(Rc::strong_count(&c1), 3);
	}

	#[test]
	fn it_removes_child() {
		let c1 = Rc::new(Canvas::new(
			None,
			"canvas",
			Extent2::new(2u32, 2u32),
			vec![255u8, 128u8, 64u8, 32u8],
		));
		let g1 = LayerGroup::new(
			None,
			"group",
			vec![c1.clone()],
			Vec2::new(0., 0.),
			Extent2::new(4u32, 4u32),
		);

		let (patch, _) = g1.remove_child(c1.id).unwrap();
		let g2 = g1.patch(&patch).unwrap();

		assert_eq!(g2.children.len(), 0);
		assert_eq!(Rc::strong_count(&c1), 2);
	}

	#[test]
	fn it_moves_child() {
		let c1 = Rc::new(Canvas::new(
			None,
			"canvas_a",
			Extent2::new(2u32, 2u32),
			vec![255u8, 128u8, 64u8, 32u8],
		));
		let c2 = Rc::new(Canvas::new(
			None,
			"canvas_b",
			Extent2::new(2u32, 2u32),
			vec![255u8, 128u8, 64u8, 32u8],
		));
		let g1 = LayerGroup::new(
			None,
			"group",
			vec![c1.clone(), c2.clone()],
			Vec2::new(0., 0.),
			Extent2::new(4u32, 4u32),
		);

		let (patch, _) = g1.move_child(c2.id, 0).unwrap();
		let g2 = g1.patch(&patch).unwrap();

		assert_eq!(g2.children.len(), 2);
		assert_eq!(g2.children.get(0).unwrap().id(), c2.id);
		assert_eq!(g2.children.get(1).unwrap().id(), c1.id);
	}

	#[test]
	fn it_patchs_child() {
		let c1 = Rc::new(Canvas::new(
			None,
			"canvas_a",
			Extent2::new(2u32, 2u32),
			vec![255u8, 128u8, 64u8, 32u8],
		));
		let c2 = Rc::new(Canvas::new(
			None,
			"canvas_b",
			Extent2::new(2u32, 2u32),
			vec![32u8, 64u8, 128u8, 255u8],
		));
		let g1 = LayerGroup::new(
			None,
			"group",
			vec![c1.clone(), c2.clone()],
			Vec2::new(0., 0.),
			Extent2::new(4u32, 4u32),
		);

		let (patch, _) = c1.rename("canvas_aa");
		let g2 = g1.patch(&patch).unwrap();

		assert_eq!(Rc::strong_count(&c1), 2);
		assert_eq!(Rc::strong_count(&c1.name), 1);
		assert_eq!(Rc::strong_count(&c2), 3);
		assert_eq!(Rc::strong_count(&c2.name), 1);
		assert_eq!(
			*g2.children
				.get(0)
				.unwrap()
				.as_any()
				.downcast_ref::<Canvas<u8>>()
				.unwrap()
				.name,
			"canvas_aa"
		);
		assert_eq!(
			*g2.children
				.get(1)
				.unwrap()
				.as_any()
				.downcast_ref::<Canvas<u8>>()
				.unwrap()
				.name,
			"canvas_b"
		);

		let (patch, _) = g1.resize(Extent2::new(4, 1), Interpolation::Nearest);
		let g2 = g1.patch(&patch).unwrap();

		assert_eq!(*g2.size, Extent2::new(4, 1));
		assert_eq!(
			*g2.children
				.get(0)
				.unwrap()
				.as_any()
				.downcast_ref::<Canvas<u8>>()
				.unwrap()
				.size,
			Extent2::new(4, 1)
		);
		assert_eq!(
			*g2.children
				.get(1)
				.unwrap()
				.as_any()
				.downcast_ref::<Canvas<u8>>()
				.unwrap()
				.size,
			Extent2::new(4, 1)
		);
		assert_eq!(
			*g2.children
				.get(0)
				.unwrap()
				.as_any()
				.downcast_ref::<Canvas<u8>>()
				.unwrap()
				.data,
			vec![255, 255, 64, 64]
		);
		assert_eq!(
			*g2.children
				.get(1)
				.unwrap()
				.as_any()
				.downcast_ref::<Canvas<u8>>()
				.unwrap()
				.data,
			vec![32, 32, 128, 128]
		);
	}
}
use crate::color::ColorMode;
use crate::patch::*;
use crate::sprite::*;
use crate::{IDocument, INode};
use math::interpolation::*;
use math::{Extent2, Vec2};
use serde::{Deserialize, Serialize};
use std::sync::Arc;
use uuid::Uuid;

#[derive(Debug, Serialize, Deserialize)]
pub struct LayerGroup {
	pub id: Uuid,
	pub is_visible: bool,
	pub is_locked: bool,
	pub is_folded: bool,
	pub name: Arc<String>,
	pub color_mode: ColorMode,
	pub children: Arc<Vec<Arc<LayerNode>>>,
	pub position: Arc<Vec2<f32>>,
	pub size: Arc<Extent2<u32>>,
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
		color_mode: ColorMode,
		children: Vec<Arc<LayerNode>>,
		position: Vec2<f32>,
		size: Extent2<u32>,
	) -> LayerGroup {
		LayerGroup {
			id: id.or(Some(Uuid::new_v4())).unwrap(),
			is_visible: true,
			is_locked: false,
			is_folded: false,
			name: Arc::new(name.to_owned()),
			color_mode: color_mode,
			children: Arc::new(children),
			position: Arc::new(position),
			size: Arc::new(size),
		}
	}

	pub fn add_layer(&self, add_layer: Arc<LayerNode>) -> Result<(Patch, Patch), LayerGroupError> {
		let index = self
			.children
			.iter()
			.position(|child| Arc::ptr_eq(&child, &add_layer));
		if index.is_some() {
			Err(LayerGroupError::LayerFound)
		} else {
			Ok((
				Patch::AddLayer(AddLayerPatch {
					target: self.id,
					child: add_layer.clone(),
					position: self.children.len(),
				}),
				Patch::RemoveLayer(RemoveLayerPatch {
					target: self.id,
					child_id: add_layer.id(),
				}),
			))
		}
	}

	pub fn remove_layer(&self, child_id: Uuid) -> Result<(Patch, Patch), LayerGroupError> {
		let index = self
			.children
			.iter()
			.position(|child| child.id() == child_id);
		if index.is_none() {
			Err(LayerGroupError::LayerNotFound)
		} else {
			let index = index.unwrap();
			Ok((
				Patch::RemoveLayer(RemoveLayerPatch {
					target: self.id,
					child_id: child_id,
				}),
				Patch::AddLayer(AddLayerPatch {
					target: self.id,
					child: self.children.get(index).unwrap().clone(),
					position: index,
				}),
			))
		}
	}

	pub fn move_layer(
		&self,
		child_id: Uuid,
		position: usize,
	) -> Result<(Patch, Patch), LayerGroupError> {
		let index = self
			.children
			.iter()
			.position(|child| child.id() == child_id);
		if index.is_none() {
			Err(LayerGroupError::LayerNotFound)
		} else {
			let index = index.unwrap();
			Ok((
				Patch::MoveLayer(MoveLayerPatch {
					target: self.id,
					child_id: child_id,
					position: position,
				}),
				Patch::MoveLayer(MoveLayerPatch {
					target: self.id,
					child_id: child_id,
					position: index,
				}),
			))
		}
	}
}

impl ILayer for LayerGroup {
	fn crop(
		&self,
		offset: Vec2<u32>,
		size: Extent2<u32>,
	) -> Result<(Patch, Patch), CropLayerError> {
		if size.w == 0 || size.h == 0 {
			Err(CropLayerError::InvalidSize)
		} else if size.w + offset.x > self.size.w || size.h + offset.y > self.size.h {
			Err(CropLayerError::OutsideRegion)
		} else {
			Ok((
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
						.map(|child| child.crop(offset, size).unwrap().1)
						.collect::<Vec<_>>(),
				}),
			))
		}
	}

	fn resize(
		&self,
		size: Extent2<u32>,
		interpolation: Interpolation,
	) -> Result<(Patch, Patch), ResizeLayerError> {
		if size.w == 0 || size.h == 0 {
			Err(ResizeLayerError::InvalidSize)
		} else {
			Ok((
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
						.map(|child| child.resize(size, interpolation).unwrap().1)
						.collect::<Vec<_>>(),
				}),
			))
		}
	}
}

impl IDocument for LayerGroup {
	fn position(&self) -> Vec2<f32> {
		*(self.position).clone()
	}
}

impl INode for LayerGroup {
	fn is_visible(&self) -> bool {
		self.is_visible
	}
	fn is_locked(&self) -> bool {
		self.is_locked
	}
	fn is_folded(&self) -> bool {
		self.is_folded
	}
}

impl<'a> Renamable<'a> for LayerGroup {
	fn rename(&self, new_name: &'a str) -> Result<(Patch, Patch), RenameError> {
		if *self.name == new_name {
			Err(RenameError::Unchanged)
		} else {
			Ok((
				Patch::Rename(RenamePatch {
					target: self.id,
					name: new_name.to_owned(),
				}),
				Patch::Rename(RenamePatch {
					target: self.id,
					name: (*self.name).to_owned(),
				}),
			))
		}
	}
}

impl IVisible for LayerGroup {
	fn set_visibility(&self, visible: bool) -> Result<(Patch, Patch), SetVisibilityError> {
		if self.is_visible == visible {
			Err(SetVisibilityError::Unchanged)
		} else {
			Ok((
				Patch::SetVisibility(SetVisibilityPatch {
					target: self.id,
					visibility: visible,
				}),
				Patch::SetVisibility(SetVisibilityPatch {
					target: self.id,
					visibility: self.is_visible,
				}),
			))
		}
	}
}

impl ILockable for LayerGroup {
	fn set_lock(&self, lock: bool) -> Result<(Patch, Patch), SetLockError> {
		if self.is_locked == lock {
			Err(SetLockError::Unchanged)
		} else {
			Ok((
				Patch::SetLock(SetLockPatch {
					target: self.id,
					lock: lock,
				}),
				Patch::SetLock(SetLockPatch {
					target: self.id,
					lock: self.is_locked,
				}),
			))
		}
	}
}

impl IFoldable for LayerGroup {
	fn set_fold(&self, folded: bool) -> Result<(Patch, Patch), SetFoldError> {
		if self.is_folded == folded {
			Err(SetFoldError::Unchanged)
		} else {
			Ok((
				Patch::SetFold(SetFoldPatch {
					target: self.id,
					folded: folded,
				}),
				Patch::SetFold(SetFoldPatch {
					target: self.id,
					folded: self.is_folded,
				}),
			))
		}
	}
}

impl IPatchable for LayerGroup {
	fn patch(&self, patch: &Patch) -> Option<Box<Self>> {
		if patch.target() == self.id {
			return match patch {
				Patch::Rename(patch) => Some(Box::new(LayerGroup {
					id: self.id,
					is_visible: self.is_visible,
					is_locked: self.is_locked,
					is_folded: self.is_folded,
					name: Arc::new(patch.name.clone()),
					color_mode: self.color_mode,
					position: self.position.clone(),
					size: self.size.clone(),
					children: self.children.clone(),
				})),
				Patch::SetVisibility(patch) => Some(Box::new(LayerGroup {
					id: self.id,
					is_visible: patch.visibility,
					is_locked: self.is_locked,
					is_folded: self.is_folded,
					name: self.name.clone(),
					color_mode: self.color_mode,
					position: self.position.clone(),
					size: self.size.clone(),
					children: self.children.clone(),
				})),
				Patch::SetLock(patch) => Some(Box::new(LayerGroup {
					id: self.id,
					is_visible: self.is_visible,
					is_locked: patch.lock,
					is_folded: self.is_folded,
					name: self.name.clone(),
					color_mode: self.color_mode,
					position: self.position.clone(),
					size: self.size.clone(),
					children: self.children.clone(),
				})),
				Patch::SetFold(patch) => Some(Box::new(LayerGroup {
					id: self.id,
					is_visible: self.is_visible,
					is_locked: self.is_locked,
					is_folded: patch.folded,
					name: self.name.clone(),
					color_mode: self.color_mode,
					position: self.position.clone(),
					size: self.size.clone(),
					children: self.children.clone(),
				})),
				Patch::AddLayer(patch) => {
					assert_eq!(patch.child.color_mode(), self.color_mode);
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
						is_visible: self.is_visible,
						is_locked: self.is_locked,
						is_folded: self.is_folded,
						name: self.name.clone(),
						color_mode: self.color_mode,
						position: self.position.clone(),
						size: self.size.clone(),
						children: Arc::new(children),
					}))
				}
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
						is_visible: self.is_visible,
						is_locked: self.is_locked,
						is_folded: self.is_folded,
						name: self.name.clone(),
						color_mode: self.color_mode,
						position: self.position.clone(),
						size: self.size.clone(),
						children: Arc::new(children),
					}))
				}
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
						is_visible: self.is_visible,
						is_locked: self.is_locked,
						is_folded: self.is_folded,
						name: self.name.clone(),
						color_mode: self.color_mode,
						position: self.position.clone(),
						size: self.size.clone(),
						children: Arc::new(children),
					}))
				}
				Patch::CropLayer(patch) => {
					let children = self
						.children
						.iter()
						.map(|child| {
							match child.patch(&Patch::CropLayer(CropLayerPatch {
								target: child.id(),
								..*patch
							})) {
								Some(new_child) => Arc::new(new_child),
								None => child.clone(),
							}
						})
						.collect::<Vec<_>>();
					Some(Box::new(LayerGroup {
						id: self.id,
						is_visible: self.is_visible,
						is_locked: self.is_locked,
						is_folded: self.is_folded,
						name: Arc::clone(&self.name),
						color_mode: self.color_mode,
						position: Arc::clone(&self.position),
						size: Arc::new(patch.size),
						children: Arc::new(children),
					}))
				}
				Patch::ResizeLayer(patch) => {
					let children = self
						.children
						.iter()
						.map(|child| {
							match child.patch(&Patch::ResizeLayer(ResizeLayerPatch {
								target: child.id(),
								..*patch
							})) {
								Some(new_child) => Arc::new(new_child),
								None => child.clone(),
							}
						})
						.collect::<Vec<_>>();
					Some(Box::new(LayerGroup {
						id: self.id,
						is_visible: self.is_visible,
						is_locked: self.is_locked,
						is_folded: self.is_folded,
						name: Arc::clone(&self.name),
						color_mode: self.color_mode,
						position: Arc::clone(&self.position),
						size: Arc::new(patch.size),
						children: Arc::new(children),
					}))
				}
				_ => None,
			};
		} else {
			let mut mutated = false;
			let children = self
				.children
				.iter()
				.map(|child| match child.patch(patch) {
					Some(new_child) => {
						mutated = true;
						Arc::new(new_child)
					}
					None => child.clone(),
				})
				.collect::<Vec<_>>();
			if mutated {
				return Some(Box::new(LayerGroup {
					id: self.id,
					is_visible: self.is_visible,
					is_locked: self.is_locked,
					is_folded: self.is_folded,
					name: Arc::clone(&self.name),
					color_mode: self.color_mode,
					position: Arc::clone(&self.position),
					size: Arc::clone(&self.size),
					children: Arc::new(children),
				}));
			}
		}
		return None;
	}
}

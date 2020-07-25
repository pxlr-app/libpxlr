use crate::{
	any::{Any, Downcast},
	node::NodeType,
};
use serde::{Deserialize, Serialize};
use std::fmt::Debug;
use uuid::Uuid;

mod canvas;
mod node;
mod palette;
mod sprite;
pub use canvas::*;
pub use node::*;
pub use palette::*;
pub use sprite::*;

pub trait Patch: Any + Debug {
	fn target(&self) -> Uuid;
}
impl Downcast for dyn Patch {}

pub trait Patchable {
	fn patch(&self, patch: &PatchType) -> Option<NodeType>;
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum PatchType {
	Translate(Translate),
	Resize(Resize),
	SetVisible(SetVisible),
	SetLock(SetLock),
	SetFold(SetFold),
	Rename(Rename),
	AddChild(AddChild),
	RemoveChild(RemoveChild),
	MoveChild(MoveChild),
	AddColor(AddColor),
	RemoveColor(RemoveColor),
	MoveColor(MoveColor),
	SetPalette(SetPalette),
	UnsetPalette(UnsetPalette),
	SetColorMode(SetColorMode),
	Crop(Crop),
	RestoreSprite(RestoreSprite),
	RestoreCanvas(RestoreCanvas),
}

impl PatchType {
	pub fn as_patch(&self) -> &dyn Patch {
		match self {
			PatchType::Translate(patch) => patch,
			PatchType::Resize(patch) => patch,
			PatchType::SetVisible(patch) => patch,
			PatchType::SetLock(patch) => patch,
			PatchType::SetFold(patch) => patch,
			PatchType::Rename(patch) => patch,
			PatchType::AddChild(patch) => patch,
			PatchType::RemoveChild(patch) => patch,
			PatchType::MoveChild(patch) => patch,
			PatchType::AddColor(patch) => patch,
			PatchType::RemoveColor(patch) => patch,
			PatchType::MoveColor(patch) => patch,
			PatchType::SetPalette(patch) => patch,
			PatchType::UnsetPalette(patch) => patch,
			PatchType::SetColorMode(patch) => patch,
			PatchType::Crop(patch) => patch,
			PatchType::RestoreSprite(patch) => patch,
			PatchType::RestoreCanvas(patch) => patch,
		}
	}
}

pub type PatchPair = (PatchType, PatchType);

#[cfg(test)]
mod tests {
	use crate::prelude::*;

	#[test]
	fn test_patch() {
		let note = Note {
			id: Uuid::new_v4(),
			position: Arc::new(Vec2::new(0, 0)),
			display: true,
			locked: false,
			name: Arc::new("Foo".into()),
		};
		let (patch, _) = note.rename("Bar".into()).unwrap();
		let node = note.patch(&patch).unwrap();
		let note2 = match node {
			NodeType::Note(n) => n,
			_ => panic!("Not a node"),
		};
		assert_eq!(*note.name, "Foo");
		assert_eq!(*note2.name, "Bar");

		let group = Group {
			id: Uuid::parse_str("fc2c9e3e-2cd7-4375-a6fe-49403cc9f82b").unwrap(),
			position: Arc::new(Vec2::new(0, 0)),
			display: true,
			locked: false,
			folded: false,
			name: Arc::new("Foo".into()),
			children: Arc::new(vec![Arc::new(NodeType::Note(Note {
				id: Uuid::parse_str("1c3deaf3-3c7f-444d-9e05-9ddbcc2b9391").unwrap(),
				position: Arc::new(Vec2::new(0, 0)),
				display: true,
				locked: false,
				name: Arc::new("Foo".into()),
			}))]),
		};
		let (patch, _) = group
			.children
			.get(0)
			.unwrap()
			.as_documentnode()
			.unwrap()
			.rename("Bar".into())
			.unwrap();
		let node = group.patch(&patch).unwrap();
		let group2 = match node {
			NodeType::Group(n) => n,
			_ => panic!("Not a Group"),
		};
		assert_eq!(
			group
				.children
				.get(0)
				.unwrap()
				.as_documentnode()
				.unwrap()
				.name(),
			"Foo"
		);
		assert_eq!(
			group2
				.children
				.get(0)
				.unwrap()
				.as_documentnode()
				.unwrap()
				.name(),
			"Bar"
		);
	}
}

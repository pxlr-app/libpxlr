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
pub use canvas::*;
pub use node::*;
pub use palette::*;

pub trait Command: Any + Debug {
	fn target(&self) -> Uuid;
}
impl Downcast for dyn Command {}

pub trait Executable {
	fn execute(&self, command: &CommandType) -> Option<NodeType>;
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum CommandType {
	Translate(TranslateCommand),
	Resize(ResizeCommand),
	SetVisible(SetVisibleCommand),
	SetLock(SetLockCommand),
	SetFold(SetFoldCommand),
	Rename(RenameCommand),
	AddChild(AddChildCommand),
	RemoveChild(RemoveChildCommand),
	MoveChild(MoveChildCommand),
	AddColor(AddColorCommand),
	RemoveColor(RemoveColorCommand),
	MoveColor(MoveColorCommand),
	SetPaletteNode(SetPaletteNodeCommand),
	SetOpacity(SetOpacityCommand),
	SetComponents(SetComponentsCommand),
	Crop(CropCommand),
	Flip(FlipCommand),
	RestoreSprite(RestoreCanvasGroupCommand),
	RestoreCanvas(RestoreCanvasCommand),
	ApplyStencil(ApplyStencilCommand),
}

impl CommandType {
	pub fn as_command(&self) -> &dyn Command {
		match self {
			CommandType::Translate(command) => command,
			CommandType::Resize(command) => command,
			CommandType::SetVisible(command) => command,
			CommandType::SetLock(command) => command,
			CommandType::SetFold(command) => command,
			CommandType::Rename(command) => command,
			CommandType::AddChild(command) => command,
			CommandType::RemoveChild(command) => command,
			CommandType::MoveChild(command) => command,
			CommandType::AddColor(command) => command,
			CommandType::RemoveColor(command) => command,
			CommandType::MoveColor(command) => command,
			CommandType::SetPaletteNode(command) => command,
			CommandType::SetOpacity(command) => command,
			CommandType::SetComponents(command) => command,
			CommandType::Crop(command) => command,
			CommandType::Flip(command) => command,
			CommandType::RestoreSprite(command) => command,
			CommandType::RestoreCanvas(command) => command,
			CommandType::ApplyStencil(command) => command,
		}
	}
}

pub type CommandPair = (CommandType, CommandType);

#[cfg(test)]
mod tests {
	use crate::prelude::*;

	#[test]
	fn test_execute() {
		let note = NoteNode {
			id: Uuid::new_v4(),
			position: Arc::new(Vec2::new(0, 0)),
			display: true,
			locked: false,
			name: Arc::new("Foo".into()),
		};
		let (command, _) = note.rename("Bar".into()).unwrap();
		let node = note.execute(&command).unwrap();
		let note2 = match node {
			NodeType::Note(n) => n,
			_ => panic!("Not a node"),
		};
		assert_eq!(*note.name, "Foo");
		assert_eq!(*note2.name, "Bar");

		let group = GroupNode {
			id: Uuid::parse_str("fc2c9e3e-2cd7-4375-a6fe-49403cc9f82b").unwrap(),
			position: Arc::new(Vec2::new(0, 0)),
			display: true,
			locked: false,
			folded: false,
			name: Arc::new("Foo".into()),
			children: Arc::new(vec![Arc::new(NodeType::Note(NoteNode {
				id: Uuid::parse_str("1c3deaf3-3c7f-444d-9e05-9ddbcc2b9391").unwrap(),
				position: Arc::new(Vec2::new(0, 0)),
				display: true,
				locked: false,
				name: Arc::new("Foo".into()),
			}))]),
		};
		let (command, _) = group
			.children
			.get(0)
			.unwrap()
			.as_documentnode()
			.unwrap()
			.rename("Bar".into())
			.unwrap();
		let node = group.execute(&command).unwrap();
		let group2 = match node {
			NodeType::Group(n) => n,
			_ => panic!("Not a GroupNode"),
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

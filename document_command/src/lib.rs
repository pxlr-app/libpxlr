use document_core::{Node, NodeType, NonLeafNode};
use serde::{Deserialize, Serialize};
use std::sync::Arc;
use uuid::Uuid;
mod group;
mod note;
mod palette;
mod rename;
mod translate;

pub static DOCUMENT_VERSION: &str = env!("CARGO_PKG_VERSION");
pub static RUSTC_VERSION: &str = env!("RUSTC_VERSION");

pub trait Command {
	fn target(&self) -> &Uuid;
	fn execute_impl(&self, _node: &NodeType) -> Option<NodeType>;
	fn execute(&self, node: &NodeType) -> Option<NodeType> {
		if node.id() == self.target() {
			self.execute_impl(node)
		} else {
			fn execute_children<'a, T: NonLeafNode>(
				node: &'a T,
				execute: impl Fn(&'a NodeType) -> Option<NodeType>,
				fork: impl Fn(Arc<Vec<Arc<NodeType>>>) -> NodeType,
			) -> Option<NodeType> {
				let mut mutated = false;
				let children: Arc<Vec<Arc<NodeType>>> = Arc::new(
					node.children()
						.iter()
						.map(|child| match execute(child) {
							Some(child) => {
								mutated = true;
								Arc::new(child)
							}
							None => child.clone(),
						})
						.collect(),
				);
				if mutated {
					Some(fork(children))
				} else {
					None
				}
			}
			match node {
				NodeType::Group(node) => execute_children(
					node,
					|child| self.execute(child),
					|children| {
						let mut cloned = node.clone();
						cloned.children = children;
						NodeType::Group(cloned)
					},
				),
				_ => None,
			}
		}
	}
}

pub use group::*;
pub use note::*;
pub use palette::*;
pub use rename::*;
pub use translate::*;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum CommandType {
	AddChild(group::AddChildCommand),
	MoveChild(group::MoveChildCommand),
	RemoveChild(group::RemoveChildCommand),
	Rename(rename::RenameCommand),
	SetNoteContent(note::SetNoteContentCommand),
	Translate(translate::TranslateCommand),
	AddPaletteColor(palette::AddPaletteColorCommand),
	MovePaletteColor(palette::MovePaletteColorCommand),
	RemovePaletteColor(palette::RemovePaletteColorCommand),
}

impl Command for CommandType {
	fn target(&self) -> &Uuid {
		match self {
			CommandType::AddChild(cmd) => cmd.target(),
			CommandType::MoveChild(cmd) => cmd.target(),
			CommandType::RemoveChild(cmd) => cmd.target(),
			CommandType::Rename(cmd) => cmd.target(),
			CommandType::SetNoteContent(cmd) => cmd.target(),
			CommandType::Translate(cmd) => cmd.target(),
			CommandType::AddPaletteColor(cmd) => cmd.target(),
			CommandType::MovePaletteColor(cmd) => cmd.target(),
			CommandType::RemovePaletteColor(cmd) => cmd.target(),
		}
	}
	fn execute_impl(&self, node: &NodeType) -> Option<NodeType> {
		match self {
			CommandType::AddChild(cmd) => cmd.execute(node),
			CommandType::MoveChild(cmd) => cmd.execute(node),
			CommandType::RemoveChild(cmd) => cmd.execute(node),
			CommandType::Rename(cmd) => cmd.execute(node),
			CommandType::SetNoteContent(cmd) => cmd.execute(node),
			CommandType::Translate(cmd) => cmd.execute(node),
			CommandType::AddPaletteColor(cmd) => cmd.execute(node),
			CommandType::MovePaletteColor(cmd) => cmd.execute(node),
			CommandType::RemovePaletteColor(cmd) => cmd.execute(node),
		}
	}
}

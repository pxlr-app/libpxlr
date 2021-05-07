use serde::{Deserialize, Serialize};
use uuid::Uuid;
mod canvas;
mod group;
mod note;
mod palette;
mod traits;
mod walk;

pub use self::canvas::*;
pub use self::group::*;
pub use self::note::*;
pub use self::palette::*;
pub use self::traits::*;
pub use self::walk::*;

pub static DOCUMENT_VERSION: &str = env!("CARGO_PKG_VERSION");
pub static RUSTC_VERSION: &str = env!("RUSTC_VERSION");

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub enum NodeType {
	Note(note::Note),
	Group(group::Group),
	Palette(palette::Palette),
	CanvasGroup(canvas::CanvasGroup),
}

impl Node for NodeType {
	fn id(&self) -> &Uuid {
		match self {
			NodeType::Note(node) => node.id(),
			NodeType::Group(node) => node.id(),
			NodeType::Palette(node) => node.id(),
			NodeType::CanvasGroup(node) => node.id(),
		}
	}
	fn set_id(&mut self, id: Uuid) {
		match self {
			NodeType::Note(node) => node.set_id(id),
			NodeType::Group(node) => node.set_id(id),
			NodeType::Palette(node) => node.set_id(id),
			NodeType::CanvasGroup(node) => node.set_id(id),
		}
	}
	fn name(&self) -> &str {
		match self {
			NodeType::Note(node) => node.name(),
			NodeType::Group(node) => node.name(),
			NodeType::Palette(node) => node.name(),
			NodeType::CanvasGroup(node) => node.name(),
		}
	}
	fn set_name(&mut self, name: String) {
		match self {
			NodeType::Note(node) => node.set_name(name),
			NodeType::Group(node) => node.set_name(name),
			NodeType::Palette(node) => node.set_name(name),
			NodeType::CanvasGroup(node) => node.set_name(name),
		}
	}
}

impl<'a> std::convert::TryFrom<&'a NodeType> for &'a dyn HasChildren {
	type Error = ();

	fn try_from(value: &'a NodeType) -> Result<&'a dyn HasChildren, Self::Error> {
		match value {
			NodeType::Group(ref node) => Ok(node),
			NodeType::CanvasGroup(ref node) => Ok(node),
			_ => Err(()),
		}
	}
}

impl<'a> std::convert::TryFrom<&'a NodeType> for &'a dyn HasBounds {
	type Error = ();

	fn try_from(value: &'a NodeType) -> Result<&'a dyn HasBounds, Self::Error> {
		match value {
			NodeType::Note(ref node) => Ok(node),
			NodeType::Group(ref node) => Ok(node),
			NodeType::CanvasGroup(ref node) => Ok(node),
			_ => Err(()),
		}
	}
}

mod lib {
	#[cfg(test)]
	mod tests {
		use crate::{Group, HasChildren, NodeType};
		use std::convert::TryInto;

		#[test]
		fn try_into_nonleafnode() {
			let node = NodeType::Group(Group::default());
			let result: Result<&dyn HasChildren, ()> = (&node).try_into();
			assert!(result.is_ok());
		}
	}
}

pub use document_derive::*;
pub mod any;
pub mod color;
pub mod node;
pub mod parser;
pub mod patch;

pub mod prelude {
	pub use super::{any::*, color::*, node::*, parser, patch};
	pub use document_derive::*;
	pub use math::{Extent2, Vec2};
	pub use serde::{Deserialize, Serialize};
	pub use std::rc::Rc;
	pub use uuid::Uuid;
}

#[doc(hidden)]
pub use typetag;

#[doc(hidden)]
pub use lazy_static;

#[cfg(test)]
mod tests {
	use super::prelude::*;
	use crate as document;

	#[derive(Debug, DocumentNode, Serialize, Deserialize)]
	struct Empty {
		pub id: Uuid,
		pub name: String,
	}

	impl Name for Empty {
		fn name(&self) -> String {
			self.name.clone()
		}
		fn rename(&self, name: String) -> Option<(patch::Rename, patch::Rename)> {
			Some((
				patch::Rename {
					target: self.id,
					name,
				},
				patch::Rename {
					target: self.id,
					name: self.name.clone(),
				},
			))
		}
	}
	impl Position for Empty {}
	impl Size for Empty {}
	impl Visible for Empty {}
	impl Locked for Empty {}
	impl Folded for Empty {}

	impl Patchable for Empty {
		fn patch(&mut self, patch: &dyn Patch) {
			if let Some(patch) = patch.downcast::<patch::Rename>() {
				self.name = patch.name.clone();
			}
		}
	}

	#[test]
	fn test_cast() {
		let empty = Empty {
			id: Uuid::new_v4(),
			name: "Foo".into(),
		};
		let node: &dyn Node = &empty;
		let documentnode = node.as_documentnode();
		assert_eq!(documentnode.is_some(), true);
		let note2 = documentnode.unwrap().downcast::<Empty>();
		assert_eq!(note2.is_some(), true);
	}

	#[test]
	fn test_patch() {
		let mut empty = Empty {
			id: Uuid::new_v4(),
			name: "Foo".into(),
		};
		let (patch, _) = empty.rename("Bar".into()).unwrap();
		let node: &mut dyn Node = &mut empty;
		node.patch(&patch);
		assert_eq!(empty.name, "Bar");

		let mut group = Group {
			id: Uuid::parse_str("fc2c9e3e-2cd7-4375-a6fe-49403cc9f82b").unwrap(),
			position: Vec2::new(0, 0),
			visible: true,
			locked: false,
			folded: false,
			name: "Foo".into(),
			items: vec![<dyn Node>::from(Box::new(Empty {
				id: Uuid::parse_str("1c3deaf3-3c7f-444d-9e05-9ddbcc2b9391").unwrap(),
				name: "Foo".into(),
			}))],
		};
		let (patch, _) = group
			.items
			.get(0)
			.unwrap()
			.borrow()
			.as_documentnode()
			.unwrap()
			.rename("Bar".into())
			.unwrap();
		group.patch(&patch);
		assert_eq!(
			group
				.items
				.get(0)
				.unwrap()
				.borrow()
				.as_documentnode()
				.unwrap()
				.name(),
			"Bar"
		);
	}

	#[test]
	fn test_json() {
		let group = Group {
			id: Uuid::parse_str("fc2c9e3e-2cd7-4375-a6fe-49403cc9f82b").unwrap(),
			position: Vec2::new(0, 0),
			visible: true,
			locked: false,
			folded: false,
			name: "Foo".into(),
			items: vec![<dyn Node>::from(Box::new(Empty {
				id: Uuid::parse_str("1c3deaf3-3c7f-444d-9e05-9ddbcc2b9391").unwrap(),
				name: "Foo".into(),
			}))],
		};
		let json = serde_json::to_string(&group).unwrap();
		assert_eq!(json, "{\"id\":\"fc2c9e3e-2cd7-4375-a6fe-49403cc9f82b\",\"position\":{\"x\":0,\"y\":0},\"visible\":true,\"locked\":false,\"folded\":false,\"name\":\"Foo\",\"items\":[{\"node\":\"Empty\",\"props\":{\"id\":\"1c3deaf3-3c7f-444d-9e05-9ddbcc2b9391\",\"name\":\"Foo\"}}]}");
	}
}

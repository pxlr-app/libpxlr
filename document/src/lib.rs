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
	pub use std::{
		io,
		sync::{Arc, RwLock},
	};
	pub use uuid::Uuid;
}

#[doc(hidden)]
pub use typetag;

#[doc(hidden)]
pub use lazy_static;

#[cfg(test)]
mod tests {
	use super::prelude::*;

	#[test]
	fn test_cast() {
		let note = Note {
			id: Uuid::new_v4(),
			position: Arc::new(Vec2::new(0, 0)),
			visible: true,
			locked: false,
			name: Arc::new("Foo".into()),
		};
		let node: &dyn Node = &note;
		let documentnode = node.as_documentnode();
		assert_eq!(documentnode.is_some(), true);
		let note2 = documentnode.unwrap().downcast::<Note>();
		assert_eq!(note2.is_some(), true);
	}

	#[test]
	fn test_patch() {
		let note = Note {
			id: Uuid::new_v4(),
			position: Arc::new(Vec2::new(0, 0)),
			visible: true,
			locked: false,
			name: Arc::new("Foo".into()),
		};
		let (patch, _) = note.rename("Bar".into()).unwrap();
		let node = note.patch(&patch).unwrap();
		let note2 = node.downcast::<Note>().unwrap();
		assert_eq!(*note.name, "Foo");
		assert_eq!(*note2.name, "Bar");

		let group = Group {
			id: Uuid::parse_str("fc2c9e3e-2cd7-4375-a6fe-49403cc9f82b").unwrap(),
			position: Arc::new(Vec2::new(0, 0)),
			visible: true,
			locked: false,
			folded: false,
			name: Arc::new("Foo".into()),
			items: Arc::new(vec![<dyn Node>::from(Box::new(Note {
				id: Uuid::parse_str("1c3deaf3-3c7f-444d-9e05-9ddbcc2b9391").unwrap(),
				position: Arc::new(Vec2::new(0, 0)),
				visible: true,
				locked: false,
				name: Arc::new("Foo".into()),
			}))]),
		};
		let (patch, _) = group
			.items
			.get(0)
			.unwrap()
			.as_documentnode()
			.unwrap()
			.rename("Bar".into())
			.unwrap();
		let node = group.patch(&patch).unwrap();
		let group2 = node.downcast::<Group>().unwrap();
		assert_eq!(
			group
				.items
				.get(0)
				.unwrap()
				.as_documentnode()
				.unwrap()
				.name(),
			"Foo"
		);
		assert_eq!(
			group2
				.items
				.get(0)
				.unwrap()
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
			position: Arc::new(Vec2::new(0, 0)),
			visible: true,
			locked: false,
			folded: false,
			name: Arc::new("Foo".into()),
			items: Arc::new(vec![<dyn Node>::from(Box::new(Note {
				id: Uuid::parse_str("1c3deaf3-3c7f-444d-9e05-9ddbcc2b9391").unwrap(),
				position: Arc::new(Vec2::new(0, 0)),
				visible: true,
				locked: false,
				name: Arc::new("Foo".into()),
			}))]),
		};
		let json = serde_json::to_string(&group).unwrap();
		assert_eq!(json, "{\"id\":\"fc2c9e3e-2cd7-4375-a6fe-49403cc9f82b\",\"position\":{\"x\":0,\"y\":0},\"visible\":true,\"locked\":false,\"folded\":false,\"name\":\"Foo\",\"items\":[{\"node\":\"Note\",\"props\":{\"id\":\"1c3deaf3-3c7f-444d-9e05-9ddbcc2b9391\",\"position\":{\"x\":0,\"y\":0},\"visible\":true,\"locked\":false,\"name\":\"Foo\"}}]}");
	}
}

#![feature(const_btree_new)]

pub use document_derive::*;
pub mod any;
pub mod color;
pub mod file;
pub mod node;
pub mod parser;
pub mod patch;

pub mod prelude {
	pub use super::{any::*, color::*, file::*, node::*, parser, patch, patch::Patchable};
	pub use document_derive::*;
	pub use math::{Extent2, Vec2};
	pub use serde::{Deserialize, Serialize};
	pub use std::{
		io,
		sync::{Arc, RwLock},
	};
	pub use uuid::Uuid;
}

#[cfg(test)]
mod tests {
	use super::prelude::*;
	use std::io;

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
		let note2 = match node {
			NodeType::Note(n) => n,
			_ => panic!("Not a node"),
		};
		assert_eq!(*note.name, "Foo");
		assert_eq!(*note2.name, "Bar");

		let group = Group {
			id: Uuid::parse_str("fc2c9e3e-2cd7-4375-a6fe-49403cc9f82b").unwrap(),
			position: Arc::new(Vec2::new(0, 0)),
			visible: true,
			locked: false,
			folded: false,
			name: Arc::new("Foo".into()),
			children: Arc::new(vec![Arc::new(NodeType::Note(Note {
				id: Uuid::parse_str("1c3deaf3-3c7f-444d-9e05-9ddbcc2b9391").unwrap(),
				position: Arc::new(Vec2::new(0, 0)),
				visible: true,
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

	#[test]
	fn test_serialize() {
		let group = Group {
			id: Uuid::parse_str("fc2c9e3e-2cd7-4375-a6fe-49403cc9f82b").unwrap(),
			position: Arc::new(Vec2::new(0, 0)),
			visible: true,
			locked: false,
			folded: false,
			name: Arc::new("Foo".into()),
			children: Arc::new(vec![Arc::new(NodeType::Note(Note {
				id: Uuid::parse_str("1c3deaf3-3c7f-444d-9e05-9ddbcc2b9391").unwrap(),
				position: Arc::new(Vec2::new(0, 0)),
				visible: true,
				locked: false,
				name: Arc::new("Bar".into()),
			}))]),
		};
		let json = serde_json::to_string(&group).unwrap();
		assert_eq!(json, "{\"id\":\"fc2c9e3e-2cd7-4375-a6fe-49403cc9f82b\",\"position\":{\"x\":0,\"y\":0},\"visible\":true,\"locked\":false,\"folded\":false,\"name\":\"Foo\",\"children\":[{\"Note\":{\"id\":\"1c3deaf3-3c7f-444d-9e05-9ddbcc2b9391\",\"position\":{\"x\":0,\"y\":0},\"visible\":true,\"locked\":false,\"name\":\"Bar\"}}]}");
		let ron = ron::to_string(&group).unwrap();
		assert_eq!(ron, "(id:\"fc2c9e3e-2cd7-4375-a6fe-49403cc9f82b\",position:(x:0,y:0),visible:true,locked:false,folded:false,name:\"Foo\",children:[Note((id:\"1c3deaf3-3c7f-444d-9e05-9ddbcc2b9391\",position:(x:0,y:0),visible:true,locked:false,name:\"Bar\"))])");
	}

	#[test]
	fn test_file_write() {
		let note = NodeType::Note(Note {
			id: Uuid::new_v4(),
			position: Arc::new(Vec2::new(0, 0)),
			visible: true,
			locked: false,
			name: Arc::new("Foo".into()),
		});
		let mut buffer: io::Cursor<Vec<u8>> = io::Cursor::new(Vec::new());
		let mut file = File::new();
		let size = file.write(&mut buffer, &note).expect("Could not write");
		assert_eq!(size, 115);
		// std::fs::write("test_file_write_1.pxlr", buffer.get_ref()).expect("Could not dump");

		let group = NodeType::Group(Group {
			id: Uuid::parse_str("fc2c9e3e-2cd7-4375-a6fe-49403cc9f82b").unwrap(),
			position: Arc::new(Vec2::new(0, 0)),
			visible: true,
			locked: false,
			folded: false,
			name: Arc::new("Foo".into()),
			children: Arc::new(vec![Arc::new(NodeType::Note(Note {
				id: Uuid::parse_str("1c3deaf3-3c7f-444d-9e05-9ddbcc2b9391").unwrap(),
				position: Arc::new(Vec2::new(0, 0)),
				visible: true,
				locked: false,
				name: Arc::new("Bar".into()),
			}))]),
		});
		let mut buffer: io::Cursor<Vec<u8>> = io::Cursor::new(Vec::new());
		let mut file = File::new();
		let size = file.write(&mut buffer, &group).expect("Could not write");
		assert_eq!(size, 197);
		// std::fs::write("test_file_write_2.pxlr", buffer.get_ref()).expect("Could not dump");
	}

	#[test]
	fn test_file_read() {
		let group_id = Uuid::parse_str("fc2c9e3e-2cd7-4375-a6fe-49403cc9f82b").unwrap();
		let note_id = Uuid::parse_str("1c3deaf3-3c7f-444d-9e05-9ddbcc2b9391").unwrap();
		let group = NodeType::Group(Group {
			id: group_id,
			position: Arc::new(Vec2::new(0, 0)),
			visible: true,
			locked: false,
			folded: false,
			name: Arc::new("Foo".into()),
			children: Arc::new(vec![Arc::new(NodeType::Note(Note {
				id: note_id,
				position: Arc::new(Vec2::new(0, 0)),
				visible: true,
				locked: false,
				name: Arc::new("Bar".into()),
			}))]),
		});
		let mut buffer: io::Cursor<Vec<u8>> = io::Cursor::new(Vec::new());
		let mut file = File::new();
		let file_hash = file.index.hash;
		let size = file.write(&mut buffer, &group).expect("Could not write");
		assert_eq!(size, 197);
		// std::fs::write("test_file_read.pxlr", buffer.get_ref()).expect("Could not dump");

		let mut file = File::read(&mut buffer).expect("Could not read");
		assert_eq!(file.header.version, 0);
		assert_eq!(file.index.hash, file_hash);
		assert_eq!(file.index.prev_offset, 0);
		assert_eq!(file.index.size, 148);
		let node = file.get(&mut buffer, note_id).expect("Could not get Note");
		let note = match &*node {
			NodeType::Note(node) => node,
			_ => panic!("Not a Note"),
		};
		assert_eq!(*note.name, "Bar");
		assert_eq!(*note.position, Vec2::new(0, 0));
		assert_eq!(note.visible, true);
		assert_eq!(note.locked, false);
		assert_eq!(file.cache_node.len(), 1);

		let mut file = File::read(&mut buffer).expect("Could not read");
		let node = file.get(&mut buffer, group_id).expect("Could not get Note");
		let group = match &*node {
			NodeType::Group(node) => node,
			_ => panic!("Not a Group"),
		};
		assert_eq!(*group.name, "Foo");
		assert_eq!(*group.position, Vec2::new(0, 0));
		assert_eq!(group.visible, true);
		assert_eq!(group.locked, false);
		assert_eq!(group.folded, false);
		assert_eq!(group.children.len(), 1);
		assert_eq!(file.cache_node.len(), 2);
	}

	#[test]
	fn test_file_update() {
		let group_id = Uuid::parse_str("fc2c9e3e-2cd7-4375-a6fe-49403cc9f82b").unwrap();
		let note_id = Uuid::parse_str("1c3deaf3-3c7f-444d-9e05-9ddbcc2b9391").unwrap();
		let group = NodeType::Group(Group {
			id: group_id,
			position: Arc::new(Vec2::new(0, 0)),
			visible: true,
			locked: false,
			folded: false,
			name: Arc::new("Foo".into()),
			children: Arc::new(vec![]),
		});
		let mut buffer: io::Cursor<Vec<u8>> = io::Cursor::new(Vec::new());
		let mut file = File::new();
		let size = file.write(&mut buffer, &group).expect("Could not write");
		assert_eq!(size, 115);
		assert_eq!(file.rows.len(), 1);
		// std::fs::write("test_file_update_1.pxlr", buffer.get_ref()).expect("Could not dump");

		let group = NodeType::Group(Group {
			id: group_id,
			position: Arc::new(Vec2::new(0, 0)),
			visible: true,
			locked: false,
			folded: false,
			name: Arc::new("Bar".into()),
			children: Arc::new(vec![Arc::new(NodeType::Note(Note {
				id: note_id,
				position: Arc::new(Vec2::new(0, 0)),
				visible: true,
				locked: false,
				name: Arc::new("Baz".into()),
			}))]),
		});

		let size = file.update(&mut buffer, &group).expect("Could not write");
		assert_eq!(size, 192);
		assert_eq!(file.rows.len(), 2);
		// std::fs::write("test_file_update_2.pxlr", buffer.get_ref()).expect("Could not dump");

		let latest = File::read(&mut buffer).expect("Could not read latest version");
		latest
			.read_previous(&mut buffer)
			.expect("Could not read previous version");
		File::read_at(&mut buffer, 115).expect("Could not read version at offset 115");
	}
}

use document::{DocumentNode, Group, Note};
use math::Vec2;
// use serde_json::{Result, Value};
use std::rc::Rc;
use uuid::Uuid;

#[test]
fn it_serialize() {
	let group = Group::new(
		Some(Uuid::parse_str("fc2c9e3e-2cd7-4375-a6fe-49403cc9f82b").unwrap()),
		"Root",
		Vec2::new(0., 0.),
		vec![Rc::new(DocumentNode::Note(Note::new(
			Some(Uuid::parse_str("1c3deaf3-3c7f-444d-9e05-9ddbcc2b9391").unwrap()),
			"Foo",
			Vec2::new(0., 0.),
		)))],
	);

	let json = serde_json::to_string(&group).unwrap();
	assert_eq!(json, "{\"id\":\"fc2c9e3e-2cd7-4375-a6fe-49403cc9f82b\",\"is_visible\":true,\"is_locked\":false,\"is_folded\":false,\"name\":\"Root\",\"children\":[{\"Note\":{\"id\":\"1c3deaf3-3c7f-444d-9e05-9ddbcc2b9391\",\"is_visible\":true,\"is_locked\":false,\"note\":\"Foo\",\"position\":{\"x\":0.0,\"y\":0.0}}}],\"position\":{\"x\":0.0,\"y\":0.0}}");
}

#[test]
fn it_deserialize() {
	let group: Group = serde_json::from_str("{\"id\":\"fc2c9e3e-2cd7-4375-a6fe-49403cc9f82b\",\"is_visible\":true,\"is_locked\":false,\"is_folded\":false,\"name\":\"Root\",\"children\":[{\"Note\":{\"id\":\"1c3deaf3-3c7f-444d-9e05-9ddbcc2b9391\",\"is_visible\":true,\"is_locked\":false,\"note\":\"Foo\",\"position\":{\"x\":0.0,\"y\":0.0}}}],\"position\":{\"x\":0.0,\"y\":0.0}}").unwrap();
	assert_eq!(
		group.id,
		Uuid::parse_str("fc2c9e3e-2cd7-4375-a6fe-49403cc9f82b").unwrap()
	);
	assert_eq!(*group.name, "Root");
	assert_eq!(group.children.len(), 1);

	if let DocumentNode::Note(note) = &**group.children.get(0).unwrap() {
		assert_eq!(
			note.id,
			Uuid::parse_str("1c3deaf3-3c7f-444d-9e05-9ddbcc2b9391").unwrap()
		);
		assert_eq!(*note.note, "Foo");
	} else {
		panic!("Not a note!");
	}
}

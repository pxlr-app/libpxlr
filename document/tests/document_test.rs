use document::patch::*;
use document::DocumentNode;
use document::Group;
use document::Note;
use math::Vec2;
use std::sync::Arc;
use uuid::Uuid;

#[test]
fn it_patches() {
	{
		let group = Group::new(None, "Root", Vec2::new(0., 0.), vec![]);
		let (rename, _) = group.rename("Boot").unwrap();
		let new_group = group.patch(&rename).unwrap();
		assert_eq!(*new_group.name, "Boot");
		assert_eq!(Arc::strong_count(&group.name), 1);
		assert_eq!(Arc::strong_count(&new_group.name), 1);
		assert_eq!(Arc::strong_count(&group.position), 2);
		assert_eq!(Arc::strong_count(&group.children), 2);
	}
	{
		let group = Group::new(None, "Root", Vec2::new(0., 0.), vec![]);
		let rename = Patch::Rename(RenamePatch {
			target: Uuid::new_v4(),
			name: "Boot".to_owned(),
		});
		let new_group = group.patch(&rename);
		assert_eq!(new_group.is_none(), true);
		assert_eq!(Arc::strong_count(&group.name), 1);
		assert_eq!(Arc::strong_count(&group.position), 1);
		assert_eq!(Arc::strong_count(&group.children), 1);
	}
}

#[test]
fn it_patches_nested() {
	{
		let group = Group::new(
			None,
			"Root",
			Vec2::new(0., 0.),
			vec![Arc::new(DocumentNode::Note(Note::new(
				None,
				"Foo",
				Vec2::new(0., 0.),
			)))],
		);
		let note = if let DocumentNode::Note(note) = &**group.children.get(0).unwrap() {
			note
		} else {
			panic!("Not a note?");
		};
		let (rename, _) = note.rename("Bar").unwrap();
		let new_group = group.patch(&rename).unwrap();
		let new_note = if let DocumentNode::Note(note) = &**new_group.children.get(0).unwrap() {
			note
		} else {
			panic!("Not a note?")
		};
		assert_eq!(*new_group.name, "Root");
		assert_eq!(*note.note, "Foo");
		assert_eq!(*new_note.note, "Bar");
		assert_eq!(Arc::strong_count(&group.name), 2);
		assert_eq!(Arc::strong_count(&group.position), 2);
		assert_eq!(Arc::strong_count(&group.children), 1);
		assert_eq!(Arc::strong_count(&note.note), 1);
		assert_eq!(Arc::strong_count(&note.position), 2);
	}
	{
		let group = Group::new(
			None,
			"Root",
			Vec2::new(0., 0.),
			vec![Arc::new(DocumentNode::Note(Note::new(
				None,
				"Foo",
				Vec2::new(0., 0.),
			)))],
		);
		let rename = Patch::Rename(RenamePatch {
			target: Uuid::new_v4(),
			name: "Bar".to_owned(),
		});
		let new_group = group.patch(&rename);
		let note = if let DocumentNode::Note(note) = &**group.children.get(0).unwrap() {
			note
		} else {
			panic!("Not a note?");
		};
		assert_eq!(new_group.is_none(), true);
		assert_eq!(Arc::strong_count(&group.name), 1);
		assert_eq!(Arc::strong_count(&group.position), 1);
		assert_eq!(Arc::strong_count(&group.children), 1);
		assert_eq!(Arc::strong_count(&note.note), 1);
		assert_eq!(Arc::strong_count(&note.position), 1);
	}
}

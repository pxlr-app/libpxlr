use math::{Vec2};

use crate::group::Group;
use crate::label::Label;
use crate::node::INode;
use crate::patch::*;
use crate::sprite::Sprite;

pub enum Document {
	Group(Group),
	Label(Label),
	Sprite(Sprite)
}

pub trait IDocument: INode {
	fn position(&self) -> Vec2;
	fn patch(&self, patch: &Patch) -> Option<Document>;
}

#[cfg(test)]
mod tests {
	use crate::document::*;
	use crate::patch::*;
	use math::{Vec2};
	use std::rc::Rc;
	use uuid::Uuid;

	#[test]
	fn it_rename() {
		let root = Group::new(None, "Root", Vec2::new(0., 0.), vec![] );
		let new_root = root.patch(&Patch { target: root.id, payload: PatchAction::Rename("Ruut") });
		assert_eq!(new_root.is_some(), true);
		if let Some(Document::Group(new_root)) = new_root {
			assert_eq!(root.id, new_root.id);
			assert_eq!(*root.name, "Root");
			assert_eq!(*new_root.name, "Ruut");
			assert_eq!(Rc::strong_count(&root.name), 1);
			assert_eq!(Rc::strong_count(&new_root.name), 1);
		}

		let root = Group::new(None, "Root", Vec2::new(0., 0.), vec![] );
		let new_root = root.patch(&Patch { target: Uuid::new_v4(), payload: PatchAction::Rename("Ruut") });
		assert_eq!(new_root.is_some(), false);

		let label_id = Uuid::new_v4();
		let root = Group::new(None, "Root", Vec2::new(0., 0.), vec![Rc::new(Document::Label(Label::new(Some(label_id), "Label", Vec2::new(0., 0.))))] );
		if let Document::Label(label) = &**root.children.get(0).unwrap() {
			let new_root = root.patch(&Patch { target: label_id, payload: PatchAction::Rename("Labell") });
			assert_eq!(new_root.is_some(), true);
			if let Some(Document::Group(new_root)) = new_root {
				if let Document::Label(new_label) = &**new_root.children.get(0).unwrap() {
					assert_eq!(*label.name, "Label");
					assert_eq!(*new_label.name, "Labell");
					assert_eq!(Rc::strong_count(&label.name), 1);
					assert_eq!(Rc::strong_count(&new_label.name), 1);
					assert_eq!(Rc::strong_count(&root.name), 2);
					assert_eq!(Rc::strong_count(&new_root.name), 2);
				}
			}
		}
	}
}
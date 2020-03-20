use std::rc::Rc;
use uuid::Uuid;
use math::{Rect, Vec2};

pub struct Patch<'a> {
	target: Uuid,
	payload: PatchPayload<'a>,
}

pub enum PatchPayload<'a> {
	Rename(&'a str),
	Resize(u32, u32),
}

pub enum Document {
	Group(Group),
	Label(Label)
}

pub trait Patchable {
	fn patch(&self, patch: &Patch) -> Option<Document>;
}

pub struct Group {
	pub id: Uuid,
	pub name: Rc<String>,
	pub children: Rc<Vec<Rc<Document>>>,
	pub position: Rc<Vec2>,
}

impl Group {
	fn new(id: Option<Uuid>, name: &str, position: Vec2, children: Vec<Rc<Document>>) -> Group {
		Group {
			id: id.or(Some(Uuid::new_v4())).unwrap(),
			name: Rc::new(name.to_owned()),
			position: Rc::new(position),
			children: Rc::new(children)
		}
	}
}

impl Patchable for Group {
	fn patch(&self, patch: &Patch) -> Option<Document> {
		if patch.target == self.id {
			match &patch.payload {
				PatchPayload::Rename(new_name) => Some(Document::Group(Group {
					id: self.id,
					name: Rc::new(new_name.to_string()),
					children: Rc::clone(&self.children),
					position: Rc::clone(&self.position),
				})),
				_ => None,
			}
		} else {
			let mut mutated = false;
			let children = self.children.iter().map(|child| {
				match &**child {
					Document::Group(group) => {
						if let Some(doc) = group.patch(patch) {
							mutated = true;
							Rc::new(doc)
						} else {
							child.clone()
						}
					},
					Document::Label(label) => {
						if let Some(doc) = label.patch(patch) {
							mutated = true;
							Rc::new(doc)
						} else {
							child.clone()
						}
					}
					_ => child.clone()
				}
			}).collect::<Vec<_>>();
			
			if mutated {
				Some(Document::Group(Group {
					id: self.id,
					name: Rc::clone(&self.name),
					children: Rc::new(children),
					position: Rc::clone(&self.position),
				}))
			} else {
				None
			}
		}
	}
}

pub struct Label {
	pub id: Uuid,
	pub name: Rc<String>,
	pub position: Rc<Vec2>,
}

impl Label {
	fn new(id: Option<Uuid>, name: &str, position: Vec2) -> Label {
		Label {
			id: id.or(Some(Uuid::new_v4())).unwrap(),
			name: Rc::new(name.to_owned()),
			position: Rc::new(position),
		}
	}
}

impl Patchable for Label {
	fn patch(&self, patch: &Patch) -> Option<Document> {
		if patch.target == self.id {
			match &patch.payload {
				PatchPayload::Rename(new_name) => Some(Document::Label(Label {
					id: self.id,
					name: Rc::new(new_name.to_string()),
					position: Rc::clone(&self.position),
				})),
				_ => None,
			}
		} else {
			None
		}
	}
}

#[cfg(test)]
mod tests {
	use super::*;
	use uuid::Uuid;

	#[test]
	fn it_rename() {
		let root = Group::new(None, "Root", Vec2::new(0., 0.), vec![] );
		let new_root = root.patch(&Patch { target: root.id, payload: PatchPayload::Rename("Ruut") });
		assert_eq!(new_root.is_some(), true);
		if let Some(Document::Group(new_root)) = new_root {
			assert_eq!(root.id, new_root.id);
			assert_eq!(*root.name, "Root");
			assert_eq!(*new_root.name, "Ruut");
			assert_eq!(Rc::strong_count(&root.name), 1);
			assert_eq!(Rc::strong_count(&new_root.name), 1);
		}

		let root = Group::new(None, "Root", Vec2::new(0., 0.), vec![] );
		let new_root = root.patch(&Patch { target: Uuid::new_v4(), payload: PatchPayload::Rename("Ruut") });
		assert_eq!(new_root.is_some(), false);

		let label_id = Uuid::new_v4();
		let root = Group::new(None, "Root", Vec2::new(0., 0.), vec![Rc::new(Document::Label(Label::new(Some(label_id), "Label", Vec2::new(0., 0.))))] );
		if let Document::Label(label) = &**root.children.get(0).unwrap() {
			let new_root = root.patch(&Patch { target: label_id, payload: PatchPayload::Rename("Labell") });
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

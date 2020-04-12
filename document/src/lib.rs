#![allow(dead_code)]

mod document;
mod group;
mod node;
mod note;
pub mod patch;
pub mod sprite;

pub use self::document::*;
pub use self::group::*;
pub use self::node::*;
pub use self::note::*;

#[cfg(test)]
mod tests {
	use super::group::Group;
	use super::note::Note;
	use super::patch::*;
	use math::Vec2;

	use std::rc::Rc;
	use uuid::Uuid;

	#[test]
	fn it_patches() {
		{
			let group = Group::new(None, "Root", Vec2::new(0., 0.), vec![]);
			let (rename, _) = group.rename("Boot");
			let new_group = group.patch(&rename).unwrap();
			assert_eq!(*new_group.name, "Boot");
			assert_eq!(Rc::strong_count(&group.name), 1);
			assert_eq!(Rc::strong_count(&new_group.name), 1);
			assert_eq!(Rc::strong_count(&group.position), 2);
			assert_eq!(Rc::strong_count(&group.children), 2);
		}
		{
			let group = Group::new(None, "Root", Vec2::new(0., 0.), vec![]);
			let rename = RenamePatch {
				target: Uuid::new_v4(),
				name: "Boot".to_owned(),
			};
			let new_group = group.patch(&rename);
			assert_eq!(new_group.is_none(), true);
			assert_eq!(Rc::strong_count(&group.name), 1);
			assert_eq!(Rc::strong_count(&group.position), 1);
			assert_eq!(Rc::strong_count(&group.children), 1);
		}
	}

	#[test]
	fn it_patches_nested() {
		{
			let note = Rc::new(Note::new(None, "Foo", Vec2::new(0., 0.)));
			let group = Group::new(None, "Root", Vec2::new(0., 0.), vec![note.clone()]);
			let (rename, _) = note.rename("Bar");
			let new_group = group.patch(&rename).unwrap();
			let new_note = new_group
				.children
				.get(0)
				.unwrap()
				.as_any()
				.downcast_ref::<Note>()
				.unwrap();
			assert_eq!(*new_group.name, "Root");
			assert_eq!(*note.name, "Foo");
			assert_eq!(*new_note.name, "Bar");
			assert_eq!(Rc::strong_count(&group.name), 2);
			assert_eq!(Rc::strong_count(&group.position), 2);
			assert_eq!(Rc::strong_count(&group.children), 1);
			assert_eq!(Rc::strong_count(&note.name), 1);
			assert_eq!(Rc::strong_count(&note.position), 2);
		}
		{
			let note = Rc::new(Note::new(None, "Foo", Vec2::new(0., 0.)));
			let group = Group::new(None, "Root", Vec2::new(0., 0.), vec![note.clone()]);
			let rename = RenamePatch {
				target: Uuid::new_v4(),
				name: "Bar".to_owned(),
			};
			let new_group = group.patch(&rename);
			assert_eq!(new_group.is_none(), true);
			assert_eq!(Rc::strong_count(&group.name), 1);
			assert_eq!(Rc::strong_count(&group.position), 1);
			assert_eq!(Rc::strong_count(&group.children), 1);
			assert_eq!(Rc::strong_count(&note.name), 1);
			assert_eq!(Rc::strong_count(&note.position), 1);
		}
	}
}

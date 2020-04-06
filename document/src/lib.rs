#![allow(dead_code)]

mod document;
mod group;
mod label;
mod node;
mod patch;
mod sprite;

pub use self::document::*;
pub use self::group::*;
pub use self::label::*;
pub use self::node::*;
pub use self::patch::*;
pub use self::sprite::*;

#[cfg(test)]
mod tests {
	use super::group::Group;
	use super::label::Label;
	use super::patch::*;
	use math::Vec2;

	use uuid::Uuid;
	use std::rc::Rc;

	#[test]
	fn it_patches() {
		{
			let group = Group::new(None, "Root", Vec2::new(0., 0.), vec![]);
			let rename = group.rename("Boot");
			let new_group = group.patch(&rename).unwrap();
			assert_eq!(*new_group.name, "Boot");
			assert_eq!(Rc::strong_count(&group.name), 1);
			assert_eq!(Rc::strong_count(&new_group.name), 1);
			assert_eq!(Rc::strong_count(&group.position), 2);
			assert_eq!(Rc::strong_count(&group.children), 2);
		}
		{
			let group = Group::new(None, "Root", Vec2::new(0., 0.), vec![]);
			let rename = RenamePatch { target: Uuid::new_v4(), new_name: "Boot".to_owned() };
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
			let label = Rc::new(Label::new(None, "Foo", Vec2::new(0., 0.)));
			let group = Group::new(None, "Root", Vec2::new(0., 0.), vec![label.clone()]);
			let rename = label.rename("Bar");
			let new_group = group.patch(&rename).unwrap();
			let new_label = new_group.children.get(0).unwrap().as_any().downcast_ref::<Label>().unwrap();
			assert_eq!(*new_group.name, "Root");
			assert_eq!(*label.name, "Foo");
			assert_eq!(*new_label.name, "Bar");
			assert_eq!(Rc::strong_count(&group.name), 2);
			assert_eq!(Rc::strong_count(&group.position), 2);
			assert_eq!(Rc::strong_count(&group.children), 1);
			assert_eq!(Rc::strong_count(&label.name), 1);
			assert_eq!(Rc::strong_count(&label.position), 2);
		}
		{
			let label = Rc::new(Label::new(None, "Foo", Vec2::new(0., 0.)));
			let group = Group::new(None, "Root", Vec2::new(0., 0.), vec![label.clone()]);
			let rename = RenamePatch { target: Uuid::new_v4(), new_name: "Bar".to_owned() };
			let new_group = group.patch(&rename);
			assert_eq!(new_group.is_none(), true);
			assert_eq!(Rc::strong_count(&group.name), 1);
			assert_eq!(Rc::strong_count(&group.position), 1);
			assert_eq!(Rc::strong_count(&group.children), 1);
			assert_eq!(Rc::strong_count(&label.name), 1);
			assert_eq!(Rc::strong_count(&label.position), 1);
		}
	}
}
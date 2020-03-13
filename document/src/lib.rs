use std::rc::Rc;
use uuid::Uuid;
// use im::Vector as List;
use math::{Rect, Vec2};

pub struct Patch {
	target: Uuid,
	payload: PatchPayload,
}

pub enum PatchPayload {
	Rename(String),
	Resize(u32, u32),
}

pub trait Patchable {
	fn apply_patch(&self, patch: &Patch) -> Option<Self>
	where
		Self: Sized;
}

pub trait Document: Patchable {
	fn get_bounds(&self) -> Rect;
}

pub struct Group {
	pub id: Uuid,
	pub name: String,
	pub children: Rc<Vec<Rc<Box<dyn Document>>>>,
	pub position: Rc<Vec2>,
}

impl Patchable for Group {
	fn apply_patch(&self, patch: &Patch) -> Option<Self> {
		if patch.target == self.id {
			match &patch.payload {
				PatchPayload::Rename(name) => Some(Group { id: self.id, name: name.clone(), children: Rc::clone(&self.children), position: Rc::clone(&self.position) }),
				_ => None
			}
		} else {
			let t: Rc<Box<dyn Document>> = self.children[0];
			t.apply_patch(patch);
			// for child in (*self.children).into_iter() {
			//  	// let _a = *child;
			//  	child.apply_patch(patch);

			// // 	//let a = (*child).apply_patch(patch);
			// }
			// let children: Vec<Rc<Box<dyn Document>>> = (*self.children).clone().into_iter().map(|child| {
			// 	if let Some(new_child) = (*child).apply_patch(patch) {
			// 		new_child
			// 	} else {
			// 		child
			// 	}
			// }).collect();
			None
		}
	}
}

impl Document for Group {
	fn get_bounds(&self) -> Rect {
		Rect::new(*self.position, *self.position)
	}
}

pub struct Label {
	pub id: Uuid,
	pub name: String,
	pub position: Rc<Vec2>,
}

impl Patchable for Label {
	fn apply_patch(&self, patch: &Patch) -> Option<Self> {
		if patch.target == self.id {
			match &patch.payload {
				PatchPayload::Rename(new_name) => Some(Label {
					id: self.id,
					name: new_name.clone(),
					position: Rc::clone(&self.position),
				}),
				_ => None,
			}
		} else {
			None
		}
	}
}

impl Document for Label {
	fn get_bounds(&self) -> Rect {
		Rect::new(*self.position, *self.position)
	}
}

#[cfg(test)]
mod tests {
	// use super::*;
	use std::rc::Rc;

	trait Printable {
		fn stringify(&self) -> String;
	}

	impl Printable for i32 {
		fn stringify(&self) -> String { self.to_string() }
	}

	fn print(a: Rc<Vec<Rc<Box<dyn Printable>>>>) {
		println!("{}", a[0].stringify());
		let t: Rc<Box<dyn Printable>> = a[0];
		t.stringify();
	}

	#[test]
	fn it_adds() {
		// assert_eq!(Vector2::new(1.0, 1.0) + Vector2::new(1.0, 1.0), Vector2::new(2.0, 2.0));
		// assert_eq!(Vector2::new(0.0, 0.0) + 2.0, Vector2::new(2.0, 2.0));
		// let mut v1 = Vector2::new(0.0, 0.0);
		// v1 += 2.0;
		// assert_eq!(v1, Vector2::new(2.0, 2.0));
		print(Rc::new(vec![Rc::new(Box::new(10))]));
	}
}

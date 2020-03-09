use std::sync::Arc;
use uuid::Uuid;
use im::Vector as List;
use math::{ Rect, Vec2 };

pub struct Patch {
	target: Uuid,
	payload: PatchPayload,
}

pub enum PatchPayload {
	Rename(String),
	Resize(u32, u32)
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
	pub children: Arc<List<Arc<Box<dyn Document>>>>,
}

impl Patchable for Group {
	fn apply_patch(&self, patch: &Patch) -> Option<Self> {
		if patch.target == self.id {
			match &patch.payload {
				PatchPayload::Rename(new_name) => Some(Group { id: self.id, name: new_name.clone(), children: self.children.clone() }),
				_ => None
			}
		} else {
			None
		}
	}
}

impl Document for Group {
	fn get_bounds(&self) -> Rect {
		Rect::new(Vec2::new(0., 0.), Vec2::new(0., 0.))
	}
}

pub struct Label {
	pub id: Uuid,
	pub name: String,
}

impl Patchable for Label {
	fn apply_patch(&self, patch: &Patch) -> Option<Self> {
		if patch.target == self.id {
			match &patch.payload {
				PatchPayload::Rename(new_name) => Some(Label { id: self.id, name: new_name.clone() }),
				_ => None
			}
		} else {
			None
		}
	}
}

#[cfg(test)]
mod tests {
	use super::*;

	#[test]
	fn it_adds() {
		// assert_eq!(Vector2::new(1.0, 1.0) + Vector2::new(1.0, 1.0), Vector2::new(2.0, 2.0));
		// assert_eq!(Vector2::new(0.0, 0.0) + 2.0, Vector2::new(2.0, 2.0));
		// let mut v1 = Vector2::new(0.0, 0.0);
		// v1 += 2.0;
		// assert_eq!(v1, Vector2::new(2.0, 2.0));
	}
}

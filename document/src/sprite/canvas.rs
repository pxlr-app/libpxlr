use std::rc::Rc;
use uuid::Uuid;
use math::{Extent2};

use crate::node::*;
use crate::patch::*;
use crate::sprite::*;

pub struct Canvas<T> {
	pub id: Uuid,
	pub name: Rc<String>,
	pub size: Rc<Extent2<u16>>,
	pub data: Rc<Vec<T>>,
}

impl<T> Canvas<T> {
	pub fn new(id: Option<Uuid>, name: &str, size: Extent2<u16>, data: Vec<T>) -> Canvas<T> {
		Canvas::<T> {
			id: id.or(Some(Uuid::new_v4())).unwrap(),
			name: Rc::new(name.to_owned()),
			size: Rc::new(size),
			data: Rc::new(data),
		}
	}
}

impl<T> Node for Canvas<T> {
	fn id(&self) -> Uuid {
		self.id
	}
}

impl<T> Layer for Canvas<T> {}

impl<T> Patchable for Canvas<T> {
	fn patch(&self, patch: &dyn PatchImpl) -> Option<Box<Self>> {
		if patch.target() == self.id {
			if let Some(rename) = patch.as_any().downcast_ref::<RenamePatch>() {
				Some(Box::new(Canvas::<T> {
					id: self.id,
					name: Rc::new(rename.new_name.clone()),
					size: self.size.clone(),
					data: self.data.clone(),
				}))
			} else {
				None
			}
		} else {
			None
		}
	}
}

pub type CanvasRGBA = Canvas<(u8, u8, u8, u8)>;
pub type CanvasI = Canvas<u8>;
pub type CanvasUV = Canvas<(u16, u16)>;
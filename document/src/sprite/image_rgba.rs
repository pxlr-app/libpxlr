use std::rc::Rc;
use uuid::Uuid;
use math::{Vec2, Extent2};

use crate::document::*;
use crate::node::*;
use crate::patch::*;
use crate::sprite::*;

pub struct ImageRGBA {
	pub id: Uuid,
	pub name: Rc<String>,
	pub position: Rc<Vec2<f32>>,
	pub size: Rc<Extent2<u16>>,
	// GL.RGBA
	pub data: Rc<[(u8, u8, u8, u8)]>,
}

impl ImageRGBA {
	pub fn new(id: Option<Uuid>, name: &str, position: Vec2<f32>, size: Extent2<u16>, data: Rc<[(u8, u8, u8, u8)]>) -> ImageRGBA {
		ImageRGBA {
			id: id.or(Some(Uuid::new_v4())).unwrap(),
			name: Rc::new(name.to_owned()),
			position: Rc::new(position),
			size: Rc::new(size),
			data: Rc::clone(&data),
		}
	}
}

impl INode for ImageRGBA {
	fn id(&self) -> Uuid {
		self.id
	}
	fn display(&self) -> String {
		self.name.to_string()
	}
}

impl IDocument for ImageRGBA {
	fn position(&self) -> Vec2<f32> {
		*(self.position).clone()
	}
}

impl ILayer for ImageRGBA {
	fn patch(&self, patch: &Patch) -> Option<Layer> {
		if patch.target == self.id {
			match &patch.payload {
				PatchAction::Rename(new_name) => Some(Layer::ImageRGBA(ImageRGBA {
					id: self.id,
					name: Rc::new(new_name.to_string()),
					position: Rc::clone(&self.position),
					size: Rc::clone(&self.size),
					data: Rc::clone(&self.data),
				})),
				_ => None,
			}
		} else {
			None
		}
	}
}
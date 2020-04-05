use std::rc::Rc;
use uuid::Uuid;
use math::{Extent2};

use crate::node::*;
use crate::patch::*;
use crate::sprite::*;

macro_rules! impl_image {
	($name:ident, $data:ty) => {
		pub struct $name {
			pub id: Uuid,
			pub name: Rc<String>,
			pub size: Rc<Extent2<u16>>,
			pub data: Rc<Vec<$data>>,
		}

		impl $name {
			pub fn new(id: Option<Uuid>, name: &str, size: Extent2<u16>, data: Vec<$data>) -> $name {
				$name {
					id: id.or(Some(Uuid::new_v4())).unwrap(),
					name: Rc::new(name.to_owned()),
					size: Rc::new(size),
					data: Rc::new(data),
				}
			}
		}

		impl Node for $name {
			fn id(&self) -> Uuid {
				self.id
			}
		}

		impl Layer for $name {}

		impl Patchable for $name {
			fn patch(&self, patch: &dyn PatchImpl) -> Option<Box<Self>> {
				if patch.target() == self.id {
					if let Some(rename) = patch.as_any().downcast_ref::<RenamePatch>() {
						Some(Box::new($name {
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
	};
}

impl_image!(ImageRGBA, (u8, u8, u8, u8));
impl_image!(ImageI, u8);
impl_image!(ImageUV, (u16, u16));
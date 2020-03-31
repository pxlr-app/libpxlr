use std::rc::Rc;
use uuid::Uuid;
use math::{Vec2, Extent2};

pub struct ImageUV {
	pub id: Uuid,
	pub name: Rc<String>,
	pub position: Rc<Vec2<f32>>,
	pub size: Rc<Extent2<u16>>,
	// GL.RG16UI
	pub data: Rc<Vec<(u16, u16)>>,
}

impl ImageUV {
	pub fn new(id: Option<Uuid>, name: &str, position: Vec2<f32>, size: Extent2<u16>, data: Vec<(u16, u16)>) -> ImageUV {
		ImageUV {
			id: id.or(Some(Uuid::new_v4())).unwrap(),
			name: Rc::new(name.to_owned()),
			position: Rc::new(position),
			size: Rc::new(size),
			data: Rc::new(data),
		}
	}
}
use std::rc::Rc;
use uuid::Uuid;
use math::{Vec2, Extent2};

pub struct ImageI {
	pub id: Uuid,
	pub name: Rc<String>,
	pub position: Rc<Vec2<f32>>,
	pub size: Rc<Extent2<u16>>,
	// GL.R8
	pub data: Rc<Vec<u8>>,
}

impl ImageI {
	pub fn new(id: Option<Uuid>, name: &str, position: Vec2<f32>, size: Extent2<u16>, data: Vec<u8>) -> ImageI {
		ImageI {
			id: id.or(Some(Uuid::new_v4())).unwrap(),
			name: Rc::new(name.to_owned()),
			position: Rc::new(position),
			size: Rc::new(size),
			data: Rc::new(data),
		}
	}
}
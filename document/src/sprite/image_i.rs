use std::rc::Rc;
use uuid::Uuid;
use math::{Vec2, Extent2};

pub struct ImageI {
	pub id: Uuid,
	pub name: Rc<String>,
	pub position: Rc<Vec2<f32>>,
	pub size: Rc<Extent2<u16>>,
	// GL.R8
	pub data: Rc<[u8]>,
}
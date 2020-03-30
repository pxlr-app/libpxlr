use std::rc::Rc;
use uuid::Uuid;
use math::{Vec2, Extent2};

pub struct ImageRGBA {
	pub id: Uuid,
	pub name: Rc<String>,
	pub position: Rc<Vec2<f32>>,
	pub size: Rc<Extent2<u16>>,
	// GL.RGBA
	pub data: Rc<[(u8, u8, u8, u8)]>,
}
use crate as document;
use crate::prelude::*;
use std::fmt::Debug;

#[typetag::serde(tag = "color", content = "props")]
pub trait Color: Any + Debug {}
impl Downcast for dyn Color {}

#[derive(Color, Debug, Clone, Serialize, Deserialize)]
pub struct Grey {
	pub g: u32,
}

#[derive(Color, Debug, Clone, Serialize, Deserialize)]
pub struct RGBA {
	pub r: u32,
	pub g: u32,
	pub b: u32,
	pub a: u32,
}

#[derive(Color, Debug, Clone, Serialize, Deserialize)]
pub struct UV {
	pub u: f32,
	pub v: f32,
}

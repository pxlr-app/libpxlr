use crate as document;
use crate::prelude::*;
use std::fmt::Debug;

pub trait Color: Any + Debug {}
impl Downcast for dyn Color {}

#[derive(Color, Debug, Clone, Serialize, Deserialize)]
pub struct Grey {
	pub g: u8,
}

#[derive(Color, Debug, Clone, Serialize, Deserialize)]
pub struct RGBA {
	pub r: u8,
	pub g: u8,
	pub b: u8,
	pub a: u8,
}

#[derive(Color, Debug, Clone, Serialize, Deserialize)]
pub struct UV {
	pub u: f32,
	pub v: f32,
}

#[derive(Color, Debug, Clone, Serialize, Deserialize)]
pub struct XYZ {
	pub x: f32,
	pub y: f32,
	pub z: f32,
}

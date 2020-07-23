use crate as document;
use crate::prelude::*;
use nom::number::complete::{le_f32, le_u8};
use std::fmt::Debug;

pub trait Color: Any + Debug {}
impl Downcast for dyn Color {}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub enum ColorMode {
	Grey,
	RGBA,
	UV,
	XYZ,
}

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

impl parser::Parse for ColorMode {
	fn parse(bytes: &[u8]) -> nom::IResult<&[u8], ColorMode> {
		let (bytes, idx) = le_u8(bytes)?;
		match idx {
			0 => Ok((bytes, ColorMode::Grey)),
			1 => Ok((bytes, ColorMode::RGBA)),
			2 => Ok((bytes, ColorMode::UV)),
			3 => Ok((bytes, ColorMode::XYZ)),
			_ => Err(nom::Err::Error((bytes, nom::error::ErrorKind::NoneOf))),
		}
	}
}

impl parser::Write for ColorMode {
	fn write(&self, writer: &mut dyn io::Write) -> io::Result<usize> {
		let idx: u8 = match self {
			ColorMode::Grey => 0,
			ColorMode::RGBA => 1,
			ColorMode::UV => 2,
			ColorMode::XYZ => 3,
		};
		writer.write_all(&idx.to_le_bytes())?;
		Ok(1)
	}
}

impl parser::Parse for Grey {
	fn parse(bytes: &[u8]) -> nom::IResult<&[u8], Grey> {
		let (bytes, g) = le_u8(bytes)?;
		Ok((bytes, Grey { g }))
	}
}

impl parser::Write for Grey {
	fn write(&self, writer: &mut dyn io::Write) -> io::Result<usize> {
		writer.write_all(&self.g.to_le_bytes())?;
		Ok(1)
	}
}

impl parser::Parse for RGBA {
	fn parse(bytes: &[u8]) -> nom::IResult<&[u8], RGBA> {
		let (bytes, r) = le_u8(bytes)?;
		let (bytes, g) = le_u8(bytes)?;
		let (bytes, b) = le_u8(bytes)?;
		let (bytes, a) = le_u8(bytes)?;
		Ok((bytes, RGBA { r, g, b, a }))
	}
}

impl parser::Write for RGBA {
	fn write(&self, writer: &mut dyn io::Write) -> io::Result<usize> {
		writer.write_all(&self.r.to_le_bytes())?;
		writer.write_all(&self.g.to_le_bytes())?;
		writer.write_all(&self.b.to_le_bytes())?;
		writer.write_all(&self.a.to_le_bytes())?;
		Ok(1)
	}
}

impl parser::Parse for UV {
	fn parse(bytes: &[u8]) -> nom::IResult<&[u8], UV> {
		let (bytes, u) = le_f32(bytes)?;
		let (bytes, v) = le_f32(bytes)?;
		Ok((bytes, UV { u, v }))
	}
}

impl parser::Write for UV {
	fn write(&self, writer: &mut dyn io::Write) -> io::Result<usize> {
		writer.write_all(&self.u.to_le_bytes())?;
		writer.write_all(&self.v.to_le_bytes())?;
		Ok(1)
	}
}

impl parser::Parse for XYZ {
	fn parse(bytes: &[u8]) -> nom::IResult<&[u8], XYZ> {
		let (bytes, x) = le_f32(bytes)?;
		let (bytes, y) = le_f32(bytes)?;
		let (bytes, z) = le_f32(bytes)?;
		Ok((bytes, XYZ { x, y, z }))
	}
}

impl parser::Write for XYZ {
	fn write(&self, writer: &mut dyn io::Write) -> io::Result<usize> {
		writer.write_all(&self.x.to_le_bytes())?;
		writer.write_all(&self.y.to_le_bytes())?;
		writer.write_all(&self.z.to_le_bytes())?;
		Ok(1)
	}
}

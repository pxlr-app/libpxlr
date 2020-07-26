use crate::prelude::*;
use bitflags::bitflags;
use nom::number::complete::{le_f32, le_u8};
use std::fmt::Debug;

pub trait Color: Debug {
	fn stride() -> usize;
}

bitflags! {
	#[derive(Serialize, Deserialize)]
	pub struct Channel: u8 {
		const A	= 0b00000001;
		const RGB 	= 0b00000010;
		const UV 	= 0b00000100;
		const XYZ 	= 0b00001000;
	}
}

#[derive(Debug, PartialEq, Clone, Copy, Serialize, Deserialize)]
pub struct A {
	pub a: u8,
}

#[derive(Debug, PartialEq, Clone, Copy, Serialize, Deserialize)]
pub struct RGB {
	pub r: u8,
	pub g: u8,
	pub b: u8,
}

#[derive(Debug, PartialEq, Clone, Copy, Serialize, Deserialize)]
pub struct UV {
	pub u: f32,
	pub v: f32,
}

#[derive(Debug, PartialEq, Clone, Copy, Serialize, Deserialize)]
pub struct XYZ {
	pub x: f32,
	pub y: f32,
	pub z: f32,
}

impl Channel {
	pub fn stride(&self) -> usize {
		match self.bits() {
			1 => A::stride(),
			2 => RGB::stride(),
			4 => UV::stride(),
			8 => XYZ::stride(),
			_ => 0,
		}
	}
}

impl Color for A {
	fn stride() -> usize {
		1
	}
}

impl Color for RGB {
	fn stride() -> usize {
		3
	}
}

impl Color for UV {
	fn stride() -> usize {
		8
	}
}

impl Color for XYZ {
	fn stride() -> usize {
		12
	}
}

impl parser::Parse for Channel {
	fn parse(bytes: &[u8]) -> nom::IResult<&[u8], Channel> {
		let (bytes, bits) = le_u8(bytes)?;
		Ok((bytes, Channel::from_bits(bits).unwrap()))
	}
}

impl parser::Write for Channel {
	fn write(&self, writer: &mut dyn io::Write) -> io::Result<usize> {
		writer.write_all(&self.bits().to_le_bytes())?;
		Ok(1)
	}
}

impl parser::Parse for A {
	fn parse(bytes: &[u8]) -> nom::IResult<&[u8], A> {
		let (bytes, a) = le_u8(bytes)?;
		Ok((bytes, A { a }))
	}
}

impl parser::Write for A {
	fn write(&self, writer: &mut dyn io::Write) -> io::Result<usize> {
		writer.write_all(&self.a.to_le_bytes())?;
		Ok(1)
	}
}

impl parser::Parse for RGB {
	fn parse(bytes: &[u8]) -> nom::IResult<&[u8], RGB> {
		let (bytes, r) = le_u8(bytes)?;
		let (bytes, g) = le_u8(bytes)?;
		let (bytes, b) = le_u8(bytes)?;
		Ok((bytes, RGB { r, g, b }))
	}
}

impl parser::Write for RGB {
	fn write(&self, writer: &mut dyn io::Write) -> io::Result<usize> {
		writer.write_all(&self.r.to_le_bytes())?;
		writer.write_all(&self.g.to_le_bytes())?;
		writer.write_all(&self.b.to_le_bytes())?;
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

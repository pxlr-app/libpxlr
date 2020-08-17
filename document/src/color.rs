use crate::prelude::*;
use bitflags::bitflags;
use nom::number::complete::{le_f32, le_u8};
use std::fmt::Debug;

pub trait Color: Debug {
	fn size() -> usize;
}

bitflags! {
	#[derive(Serialize, Deserialize)]
	pub struct Channel: u8 {
		const I     = 0b00000001;
		const RGB 	= 0b00000010;
		const A		= 0b00000100;
		const UV 	= 0b00001000;
		const XYZ 	= 0b00010000;
	}
}

#[repr(C)]
#[derive(Debug, PartialEq, Clone, Copy, Serialize, Deserialize)]
pub struct I {
	pub i: u8,
}

#[repr(C)]
#[derive(Debug, PartialEq, Clone, Copy, Serialize, Deserialize)]
pub struct RGB {
	pub r: u8,
	pub g: u8,
	pub b: u8,
}

#[repr(C)]
#[derive(Debug, PartialEq, Clone, Copy, Serialize, Deserialize)]
pub struct A {
	pub a: u8,
}

#[repr(C)]
#[derive(Debug, PartialEq, Clone, Copy, Serialize, Deserialize)]
pub struct UV {
	pub u: f32,
	pub v: f32,
}

#[repr(C)]
#[derive(Debug, PartialEq, Clone, Copy, Serialize, Deserialize)]
pub struct XYZ {
	pub x: f32,
	pub y: f32,
	pub z: f32,
}

impl Channel {
	/// Length of the entire channel
	pub fn len(&self) -> usize {
		let mut len = 0usize;
		let bits = self.bits();
		if bits & Channel::I.bits > 0 {
			len += I::size();
		}
		if bits & Channel::RGB.bits > 0 {
			len += RGB::size();
		}
		if bits & Channel::A.bits > 0 {
			len += A::size();
		}
		if bits & Channel::UV.bits > 0 {
			len += UV::size();
		}
		if bits & Channel::XYZ.bits > 0 {
			len += XYZ::size();
		}
		len
	}

	/// Retrive the offset to iterate from channel to channel
	///
	/// Given a composite channel RGBA, retrive the offset to A.
	/// Offset is equal to the length of RGB.
	///
	/// In a composite channel of RGBAXYZ, retrieve the offset to XYZ.
	/// Offset is equal to the length of RGB + A.
	///
	/// ```
	/// use document::color::Channel;
	/// let composite = Channel::RGB | Channel::A | Channel::XYZ;
	/// assert_eq!(composite.offset(Channel::XYZ), Channel::RGB.len() + Channel::A.len());
	/// ```
	///
	pub fn offset(&self, channel: Channel) -> usize {
		let mut offset = 0usize;
		let bits = self.bits();
		for chan in [
			Channel::I,
			Channel::RGB,
			Channel::A,
			Channel::UV,
			Channel::XYZ,
		]
		.iter()
		{
			if &channel == chan {
				return offset;
			}
			if bits & chan.bits > 0 {
				offset += chan.len();
			}
		}
		0
	}
}

impl Color for I {
	fn size() -> usize {
		1
	}
}

impl Color for RGB {
	fn size() -> usize {
		3
	}
}

impl Color for A {
	fn size() -> usize {
		1
	}
}

impl Color for UV {
	fn size() -> usize {
		8
	}
}

impl Color for XYZ {
	fn size() -> usize {
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

impl parser::Parse for I {
	fn parse(bytes: &[u8]) -> nom::IResult<&[u8], I> {
		let (bytes, i) = le_u8(bytes)?;
		Ok((bytes, I { i }))
	}
}

impl parser::Write for I {
	fn write(&self, writer: &mut dyn io::Write) -> io::Result<usize> {
		writer.write_all(&self.i.to_le_bytes())?;
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

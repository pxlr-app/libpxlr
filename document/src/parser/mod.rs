pub mod v0;
use math::{Extent2, Vec2};
pub use nom::IResult as Result;
use nom::{
	bytes::complete::{tag, take},
	number::complete::{le_u32, le_u8},
};
use std::io;
use uuid::Uuid;

pub trait Parse {
	fn parse(bytes: &[u8]) -> Result<&[u8], Self>
	where
		Self: Sized;
}

pub trait Write {
	fn write(&self, writer: &mut dyn io::Write) -> io::Result<usize>;
}

#[derive(Debug, Clone)]
pub struct Header {
	pub version: u8,
}

const MAGIC_NUMBER: &'static str = "PXLR";

impl Parse for Header {
	fn parse(bytes: &[u8]) -> Result<&[u8], Header> {
		let (bytes, _) = tag(MAGIC_NUMBER)(bytes)?;
		let (bytes, version) = le_u8(bytes)?;
		Ok((bytes, Header { version }))
	}
}

impl Write for Header {
	fn write(&self, writer: &mut dyn io::Write) -> io::Result<usize> {
		writer.write_all(MAGIC_NUMBER.as_bytes())?;
		writer.write_all(&self.version.to_le_bytes())?;
		Ok(5)
	}
}

impl Parse for String {
	fn parse(bytes: &[u8]) -> Result<&[u8], String> {
		let (bytes, len) = le_u32(bytes)?;
		let (bytes, buffer) = take(len as usize)(bytes)?;
		Ok((
			bytes,
			std::str::from_utf8(buffer)
				.expect("Expected a valid UTF8 string.")
				.to_owned(),
		))
	}
}

impl Write for String {
	fn write(&self, writer: &mut dyn io::Write) -> io::Result<usize> {
		writer.write_all(&(self.len() as u32).to_le_bytes())?;
		let buf = self.as_bytes();
		writer.write_all(buf)?;
		Ok(4usize + buf.len())
	}
}

impl Parse for Uuid {
	fn parse(bytes: &[u8]) -> Result<&[u8], Uuid> {
		let (bytes, buffer) = take(16usize)(bytes)?;
		Ok((bytes, Uuid::from_slice(buffer).unwrap()))
	}
}

impl Write for Uuid {
	fn write(&self, writer: &mut dyn io::Write) -> io::Result<usize> {
		writer.write_all(self.as_bytes())?;
		Ok(16)
	}
}

impl Parse for Vec2<u32> {
	fn parse(bytes: &[u8]) -> Result<&[u8], Vec2<u32>> {
		let (bytes, x) = le_u32(bytes)?;
		let (bytes, y) = le_u32(bytes)?;
		Ok((bytes, Vec2::new(x, y)))
	}
}

impl Write for Vec2<u32> {
	fn write(&self, writer: &mut dyn io::Write) -> io::Result<usize> {
		writer.write_all(&self.x.to_le_bytes())?;
		writer.write_all(&self.y.to_le_bytes())?;
		Ok(8)
	}
}

impl Parse for Extent2<u32> {
	fn parse(bytes: &[u8]) -> Result<&[u8], Extent2<u32>> {
		let (bytes, w) = le_u32(bytes)?;
		let (bytes, h) = le_u32(bytes)?;
		Ok((bytes, Extent2::new(w, h)))
	}
}

impl Write for Extent2<u32> {
	fn write(&self, writer: &mut dyn io::Write) -> io::Result<usize> {
		writer.write_all(&self.w.to_le_bytes())?;
		writer.write_all(&self.h.to_le_bytes())?;
		Ok(8)
	}
}

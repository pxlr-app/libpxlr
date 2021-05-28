use crate::{Parse, Write};
use async_std::io;
use async_trait::async_trait;
use nom::{
	bytes::complete::take,
	number::complete::{le_i32, le_u32},
	IResult,
};
use uuid::Uuid;
use vek::{geom::repr_c::Rect, vec::repr_c::vec2::Vec2};

impl Parse for String {
	fn parse(bytes: &[u8]) -> IResult<&[u8], String> {
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

#[async_trait(?Send)]
impl Write for String {
	async fn write<W: io::Write + std::marker::Unpin>(&self, writer: &mut W) -> io::Result<usize> {
		use async_std::io::prelude::WriteExt;
		writer.write(&(self.len() as u32).to_le_bytes()).await?;
		let buf = self.as_bytes();
		writer.write(buf).await?;
		Ok(4usize + buf.len())
	}
}

#[async_trait(?Send)]
impl Write for &str {
	async fn write<W: io::Write + std::marker::Unpin>(&self, writer: &mut W) -> io::Result<usize> {
		use async_std::io::prelude::WriteExt;
		writer.write(&(self.len() as u32).to_le_bytes()).await?;
		let buf = self.as_bytes();
		writer.write(buf).await?;
		Ok(4usize + buf.len())
	}
}

impl Parse for Uuid {
	fn parse(bytes: &[u8]) -> IResult<&[u8], Uuid> {
		let (bytes, buffer) = take(16usize)(bytes)?;
		Ok((bytes, Uuid::from_slice(buffer).unwrap()))
	}
}

#[async_trait(?Send)]
impl Write for Uuid {
	async fn write<W: io::Write + std::marker::Unpin>(&self, writer: &mut W) -> io::Result<usize> {
		use async_std::io::prelude::WriteExt;
		writer.write(self.as_bytes()).await?;
		Ok(16)
	}
}

impl Parse for Rect<i32, i32> {
	fn parse(bytes: &[u8]) -> IResult<&[u8], Rect<i32, i32>> {
		let (bytes, x) = le_i32(bytes)?;
		let (bytes, y) = le_i32(bytes)?;
		let (bytes, w) = le_i32(bytes)?;
		let (bytes, h) = le_i32(bytes)?;
		Ok((bytes, Rect::new(x, y, w, h)))
	}
}

#[async_trait(?Send)]
impl Write for Rect<i32, i32> {
	async fn write<W: io::Write + std::marker::Unpin>(&self, writer: &mut W) -> io::Result<usize> {
		use async_std::io::prelude::WriteExt;
		writer.write(&self.x.to_le_bytes()).await?;
		writer.write(&self.y.to_le_bytes()).await?;
		writer.write(&self.w.to_le_bytes()).await?;
		writer.write(&self.h.to_le_bytes()).await?;
		Ok(16)
	}
}

impl Parse for Vec2<i32> {
	fn parse(bytes: &[u8]) -> IResult<&[u8], Vec2<i32>> {
		let (bytes, x) = le_i32(bytes)?;
		let (bytes, y) = le_i32(bytes)?;
		Ok((bytes, Vec2::new(x, y)))
	}
}

#[async_trait(?Send)]
impl Write for Vec2<i32> {
	async fn write<W: io::Write + std::marker::Unpin>(&self, writer: &mut W) -> io::Result<usize> {
		use async_std::io::prelude::WriteExt;
		writer.write(&self.x.to_le_bytes()).await?;
		writer.write(&self.y.to_le_bytes()).await?;
		Ok(8)
	}
}

#[cfg(test)]
mod tests {
	use super::*;
	use async_std::task;

	#[test]
	fn string_parse() {
		let string: String = "Foobar".into();
		let mut buffer: io::Cursor<Vec<u8>> = io::Cursor::new(Vec::new());

		let size = task::block_on(string.write(&mut buffer)).expect("Could not write");
		assert_eq!(buffer.get_ref().len(), size);
		assert_eq!(
			buffer.get_ref(),
			&vec![6u8, 0, 0, 0, 70, 111, 111, 98, 97, 114]
		);

		let (_, string2) = String::parse(&buffer.get_ref()).expect("Could not parse");
		assert_eq!(string2, string);
	}

	#[test]
	fn uuid_parse() {
		let id = Uuid::parse_str("99d59b4f-1ab8-4103-ba3c-61f4d68a9c48").unwrap();
		let mut buffer: io::Cursor<Vec<u8>> = io::Cursor::new(Vec::new());

		let size = task::block_on(id.write(&mut buffer)).expect("Could not write");
		assert_eq!(buffer.get_ref().len(), size);
		assert_eq!(
			buffer.get_ref(),
			&vec![153u8, 213, 155, 79, 26, 184, 65, 3, 186, 60, 97, 244, 214, 138, 156, 72]
		);

		let (_, id2) = Uuid::parse(&buffer.get_ref()).expect("Could not parse");
		assert_eq!(id2, id);
	}

	#[test]
	fn rect_parse() {
		let rect: Rect<i32, i32> = Rect::new(1, 2, 3, 4);
		let mut buffer: io::Cursor<Vec<u8>> = io::Cursor::new(Vec::new());

		let size = task::block_on(rect.write(&mut buffer)).expect("Could not write");
		assert_eq!(buffer.get_ref().len(), size);
		assert_eq!(
			buffer.get_ref(),
			&vec![1u8, 0, 0, 0, 2, 0, 0, 0, 3, 0, 0, 0, 4, 0, 0, 0]
		);

		let (_, rect2) = Rect::<i32, i32>::parse(&buffer.get_ref()).expect("Could not parse");
		assert_eq!(rect2, rect);
	}

	#[test]
	fn vec_parse() {
		let vec: Vec2<i32> = Vec2::new(1, 2);
		let mut buffer: io::Cursor<Vec<u8>> = io::Cursor::new(Vec::new());

		let size = task::block_on(vec.write(&mut buffer)).expect("Could not write");
		assert_eq!(buffer.get_ref().len(), size);
		assert_eq!(buffer.get_ref(), &vec![1u8, 0, 0, 0, 2, 0, 0, 0]);

		let (_, vec2) = Vec2::<i32>::parse(&buffer.get_ref()).expect("Could not parse");
		assert_eq!(vec2, vec);
	}
}

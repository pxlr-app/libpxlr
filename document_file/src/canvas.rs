use crate::{Parse, Write};
use async_std::io;
use async_trait::async_trait;
use bitvec::{order::Lsb0, vec::BitVec};
use canvas::{Canvas, Stencil};
use color::Channel;
use nom::{
	multi::many_m_n,
	number::complete::{le_u32, le_u8},
	IResult,
};
use std::sync::Arc;
use vek::geom::repr_c::Rect;

impl Parse for Stencil {
	fn parse(bytes: &[u8]) -> IResult<&[u8], Stencil> {
		// Parse bounds
		let (bytes, bounds) = Rect::<i32, i32>::parse(bytes)?;
		// Parse mask
		let len = (((bounds.w * bounds.h) + 8 - 1) / 8) as usize;
		let (bytes, buffer) = many_m_n(len, len, le_u8)(bytes)?;
		let mask: BitVec<Lsb0, u8> = BitVec::from_vec(buffer);
		// Parse channel
		let (bytes, channel) = Channel::parse(bytes)?;
		// Parse data
		let len = mask.count_ones() * channel.pixel_stride();
		let (bytes, data) = many_m_n(len, len, le_u8)(bytes)?;
		Ok((bytes, unsafe {
			Stencil::from_raw_parts(bounds, mask, channel, data)
		}))
	}
}

#[async_trait(?Send)]
impl Write for Stencil {
	async fn write<W: io::Write + std::marker::Unpin>(&self, writer: &mut W) -> io::Result<usize> {
		use async_std::io::prelude::WriteExt;
		let mut size = 17;
		// Write bounds
		self.bounds().write(writer).await?;
		// Write mask
		let buffer = self.mask().as_slice();
		writer.write_all(&buffer).await?;
		size += buffer.len();
		// Write channel
		self.channel().write(writer).await?;
		// Write data
		let buffer = self.data().as_slice();
		writer.write_all(&buffer).await?;
		size += buffer.len();
		Ok(size)
	}
}

impl Parse for Canvas {
	fn parse(bytes: &[u8]) -> IResult<&[u8], Canvas> {
		// Parse channel
		let (bytes, channel) = Channel::parse(bytes)?;
		// Parse stencils
		let (bytes, len) = le_u32(bytes)?;
		let (bytes, stencils) = many_m_n(len as usize, len as usize, Stencil::parse)(bytes)?;
		let stencils = stencils.into_iter().map(|s| Arc::new(s)).collect();
		Ok((bytes, unsafe { Canvas::from_raw_parts(channel, stencils) }))
	}
}

#[async_trait(?Send)]
impl Write for Canvas {
	async fn write<W: io::Write + std::marker::Unpin>(&self, writer: &mut W) -> io::Result<usize> {
		use async_std::io::prelude::WriteExt;
		let mut size = 5;
		// Write channel
		self.channel().write(writer).await?;
		// Write stencils
		let stencils = self.stencils();
		writer
			.write_all(&(stencils.len() as u32).to_le_bytes())
			.await?;
		for stencil in stencils {
			size += stencil.write(writer).await?;
		}
		Ok(size)
	}
}

#[cfg(test)]
mod tests {
	use super::*;
	use async_std::task;

	#[test]
	fn stencil_parse() {
		let stencil = Stencil::from_buffer(
			Rect::new(0, 0, 4, 4),
			Channel::Lumaa,
			vec![
				1, 255, 2, 255, 3, 255, 4, 255, 5, 255, 6, 255, 7, 255, 8, 255, 9, 255, 10, 255,
				11, 255, 12, 255, 13, 255, 14, 255, 15, 255, 16, 255,
			],
		);
		let mut buffer: io::Cursor<Vec<u8>> = io::Cursor::new(Vec::new());

		let size = task::block_on(stencil.write(&mut buffer)).expect("Could not write");
		assert_eq!(buffer.get_ref().len(), size);
		assert_eq!(
			buffer.get_ref(),
			&vec![
				0, 0, 0, 0, 0, 0, 0, 0, 4, 0, 0, 0, 4, 0, 0, 0, 255, 255, 1, 1, 255, 2, 255, 3,
				255, 4, 255, 5, 255, 6, 255, 7, 255, 8, 255, 9, 255, 10, 255, 11, 255, 12, 255, 13,
				255, 14, 255, 15, 255, 16, 255
			]
		);

		let (_, stencil2) = Stencil::parse(&buffer.get_ref()).expect("Could not parse");
		assert_eq!(stencil2.channel(), stencil.channel());
		assert_eq!(stencil2.data(), stencil.data());
	}

	#[test]
	fn canvas_parse() {
		let canvas = Canvas::from_stencil(Stencil::from_buffer(
			Rect::new(0, 0, 4, 4),
			Channel::Lumaa,
			vec![
				1, 255, 2, 255, 3, 255, 4, 255, 5, 255, 6, 255, 7, 255, 8, 255, 9, 255, 10, 255,
				11, 255, 12, 255, 13, 255, 14, 255, 15, 255, 16, 255,
			],
		));
		let mut buffer: io::Cursor<Vec<u8>> = io::Cursor::new(Vec::new());

		let size = task::block_on(canvas.write(&mut buffer)).expect("Could not write");
		assert_eq!(buffer.get_ref().len(), size);
		assert_eq!(
			buffer.get_ref(),
			&vec![
				1, 1, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 4, 0, 0, 0, 4, 0, 0, 0, 255, 255, 1, 1, 255,
				2, 255, 3, 255, 4, 255, 5, 255, 6, 255, 7, 255, 8, 255, 9, 255, 10, 255, 11, 255,
				12, 255, 13, 255, 14, 255, 15, 255, 16, 255
			]
		);

		let (_, canvas2) = Canvas::parse(&buffer.get_ref()).expect("Could not parse");
		assert_eq!(canvas2.channel(), canvas.channel());
		let pixels: Vec<_> = canvas.iter().flatten().map(|b| *b).collect();
		let pixels2: Vec<_> = canvas2.iter().flatten().map(|b| *b).collect();
		assert_eq!(pixels, pixels2);
	}
}

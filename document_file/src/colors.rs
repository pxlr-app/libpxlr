use crate::{Parse, Write};
use color::*;
use nom::{
	number::complete::{le_f32, le_u8},
	IResult,
};
use std::io;

impl Parse for Luma {
	fn parse(bytes: &[u8]) -> IResult<&[u8], Luma> {
		let (bytes, luma) = le_u8(bytes)?;
		Ok((bytes, Luma::new(luma)))
	}
}

impl Write for Luma {
	fn write(&self, writer: &mut dyn io::Write) -> io::Result<usize> {
		writer.write_all(&self.luma.to_le_bytes())?;
		Ok(1)
	}
}

impl Parse for Lumaa {
	fn parse(bytes: &[u8]) -> IResult<&[u8], Lumaa> {
		let (bytes, color) = Luma::parse(bytes)?;
		let (bytes, alpha) = le_u8(bytes)?;
		Ok((bytes, Lumaa::new(color, alpha)))
	}
}

impl Write for Lumaa {
	fn write(&self, writer: &mut dyn io::Write) -> io::Result<usize> {
		let size = self.color.write(writer)?;
		writer.write_all(&self.alpha.to_le_bytes())?;
		Ok(1 + size)
	}
}

impl Parse for Rgb {
	fn parse(bytes: &[u8]) -> IResult<&[u8], Rgb> {
		let (bytes, red) = le_u8(bytes)?;
		let (bytes, green) = le_u8(bytes)?;
		let (bytes, blue) = le_u8(bytes)?;
		Ok((bytes, Rgb::new(red, green, blue)))
	}
}

impl Write for Rgb {
	fn write(&self, writer: &mut dyn io::Write) -> io::Result<usize> {
		writer.write_all(&self.red.to_le_bytes())?;
		writer.write_all(&self.green.to_le_bytes())?;
		writer.write_all(&self.blue.to_le_bytes())?;
		Ok(3)
	}
}

impl Parse for Rgba {
	fn parse(bytes: &[u8]) -> IResult<&[u8], Rgba> {
		let (bytes, color) = Rgb::parse(bytes)?;
		let (bytes, alpha) = le_u8(bytes)?;
		Ok((bytes, Rgba::new(color, alpha)))
	}
}

impl Write for Rgba {
	fn write(&self, writer: &mut dyn io::Write) -> io::Result<usize> {
		let size = self.color.write(writer)?;
		writer.write_all(&self.alpha.to_le_bytes())?;
		Ok(1 + size)
	}
}

impl Parse for Uv {
	fn parse(bytes: &[u8]) -> IResult<&[u8], Uv> {
		let (bytes, u) = le_f32(bytes)?;
		let (bytes, v) = le_f32(bytes)?;
		Ok((bytes, Uv::new(u, v)))
	}
}

impl Write for Uv {
	fn write(&self, writer: &mut dyn io::Write) -> io::Result<usize> {
		writer.write_all(&self.u.to_le_bytes())?;
		writer.write_all(&self.v.to_le_bytes())?;
		Ok(8)
	}
}

impl Parse for Normal {
	fn parse(bytes: &[u8]) -> IResult<&[u8], Normal> {
		let (bytes, x) = le_f32(bytes)?;
		let (bytes, y) = le_f32(bytes)?;
		let (bytes, z) = le_f32(bytes)?;
		Ok((bytes, Normal::new(x, y, z)))
	}
}

impl Write for Normal {
	fn write(&self, writer: &mut dyn io::Write) -> io::Result<usize> {
		writer.write_all(&self.x.to_le_bytes())?;
		writer.write_all(&self.y.to_le_bytes())?;
		writer.write_all(&self.z.to_le_bytes())?;
		Ok(12)
	}
}

impl Parse for Channel {
	fn parse(bytes: &[u8]) -> IResult<&[u8], Channel> {
		let (bytes, id) = le_u8(bytes)?;
		let channel = match id {
			0 => Channel::Luma,
			1 => Channel::Lumaa,
			2 => Channel::Rgb,
			3 => Channel::Rgba,
			4 => Channel::Uv,
			5 => Channel::Normal,
			6 => Channel::LumaNormal,
			7 => Channel::LumaaNormal,
			8 => Channel::RgbNormal,
			9 => Channel::RgbaNormal,
			_ => {
				return Err(nom::Err::Error(nom::error_position!(
					bytes,
					nom::error::ErrorKind::Complete
				)))
			}
		};
		Ok((bytes, channel))
	}
}

impl Write for Channel {
	fn write(&self, writer: &mut dyn io::Write) -> io::Result<usize> {
		let id: u8 = match self {
			Channel::Luma => 0,
			Channel::Lumaa => 1,
			Channel::Rgb => 2,
			Channel::Rgba => 3,
			Channel::Uv => 4,
			Channel::Normal => 5,
			Channel::LumaNormal => 6,
			Channel::LumaaNormal => 7,
			Channel::RgbNormal => 8,
			Channel::RgbaNormal => 9,
		};
		writer.write_all(&id.to_le_bytes())?;
		Ok(1)
	}
}

#[cfg(test)]
mod tests {
	use super::*;

	#[test]
	fn luma_parse() {
		let color = Luma::new(32);
		let mut buffer: io::Cursor<Vec<u8>> = io::Cursor::new(Vec::new());

		let size = color.write(&mut buffer).expect("Could not write");
		assert_eq!(buffer.get_ref().len(), size);
		assert_eq!(buffer.get_ref(), &vec![32]);

		let (_, color2) = Luma::parse(&buffer.get_ref()).expect("Could not parse");
		assert_eq!(color2, color);
	}

	#[test]
	fn lumaa_parse() {
		let color = Lumaa::new(Luma::new(32), 128);
		let mut buffer: io::Cursor<Vec<u8>> = io::Cursor::new(Vec::new());

		let size = color.write(&mut buffer).expect("Could not write");
		assert_eq!(buffer.get_ref().len(), size);
		assert_eq!(buffer.get_ref(), &vec![32, 128]);

		let (_, color2) = Lumaa::parse(&buffer.get_ref()).expect("Could not parse");
		assert_eq!(color2, color);
	}

	#[test]
	fn rgb_parse() {
		let color = Rgb::new(32, 64, 128);
		let mut buffer: io::Cursor<Vec<u8>> = io::Cursor::new(Vec::new());

		let size = color.write(&mut buffer).expect("Could not write");
		assert_eq!(buffer.get_ref().len(), size);
		assert_eq!(buffer.get_ref(), &vec![32, 64, 128]);

		let (_, color2) = Rgb::parse(&buffer.get_ref()).expect("Could not parse");
		assert_eq!(color2, color);
	}

	#[test]
	fn rgba_parse() {
		let color = Rgba::new(Rgb::new(32, 64, 128), 196);
		let mut buffer: io::Cursor<Vec<u8>> = io::Cursor::new(Vec::new());

		let size = color.write(&mut buffer).expect("Could not write");
		assert_eq!(buffer.get_ref().len(), size);
		assert_eq!(buffer.get_ref(), &vec![32, 64, 128, 196]);

		let (_, color2) = Rgba::parse(&buffer.get_ref()).expect("Could not parse");
		assert_eq!(color2, color);
	}

	#[test]
	fn uv_parse() {
		let color = Uv::new(0.2, 0.8);
		let mut buffer: io::Cursor<Vec<u8>> = io::Cursor::new(Vec::new());

		let size = color.write(&mut buffer).expect("Could not write");
		assert_eq!(buffer.get_ref().len(), size);
		assert_eq!(buffer.get_ref(), &vec![205, 204, 76, 62, 205, 204, 76, 63]);

		let (_, color2) = Uv::parse(&buffer.get_ref()).expect("Could not parse");
		assert_eq!(color2, color);
	}

	#[test]
	fn normal_parse() {
		let color = Normal::new(0.2, 0.5, 0.8);
		let mut buffer: io::Cursor<Vec<u8>> = io::Cursor::new(Vec::new());

		let size = color.write(&mut buffer).expect("Could not write");
		assert_eq!(buffer.get_ref().len(), size);
		assert_eq!(
			buffer.get_ref(),
			&vec![205, 204, 76, 62, 0, 0, 0, 63, 205, 204, 76, 63]
		);

		let (_, color2) = Normal::parse(&buffer.get_ref()).expect("Could not parse");
		assert_eq!(color2, color);
	}

	#[test]
	fn channel_parse() {
		fn assert_channel(channel: Channel, id: u8) {
			let mut buffer: io::Cursor<Vec<u8>> = io::Cursor::new(Vec::new());
			let size = channel.write(&mut buffer).expect("Could not write");
			assert_eq!(buffer.get_ref().len(), size);

			assert_eq!(buffer.get_ref(), &vec![id]);

			let (_, channel2) = Channel::parse(&buffer.get_ref()).expect("Could not parse");
			assert_eq!(channel2, channel);
		}

		assert_channel(Channel::Luma, 0);
		assert_channel(Channel::Lumaa, 1);
		assert_channel(Channel::Rgb, 2);
		assert_channel(Channel::Rgba, 3);
		assert_channel(Channel::Uv, 4);
		assert_channel(Channel::Normal, 5);
		assert_channel(Channel::LumaNormal, 6);
		assert_channel(Channel::LumaaNormal, 7);
		assert_channel(Channel::RgbNormal, 8);
		assert_channel(Channel::RgbaNormal, 9);
	}
}

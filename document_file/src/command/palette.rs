use crate::{Parse, Write};
use async_std::io;
use async_trait::async_trait;
use color::Rgba;
use document_command::{
	AddPaletteColorCommand, Command, MovePaletteColorCommand, RemovePaletteColorCommand,
};
use nom::{number::complete::le_u32, IResult};
use uuid::Uuid;

impl Parse for AddPaletteColorCommand {
	fn parse(bytes: &[u8]) -> IResult<&[u8], AddPaletteColorCommand> {
		let (bytes, target) = Uuid::parse(bytes)?;
		let (bytes, color) = Rgba::parse(bytes)?;
		Ok((bytes, AddPaletteColorCommand::new(target, color)))
	}
}

#[async_trait(?Send)]
impl Write for AddPaletteColorCommand {
	async fn write<W: io::Write + std::marker::Unpin>(&self, writer: &mut W) -> io::Result<usize> {
		let mut size = self.target().write(writer).await?;
		size += self.color().write(writer).await?;
		Ok(size)
	}
}

impl Parse for RemovePaletteColorCommand {
	fn parse(bytes: &[u8]) -> IResult<&[u8], RemovePaletteColorCommand> {
		let (bytes, target) = Uuid::parse(bytes)?;
		let (bytes, color) = Rgba::parse(bytes)?;
		Ok((bytes, RemovePaletteColorCommand::new(target, color)))
	}
}

#[async_trait(?Send)]
impl Write for RemovePaletteColorCommand {
	async fn write<W: io::Write + std::marker::Unpin>(&self, writer: &mut W) -> io::Result<usize> {
		let mut size = self.target().write(writer).await?;
		size += self.color().write(writer).await?;
		Ok(size)
	}
}

impl Parse for MovePaletteColorCommand {
	fn parse(bytes: &[u8]) -> IResult<&[u8], MovePaletteColorCommand> {
		let (bytes, target) = Uuid::parse(bytes)?;
		let (bytes, color) = Rgba::parse(bytes)?;
		let (bytes, position) = le_u32(bytes)?;
		Ok((
			bytes,
			MovePaletteColorCommand::new(target, color, position as usize),
		))
	}
}

#[async_trait(?Send)]
impl Write for MovePaletteColorCommand {
	async fn write<W: io::Write + std::marker::Unpin>(&self, writer: &mut W) -> io::Result<usize> {
		use async_std::io::prelude::WriteExt;
		let mut size = self.target().write(writer).await?;
		size += self.color().write(writer).await?;
		writer
			.write_all(&(*self.position() as u32).to_le_bytes())
			.await?;
		Ok(size + 4)
	}
}

#[cfg(test)]
mod tests {
	use super::*;
	use async_std::task;
	use color::Rgb;

	#[test]
	fn addpalettecolorcommand_parse() {
		let cmd = AddPaletteColorCommand::new(Uuid::new_v4(), Rgba::new(Rgb::new(1, 2, 3), 4));
		let mut buffer: io::Cursor<Vec<u8>> = io::Cursor::new(Vec::new());

		let size = task::block_on(cmd.write(&mut buffer)).expect("Could not write");
		assert_eq!(buffer.get_ref().len(), size);

		let (_, cmd2) = AddPaletteColorCommand::parse(&buffer.get_ref()).expect("Could not parse");
		assert_eq!(cmd2, cmd);
	}

	#[test]
	fn removepalettecolorcommand_parse() {
		let cmd = RemovePaletteColorCommand::new(Uuid::new_v4(), Rgba::new(Rgb::new(1, 2, 3), 4));
		let mut buffer: io::Cursor<Vec<u8>> = io::Cursor::new(Vec::new());

		let size = task::block_on(cmd.write(&mut buffer)).expect("Could not write");
		assert_eq!(buffer.get_ref().len(), size);

		let (_, cmd2) =
			RemovePaletteColorCommand::parse(&buffer.get_ref()).expect("Could not parse");
		assert_eq!(cmd2, cmd);
	}

	#[test]
	fn movepalettecolorcommand_parse() {
		let cmd = MovePaletteColorCommand::new(Uuid::new_v4(), Rgba::new(Rgb::new(1, 2, 3), 4), 1);
		let mut buffer: io::Cursor<Vec<u8>> = io::Cursor::new(Vec::new());

		let size = task::block_on(cmd.write(&mut buffer)).expect("Could not write");
		assert_eq!(buffer.get_ref().len(), size);

		let (_, cmd2) = MovePaletteColorCommand::parse(&buffer.get_ref()).expect("Could not parse");
		assert_eq!(cmd2, cmd);
	}
}

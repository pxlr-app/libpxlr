use crate::{Parse, Write};
use async_std::io;
use async_trait::async_trait;
use document_command::CommandType;
use nom::{number::complete::le_u16, IResult};

mod group;
mod note;
mod palette;
mod rename;
mod translate;
mod unloaded;

pub trait CommandId {
	fn command_id(&self) -> u16;
}

impl CommandId for CommandType {
	fn command_id(&self) -> u16 {
		match self {
			CommandType::LoadNode(_) => 0,
			CommandType::UnloadNode(_) => 1,
			CommandType::AddChild(_) => 2,
			CommandType::MoveChild(_) => 3,
			CommandType::RemoveChild(_) => 4,
			CommandType::Rename(_) => 5,
			CommandType::SetNoteContent(_) => 6,
			CommandType::Translate(_) => 7,
			CommandType::AddPaletteColor(_) => 8,
			CommandType::MovePaletteColor(_) => 9,
			CommandType::RemovePaletteColor(_) => 10,
		}
	}
}

impl Parse for CommandType {
	fn parse(bytes: &[u8]) -> IResult<&[u8], CommandType> {
		let (bytes, command_id) = le_u16(bytes)?;
		let (bytes, command_type) = match command_id {
			0 => document_command::LoadNodeCommand::parse(bytes)
				.map(|(bytes, cmd)| (bytes, CommandType::LoadNode(cmd))),
			1 => document_command::UnloadNodeCommand::parse(bytes)
				.map(|(bytes, cmd)| (bytes, CommandType::UnloadNode(cmd))),
			2 => document_command::AddChildCommand::parse(bytes)
				.map(|(bytes, cmd)| (bytes, CommandType::AddChild(cmd))),
			3 => document_command::MoveChildCommand::parse(bytes)
				.map(|(bytes, cmd)| (bytes, CommandType::MoveChild(cmd))),
			4 => document_command::RemoveChildCommand::parse(bytes)
				.map(|(bytes, cmd)| (bytes, CommandType::RemoveChild(cmd))),
			5 => document_command::RenameCommand::parse(bytes)
				.map(|(bytes, cmd)| (bytes, CommandType::Rename(cmd))),
			6 => document_command::SetNoteContentCommand::parse(bytes)
				.map(|(bytes, cmd)| (bytes, CommandType::SetNoteContent(cmd))),
			7 => document_command::TranslateCommand::parse(bytes)
				.map(|(bytes, cmd)| (bytes, CommandType::Translate(cmd))),
			8 => document_command::AddPaletteColorCommand::parse(bytes)
				.map(|(bytes, cmd)| (bytes, CommandType::AddPaletteColor(cmd))),
			9 => document_command::MovePaletteColorCommand::parse(bytes)
				.map(|(bytes, cmd)| (bytes, CommandType::MovePaletteColor(cmd))),
			10 => document_command::RemovePaletteColorCommand::parse(bytes)
				.map(|(bytes, cmd)| (bytes, CommandType::RemovePaletteColor(cmd))),
			_ => unreachable!(),
		}?;
		Ok((bytes, command_type))
	}
}

#[async_trait(?Send)]
impl Write for CommandType {
	async fn write<W: io::Write + std::marker::Unpin>(&self, writer: &mut W) -> io::Result<usize> {
		use async_std::io::prelude::WriteExt;
		let size = match self {
			CommandType::LoadNode(cmd) => {
				writer.write_all(&0u16.to_le_bytes()).await?;
				cmd.write(writer).await?
			}
			CommandType::UnloadNode(cmd) => {
				writer.write_all(&1u16.to_le_bytes()).await?;
				cmd.write(writer).await?
			}
			CommandType::AddChild(cmd) => {
				writer.write_all(&2u16.to_le_bytes()).await?;
				cmd.write(writer).await?
			}
			CommandType::MoveChild(cmd) => {
				writer.write_all(&3u16.to_le_bytes()).await?;
				cmd.write(writer).await?
			}
			CommandType::RemoveChild(cmd) => {
				writer.write_all(&4u16.to_le_bytes()).await?;
				cmd.write(writer).await?
			}
			CommandType::Rename(cmd) => {
				writer.write_all(&5u16.to_le_bytes()).await?;
				cmd.write(writer).await?
			}
			CommandType::SetNoteContent(cmd) => {
				writer.write_all(&6u16.to_le_bytes()).await?;
				cmd.write(writer).await?
			}
			CommandType::Translate(cmd) => {
				writer.write_all(&7u16.to_le_bytes()).await?;
				cmd.write(writer).await?
			}
			CommandType::AddPaletteColor(cmd) => {
				writer.write_all(&8u16.to_le_bytes()).await?;
				cmd.write(writer).await?
			}
			CommandType::MovePaletteColor(cmd) => {
				writer.write_all(&9u16.to_le_bytes()).await?;
				cmd.write(writer).await?
			}
			CommandType::RemovePaletteColor(cmd) => {
				writer.write_all(&10u16.to_le_bytes()).await?;
				cmd.write(writer).await?
			}
		};
		Ok(size + 2)
	}
}

#[cfg(test)]
mod tests {
	use super::*;
	use async_std::task;
	use document_command::RenameCommand;
	use uuid::Uuid;

	#[test]
	fn commandtype_parse() {
		let cmd = CommandType::Rename(RenameCommand::new(Uuid::new_v4(), "Foo"));
		let mut buffer: io::Cursor<Vec<u8>> = io::Cursor::new(Vec::new());

		let size = task::block_on(cmd.write(&mut buffer)).expect("Could not write");
		assert_eq!(buffer.get_ref().len(), size);

		let (_, cmd2) = CommandType::parse(&buffer.get_ref()).expect("Could not parse");
		assert_eq!(cmd2, cmd);
	}
}

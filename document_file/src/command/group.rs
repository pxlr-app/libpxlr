use std::sync::Arc;

use crate::{Parse, Write};
use async_std::io;
use async_trait::async_trait;
use document_command::{AddChildCommand, Command, MoveChildCommand, RemoveChildCommand};
use document_core::NodeType;
use nom::{number::complete::le_u32, IResult};
use uuid::Uuid;

impl Parse for AddChildCommand {
	fn parse(bytes: &[u8]) -> IResult<&[u8], AddChildCommand> {
		let (bytes, target) = Uuid::parse(bytes)?;
		let (bytes, child) = <Arc<NodeType> as Parse>::parse(bytes)?;
		Ok((bytes, AddChildCommand::new(target, child)))
	}
}

#[async_trait(?Send)]
impl Write for AddChildCommand {
	async fn write<W: io::Write + std::marker::Unpin>(&self, writer: &mut W) -> io::Result<usize> {
		let mut size = self.target().write(writer).await?;
		size += self.child().write(writer).await?;
		Ok(size)
	}
}

impl Parse for RemoveChildCommand {
	fn parse(bytes: &[u8]) -> IResult<&[u8], RemoveChildCommand> {
		let (bytes, target) = Uuid::parse(bytes)?;
		let (bytes, child_id) = Uuid::parse(bytes)?;
		Ok((bytes, RemoveChildCommand::new(target, child_id)))
	}
}

#[async_trait(?Send)]
impl Write for RemoveChildCommand {
	async fn write<W: io::Write + std::marker::Unpin>(&self, writer: &mut W) -> io::Result<usize> {
		let mut size = self.target().write(writer).await?;
		size += self.child_id().write(writer).await?;
		Ok(size)
	}
}

impl Parse for MoveChildCommand {
	fn parse(bytes: &[u8]) -> IResult<&[u8], MoveChildCommand> {
		let (bytes, target) = Uuid::parse(bytes)?;
		let (bytes, child_id) = Uuid::parse(bytes)?;
		let (bytes, position) = le_u32(bytes)?;
		Ok((
			bytes,
			MoveChildCommand::new(target, child_id, position as usize),
		))
	}
}

#[async_trait(?Send)]
impl Write for MoveChildCommand {
	async fn write<W: io::Write + std::marker::Unpin>(&self, writer: &mut W) -> io::Result<usize> {
		use async_std::io::prelude::WriteExt;
		let mut size = self.target().write(writer).await?;
		size += self.child_id().write(writer).await?;
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
	use document_core::Note;

	#[test]
	fn addchildcommand_parse() {
		let cmd = AddChildCommand::new(
			Uuid::new_v4(),
			Arc::new(NodeType::Note(Note::new("My note", (0, 0), ""))),
		);
		let mut buffer: io::Cursor<Vec<u8>> = io::Cursor::new(Vec::new());

		let size = task::block_on(cmd.write(&mut buffer)).expect("Could not write");
		assert_eq!(buffer.get_ref().len(), size);

		let (_, cmd2) = AddChildCommand::parse(&buffer.get_ref()).expect("Could not parse");
		assert_eq!(cmd2, cmd);
	}

	#[test]
	fn removechildcommand_parse() {
		let cmd = RemoveChildCommand::new(Uuid::new_v4(), Uuid::new_v4());
		let mut buffer: io::Cursor<Vec<u8>> = io::Cursor::new(Vec::new());

		let size = task::block_on(cmd.write(&mut buffer)).expect("Could not write");
		assert_eq!(buffer.get_ref().len(), size);

		let (_, cmd2) = RemoveChildCommand::parse(&buffer.get_ref()).expect("Could not parse");
		assert_eq!(cmd2, cmd);
	}

	#[test]
	fn movechildcommand_parse() {
		let cmd = MoveChildCommand::new(Uuid::new_v4(), Uuid::new_v4(), 1);
		let mut buffer: io::Cursor<Vec<u8>> = io::Cursor::new(Vec::new());

		let size = task::block_on(cmd.write(&mut buffer)).expect("Could not write");
		assert_eq!(buffer.get_ref().len(), size);

		let (_, cmd2) = MoveChildCommand::parse(&buffer.get_ref()).expect("Could not parse");
		assert_eq!(cmd2, cmd);
	}
}

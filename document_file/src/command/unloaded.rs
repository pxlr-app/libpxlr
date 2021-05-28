use crate::{Parse, Write};
use async_std::io;
use async_trait::async_trait;
use document_command::{Command, LoadNodeCommand, UnloadNodeCommand};
use document_core::NodeType;
use nom::IResult;
use std::sync::Arc;
use uuid::Uuid;

impl Parse for LoadNodeCommand {
	fn parse(bytes: &[u8]) -> IResult<&[u8], LoadNodeCommand> {
		let (bytes, target) = Uuid::parse(bytes)?;
		let (bytes, node) = <Arc<NodeType> as Parse>::parse(bytes)?;
		Ok((bytes, LoadNodeCommand::new(target, node)))
	}
}

#[async_trait(?Send)]
impl Write for LoadNodeCommand {
	async fn write<W: io::Write + std::marker::Unpin>(&self, writer: &mut W) -> io::Result<usize> {
		let mut size = self.target().write(writer).await?;
		size += self.node().write(writer).await?;
		Ok(size)
	}
}

impl Parse for UnloadNodeCommand {
	fn parse(bytes: &[u8]) -> IResult<&[u8], UnloadNodeCommand> {
		let (bytes, target) = Uuid::parse(bytes)?;
		Ok((bytes, UnloadNodeCommand::new(target)))
	}
}

#[async_trait(?Send)]
impl Write for UnloadNodeCommand {
	async fn write<W: io::Write + std::marker::Unpin>(&self, writer: &mut W) -> io::Result<usize> {
		let size = self.target().write(writer).await?;
		Ok(size)
	}
}

#[cfg(test)]
mod tests {
	use super::*;
	use async_std::task;
	use document_core::{Node, Note};

	#[test]
	fn loadcommand_parse() {
		let node = Arc::new(NodeType::Note(Note::new("My note", (0, 0), "")));
		let cmd = LoadNodeCommand::new(*node.id(), node);
		let mut buffer: io::Cursor<Vec<u8>> = io::Cursor::new(Vec::new());

		let size = task::block_on(cmd.write(&mut buffer)).expect("Could not write");
		assert_eq!(buffer.get_ref().len(), size);

		let (_, cmd2) = LoadNodeCommand::parse(&buffer.get_ref()).expect("Could not parse");
		assert_eq!(cmd2, cmd);
	}

	#[test]
	fn unloadcommand_parse() {
		let cmd = UnloadNodeCommand::new(Uuid::new_v4());
		let mut buffer: io::Cursor<Vec<u8>> = io::Cursor::new(Vec::new());

		let size = task::block_on(cmd.write(&mut buffer)).expect("Could not write");
		assert_eq!(buffer.get_ref().len(), size);

		let (_, cmd2) = UnloadNodeCommand::parse(&buffer.get_ref()).expect("Could not parse");
		assert_eq!(cmd2, cmd);
	}
}

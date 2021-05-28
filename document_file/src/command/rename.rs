use crate::{Parse, Write};
use async_std::io;
use async_trait::async_trait;
use document_command::{Command, RenameCommand};
use nom::IResult;
use uuid::Uuid;

impl Parse for RenameCommand {
	fn parse(bytes: &[u8]) -> IResult<&[u8], RenameCommand> {
		let (bytes, target) = Uuid::parse(bytes)?;
		let (bytes, name) = String::parse(bytes)?;
		Ok((bytes, RenameCommand::new(target, name)))
	}
}

#[async_trait(?Send)]
impl Write for RenameCommand {
	async fn write<W: io::Write + std::marker::Unpin>(&self, writer: &mut W) -> io::Result<usize> {
		let mut size = self.target().write(writer).await?;
		size += self.name().write(writer).await?;
		Ok(size)
	}
}

#[cfg(test)]
mod tests {
	use super::*;
	use async_std::task;

	#[test]
	fn renamecommand_parse() {
		let cmd = RenameCommand::new(Uuid::new_v4(), "Foo");
		let mut buffer: io::Cursor<Vec<u8>> = io::Cursor::new(Vec::new());

		let size = task::block_on(cmd.write(&mut buffer)).expect("Could not write");
		assert_eq!(buffer.get_ref().len(), size);

		let (_, cmd2) = RenameCommand::parse(&buffer.get_ref()).expect("Could not parse");
		assert_eq!(cmd2, cmd);
	}
}

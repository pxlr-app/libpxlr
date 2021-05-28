use crate::{Parse, Write};
use async_std::io;
use async_trait::async_trait;
use document_command::{Command, SetNoteContentCommand};
use nom::IResult;
use uuid::Uuid;

impl Parse for SetNoteContentCommand {
	fn parse(bytes: &[u8]) -> IResult<&[u8], SetNoteContentCommand> {
		let (bytes, target) = Uuid::parse(bytes)?;
		let (bytes, content) = String::parse(bytes)?;
		Ok((bytes, SetNoteContentCommand::new(target, content)))
	}
}

#[async_trait(?Send)]
impl Write for SetNoteContentCommand {
	async fn write<W: io::Write + std::marker::Unpin>(&self, writer: &mut W) -> io::Result<usize> {
		let mut size = self.target().write(writer).await?;
		size += self.content().write(writer).await?;
		Ok(size)
	}
}

#[cfg(test)]
mod tests {
	use super::*;
	use async_std::task;

	#[test]
	fn setnotecommand_parse() {
		let cmd = SetNoteContentCommand::new(Uuid::new_v4(), "Content");
		let mut buffer: io::Cursor<Vec<u8>> = io::Cursor::new(Vec::new());

		let size = task::block_on(cmd.write(&mut buffer)).expect("Could not write");
		assert_eq!(buffer.get_ref().len(), size);

		let (_, cmd2) = SetNoteContentCommand::parse(&buffer.get_ref()).expect("Could not parse");
		assert_eq!(cmd2, cmd);
	}
}

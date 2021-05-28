use crate::{Parse, Write};
use async_std::io;
use async_trait::async_trait;
use document_command::{Command, TranslateCommand};
use nom::IResult;
use uuid::Uuid;
use vek::vec::repr_c::vec2::Vec2;

impl Parse for TranslateCommand {
	fn parse(bytes: &[u8]) -> IResult<&[u8], TranslateCommand> {
		let (bytes, target) = Uuid::parse(bytes)?;
		let (bytes, position) = Vec2::<i32>::parse(bytes)?;
		Ok((bytes, TranslateCommand::new(target, position)))
	}
}

#[async_trait(?Send)]
impl Write for TranslateCommand {
	async fn write<W: io::Write + std::marker::Unpin>(&self, writer: &mut W) -> io::Result<usize> {
		let mut size = self.target().write(writer).await?;
		size += self.position().write(writer).await?;
		Ok(size)
	}
}

#[cfg(test)]
mod tests {
	use super::*;
	use async_std::task;

	#[test]
	fn translatecommand_parse() {
		let cmd = TranslateCommand::new(Uuid::new_v4(), (0, 0));
		let mut buffer: io::Cursor<Vec<u8>> = io::Cursor::new(Vec::new());

		let size = task::block_on(cmd.write(&mut buffer)).expect("Could not write");
		assert_eq!(buffer.get_ref().len(), size);

		let (_, cmd2) = TranslateCommand::parse(&buffer.get_ref()).expect("Could not parse");
		assert_eq!(cmd2, cmd);
	}
}

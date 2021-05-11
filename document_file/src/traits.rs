use nom::IResult;
use std::io;

pub trait Parse {
	fn parse(bytes: &[u8]) -> IResult<&[u8], Self>
	where
		Self: Sized;
}

pub trait Write {
	fn write(&self, writer: &mut dyn io::Write) -> io::Result<usize>;
}
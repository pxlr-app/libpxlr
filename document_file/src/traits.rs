use async_std::io;
use async_trait::async_trait;
use nom::IResult;

pub trait Parse {
	fn parse(bytes: &[u8]) -> IResult<&[u8], Self>
	where
		Self: Sized;
}

#[async_trait(?Send)]
pub trait Write {
	async fn write<W: io::Write + std::marker::Unpin>(&self, writer: &mut W) -> io::Result<usize>;
}

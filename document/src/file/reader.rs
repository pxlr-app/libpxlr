pub mod v0 {
	use nom::multi::many0;
	use nom::IResult;

	pub trait Reader {
		fn from_bytes(bytes: &[u8]) -> IResult<&[u8], Self>
		where
			Self: Sized;
	}

	impl<T> Reader for Vec<T>
	where
		T: Reader,
	{
		fn from_bytes(bytes: &[u8]) -> IResult<&[u8], Self> {
			many0(<T as Reader>::from_bytes)(bytes)
		}
	}
}

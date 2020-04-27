pub mod v0 {
	use nom::multi::many_m_n;
	use nom::IResult;

	pub trait Reader {
		fn read(bytes: &[u8]) -> IResult<&[u8], Self>
		where
			Self: Sized;
	}

	pub trait ReaderVec {
		fn read_n(bytes: &[u8], n: usize) -> IResult<&[u8], Self>
		where
			Self: Sized;
	}

	impl<T> ReaderVec for Vec<T>
	where
		T: Reader,
	{
		fn read_n(bytes: &[u8], n: usize) -> IResult<&[u8], Self> {
			many_m_n(n, n, <T as Reader>::read)(bytes)
		}
	}
}

pub trait WriteTo {
	fn write_to<W: std::io::Write>(&self, writer: &mut W) -> std::io::Result<usize>;
}

impl<T> WriteTo for Vec<T>
where
	T: WriteTo,
{
	fn write_to<W: std::io::Write>(&self, writer: &mut W) -> std::io::Result<usize> {
		let mut b: usize = 0;
		for t in self.iter() {
			b += t.write_to(writer)?;
		}
		Ok(b)
	}
}

pub trait Writer {
	fn write<W: std::io::Write + std::io::Seek>(
		&self,
		file: &mut crate::file::File,
		writer: &mut W,
	) -> std::io::Result<usize>;
}

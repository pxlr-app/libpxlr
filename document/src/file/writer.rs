use std::io::{Result, Write};

pub trait Writer {
	fn write<W: Write>(&self, writer: &mut W) -> Result<usize>;
}

impl<T> Writer for Vec<T>
where
	T: Writer,
{
	fn write<W: Write>(&self, writer: &mut W) -> std::io::Result<usize> {
		let mut b: usize = 0;
		for t in self.iter() {
			b += t.write(writer)?;
		}
		Ok(b)
	}
}

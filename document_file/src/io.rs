use std::io::{Result, Seek, SeekFrom, Write};
pub struct Void;

impl Write for Void {
	fn write(&mut self, _buf: &[u8]) -> Result<usize> {
		Ok(0)
	}
	fn flush(&mut self) -> Result<()> {
		Ok(())
	}
}

impl Write for &Void {
	fn write(&mut self, _buf: &[u8]) -> Result<usize> {
		Ok(0)
	}
	fn flush(&mut self) -> Result<()> {
		Ok(())
	}
}

impl Seek for Void {
	fn seek(&mut self, _pos: SeekFrom) -> Result<u64> {
		Ok(0)
	}
}

impl Seek for &Void {
	fn seek(&mut self, _pos: SeekFrom) -> Result<u64> {
		Ok(0)
	}
}

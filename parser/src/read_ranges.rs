use super::range_bounds::RangeBounds;
use async_trait::async_trait;
use futures::io;
use std::ops::Bound;

#[async_trait]
pub trait ReadRanges {
	async fn read_ranges(
		&mut self,
		ranges: Vec<Box<dyn RangeBounds<i64> + std::marker::Send>>,
	) -> io::Result<Vec<u8>>;
}

#[async_trait]
impl<T> ReadRanges for T
where
	T: io::AsyncReadExt + io::AsyncSeekExt + std::marker::Send + std::marker::Unpin,
{
	async fn read_ranges(
		&mut self,
		ranges: Vec<Box<dyn RangeBounds<i64> + std::marker::Send>>,
	) -> io::Result<Vec<u8>> {
		let mut seek_sizes: Vec<(io::SeekFrom, u64)> = ranges
			.iter()
			.map(|r| match (r.start_bound(), r.end_bound()) {
				// ..E => first E-1 bytes
				(Bound::Unbounded, Bound::Excluded(e)) => (io::SeekFrom::Start(0), (*e - 1) as u64),
				// ..=E => first E bytes
				(Bound::Unbounded, Bound::Included(e)) => (io::SeekFrom::Start(0), *e as u64),
				// -S.. => last S bytes
				(Bound::Included(s), Bound::Unbounded) if s < &0 => {
					(io::SeekFrom::End(-*s), -*s as u64)
				}
				// S..E => S to E-1 bytes
				(Bound::Included(s), Bound::Excluded(e)) => {
					(io::SeekFrom::Start(*s as u64), (*e - *s - 1) as u64)
				}
				// S..=E => S to E bytes
				(Bound::Included(s), Bound::Included(e)) => {
					(io::SeekFrom::Start(*s as u64), (*e - *s) as u64)
				}
				_ => panic!("Range not supported."),
			})
			.collect();
		let size: u64 = seek_sizes.iter().fold(0, |size, p| size + p.1);
		let mut buffer: Vec<u8> = Vec::with_capacity(size as usize);

		for (seek, size) in seek_sizes.drain(..) {
			let mut buf: Vec<u8> = vec![0; size as usize];
			self.seek(seek).await?;
			self.read_exact(&mut buf[..]).await?;
			buffer.append(&mut buf);
		}

		Ok(buffer)
	}
}

use super::range_bounds::RangeBounds;
use super::read_ranges::ReadRanges;
use async_trait::async_trait;
use futures::io;
use std::ops::Bound;

pub struct Request {
	url: reqwest::Url,
}

impl Request {
	pub fn new<U: std::convert::Into<reqwest::Url>>(url: U) -> Self {
		Request { url: url.into() }
	}
}

fn bound_to_string(bounds: &dyn RangeBounds<i64>) -> Result<String, &str> {
	match (bounds.start_bound(), bounds.end_bound()) {
		(Bound::Unbounded, Bound::Unbounded) => Ok("0-".to_owned()),
		(Bound::Included(s), Bound::Unbounded) if s >= &0 => Ok(format!("{}-", s)),
		(Bound::Excluded(s), Bound::Unbounded) if s >= &0 => Ok(format!("{}-", s + 1)),
		(Bound::Included(s), Bound::Unbounded) if s < &0 => Ok(format!("{}", s)),
		(Bound::Excluded(s), Bound::Unbounded) if s < &0 => Ok(format!("{}", s)),
		(Bound::Included(s), Bound::Included(e)) => Ok(format!("{}-{}", s, e)),
		(Bound::Included(s), Bound::Excluded(e)) => Ok(format!("{}-{}", s, e - 1)),
		(Bound::Excluded(s), Bound::Included(e)) => Ok(format!("{}-{}", s + 1, e)),
		(Bound::Excluded(s), Bound::Excluded(e)) => Ok(format!("{}-{}", s + 1, e - 1)),
		_ => Err("Range not supported"),
	}
}

#[async_trait]
impl ReadRanges for Request {
	async fn read_ranges(
		&mut self,
		ranges: Vec<Box<dyn RangeBounds<i64> + std::marker::Send + std::marker::Sync>>,
	) -> io::Result<Vec<u8>> {
		use reqwest::header::RANGE;
		use std::iter::Extend;

		let client = reqwest::Client::new();

		let mut range_bytes: Vec<u8> = Vec::with_capacity(ranges.len());
		for range in ranges.iter() {
			let byte_range = bound_to_string(&**range).unwrap();
			let resp = client
				.get(self.url.clone())
				.header(RANGE, format!("bytes={}", byte_range))
				.send()
				.await
				.unwrap();
			let bytes = resp.bytes().await.unwrap();
			range_bytes.extend(bytes.into_iter());
		}

		// use futures::{stream, StreamExt};
		// let range_bytes: Vec<bytes::Bytes> = stream::iter(ranges)
		// 		.map(|range| {
		// 			let byte_range = bound_to_string(&*range).unwrap();
		// 			let client = &client;
		// 			async move {
		// 				let resp = client
		// 					.get("https://firebasestorage.googleapis.com/v0/b/pxlr-app.appspot.com/o/it_dump_to_disk.pxlr?alt=media&token=c4668623-f7d1-49a0-8496-976b9ba54891")
		// 					.header(RANGE, format!("bytes={}", byte_range))
		// 					.send()
		// 					.await
		// 					.unwrap();
		// 				let bytes = resp
		// 					.bytes()
		// 					.await
		// 					.unwrap();
		// 				bytes
		// 			}
		// 		})
		// 		.buffered(4)
		// 		.collect::<Vec<_>>()
		// 		.await;

		Ok(range_bytes)
	}
}

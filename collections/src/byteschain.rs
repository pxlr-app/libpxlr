use bytes::{Bytes, BytesMut};
use std::{
	collections::VecDeque,
	iter::FromIterator,
	ops::{Bound, Index, RangeBounds},
};

#[derive(Debug)]
pub struct BytesChain {
	chain: VecDeque<Bytes>,
}

impl BytesChain {
	pub fn new() -> Self {
		BytesChain {
			chain: Default::default(),
		}
	}

	pub fn push(&mut self, bytes: Bytes) {
		self.chain.push_back(bytes);
	}

	pub fn append(&mut self, mut bytes: BytesChain) {
		self.chain.append(&mut bytes.chain);
	}

	pub fn clear(&mut self) {
		self.chain.clear();
	}

	pub fn len(&self) -> usize {
		self.chain.iter().map(|b| b.len()).sum()
	}

	pub fn iter(&self) -> impl Iterator<Item = &u8> + '_ {
		self.chain.iter().flatten()
	}

	pub fn slice(&self, range: impl RangeBounds<usize>) -> BytesChain {
		let from = match range.start_bound() {
			Bound::Unbounded => 0usize,
			Bound::Excluded(e) => *e,
			Bound::Included(e) => *e,
		};
		let to = match range.end_bound() {
			Bound::Unbounded => self.len(),
			Bound::Excluded(e) => *e,
			Bound::Included(e) => *e,
		};
		assert!(from <= to);
		assert!(to <= self.len());
		let mut chain = BytesChain::new();
		let mut offset = 0usize;
		for bytes in self.chain.iter() {
			let len = bytes.len();
			let begin;
			if offset > from {
				begin = 0;
			} else if offset + len > from {
				begin = from - offset;
			} else {
				offset += len;
				continue;
			}
			let end = std::cmp::min(len, to - offset);
			chain.push(bytes.slice(begin..end));
			offset += len;
			if offset >= to {
				break;
			}
		}
		chain
	}

	pub fn copy_to_slice(&self, range: impl RangeBounds<usize>, target: &mut [u8]) {
		let mut from = match range.start_bound() {
			Bound::Unbounded => 0usize,
			Bound::Excluded(e) => *e,
			Bound::Included(e) => *e,
		};
		let to = match range.end_bound() {
			Bound::Unbounded => self.len(),
			Bound::Excluded(e) => *e,
			Bound::Included(e) => *e,
		};
		assert!(from <= to);
		assert!(to <= self.len());
		let tlen = target.len();
		let mut copied = 0usize;
		for bytes in self.chain.iter() {
			let len = bytes.len();
			if from < len {
				let available = std::cmp::min(len - from, tlen - copied);
				target[copied..(copied + available)]
					.copy_from_slice(&bytes.slice(from..(from + available)));
				copied += available;
				from = 0;
				if tlen == copied {
					break;
				}
			} else {
				from -= len;
			}
		}
	}
}

impl Default for BytesChain {
	fn default() -> Self {
		BytesChain::new()
	}
}

impl Index<usize> for BytesChain {
	type Output = u8;

	fn index(&self, idx: usize) -> &Self::Output {
		&self.iter().skip(idx).take(1).next().unwrap()
	}
}

impl Into<Bytes> for BytesChain {
	fn into(self) -> Bytes {
		let mut data = vec![0; self.len()];
		self.copy_to_slice(.., &mut data);
		Bytes::from(data)
	}
}

impl IntoIterator for BytesChain {
	type Item = u8;
	type IntoIter = std::iter::Flatten<std::collections::vec_deque::IntoIter<Bytes>>;

	fn into_iter(self) -> Self::IntoIter {
		self.chain.into_iter().flatten()
	}
}

impl From<Bytes> for BytesChain {
	fn from(value: Bytes) -> Self {
		let mut chain = VecDeque::with_capacity(1);
		chain.push_back(value);
		BytesChain { chain }
	}
}

impl From<BytesMut> for BytesChain {
	fn from(value: BytesMut) -> Self {
		let mut chain = VecDeque::with_capacity(1);
		chain.push_back(value.freeze());
		BytesChain { chain }
	}
}

impl From<Vec<u8>> for BytesChain {
	fn from(value: Vec<u8>) -> Self {
		let mut chain = VecDeque::with_capacity(1);
		chain.push_back(Bytes::from(value));
		BytesChain { chain }
	}
}

impl From<&'static [u8]> for BytesChain {
	fn from(value: &'static [u8]) -> Self {
		let mut chain = VecDeque::with_capacity(1);
		chain.push_back(Bytes::from(value));
		BytesChain { chain }
	}
}

impl FromIterator<u8> for BytesChain {
	fn from_iter<I: IntoIterator<Item = u8>>(iter: I) -> Self {
		let mut data: Vec<u8> = Vec::new();
		for i in iter {
			data.push(i);
		}
		let mut chain = VecDeque::with_capacity(1);
		chain.push_back(Bytes::from(data));
		BytesChain { chain }
	}
}

#[cfg(test)]
mod tests {
	use super::*;
	use bytes::Bytes;

	#[test]
	fn test_specs() {
		let mut a = BytesChain::new();
		a.push(Bytes::from(&[1u8, 2, 3, 4, 5][..]));
		a.push(Bytes::from(&[6u8, 7, 8, 9, 10][..]));
		let mut b = a.slice(3..8);
		b.push(Bytes::from(&[11u8, 12, 13, 14, 15][..]));

		let mut buf = vec![0u8; 5];
		b.copy_to_slice(5..10, &mut buf);
		buf = buf.drain(..).map(|i| i + 10).collect();
		let mut c = BytesChain::new();
		c.append(b.slice(0..5));
		c.push(Bytes::from(buf));

		assert_eq!(c[5], 21);
		assert_eq!(
			c.iter().collect::<Vec<_>>(),
			vec![&4, &5, &6, &7, &8, &21, &22, &23, &24, &25]
		);
	}
}

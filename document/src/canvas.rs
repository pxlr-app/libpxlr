use crate::prelude::*;
use bytes::Bytes;
use std::collections::VecDeque;

#[derive(Debug, Clone)]
pub struct Canvas {
	pub size: Extent2<u32>,
	pub channels: Channel,
	pub data: Bytes,
	pub stencils: VecDeque<Arc<PositionnedStencil>>,
}

#[derive(Debug)]
pub enum CanvasError {
	ComponentMismatch,
	SizeMismatch,
}

impl std::error::Error for CanvasError {}

impl std::fmt::Display for CanvasError {
	fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
		match *self {
			CanvasError::ComponentMismatch => write!(f, "Component mismatch."),
			CanvasError::SizeMismatch => write!(f, "Size mismatch."),
		}
	}
}

impl Canvas {
	pub fn new(size: Extent2<u32>, channels: Channel, data: Vec<u8>) -> Self {
		Canvas {
			size,
			channels,
			data: Bytes::from(data),
			stencils: VecDeque::new(),
		}
	}

	pub fn apply_stencil(
		&self,
		position: Vec2<u32>,
		stencil: Stencil,
	) -> Result<Canvas, CanvasError> {
		if self.channels != stencil.channels {
			return Err(CanvasError::ComponentMismatch);
		}
		let mut cloned = self.clone();
		cloned
			.stencils
			.push_front(Arc::new(PositionnedStencil { position, stencil }));
		Ok(cloned)
	}

	/// Copy the pixel data to new Bytes buffer
	///
	/// ```
	/// use document::prelude::*;
	/// let canvas = Canvas::new(Extent2::new(2, 2), Channel::A, vec![1u8, 2, 3, 4]);
	/// let bytes = canvas.copy_to_bytes();
	/// assert_eq!(&bytes[..], &[1u8, 2, 3, 4][..]);
	/// ```
	pub fn copy_to_bytes(&self) -> Bytes {
		Bytes::from(self.into_iter().flatten().map(|b| *b).collect::<Vec<u8>>())
	}
}

impl std::ops::Index<(&u32, &u32)> for Canvas {
	type Output = [u8];

	fn index(&self, index: (&u32, &u32)) -> &Self::Output {
		let index = (index.1 + self.size.w * index.0) as usize;
		self.index(index)
	}
}

impl std::ops::Index<usize> for Canvas {
	type Output = [u8];

	fn index(&self, index: usize) -> &Self::Output {
		let stride = Channel::size_of(self.channels);
		let x = index as u32 % self.size.w;
		let y = index as u32 / self.size.w;
		for stencil in self.stencils.iter() {
			if x >= stencil.position.x
				&& x <= stencil.position.x + stencil.stencil.size.w
				&& y >= stencil.position.y
				&& y <= stencil.position.y + stencil.stencil.size.h
			{
				let x = x - stencil.position.x;
				let y = y - stencil.position.y;
				if let Ok(data) = stencil.stencil.try_get((&x, &y)) {
					return data;
				}
			}
		}
		&self.data[(index * stride)..((index + 1) * stride)]
	}
}

pub struct CanvasIterator<'a> {
	canvas: &'a Canvas,
	index: usize,
}

impl<'a> Iterator for CanvasIterator<'a> {
	type Item = &'a [u8];

	fn next(&mut self) -> Option<&'a [u8]> {
		if self.index < (self.canvas.size.w * self.canvas.size.h) as usize {
			let index = self.index;
			self.index += 1;
			return Some(&self.canvas[index]);
		}
		return None;
	}
}

impl<'a> IntoIterator for &'a Canvas {
	type Item = &'a [u8];
	type IntoIter = CanvasIterator<'a>;

	fn into_iter(self) -> Self::IntoIter {
		CanvasIterator {
			canvas: self,
			index: 0,
		}
	}
}

#[cfg(test)]
mod tests {
	use crate::prelude::*;
	use collections::bitvec;

	#[test]
	fn test_index() {
		let a = Canvas::new(Extent2::new(2, 2), Channel::A, vec![1u8, 2, 3, 4]);
		assert_eq!(&a[0], &[1u8][..]);
		assert_eq!(&a[1], &[2u8][..]);
		assert_eq!(&a[2], &[3u8][..]);
		assert_eq!(&a[3], &[4u8][..]);

		let stencil = Stencil {
			size: Extent2::new(2, 2),
			mask: bitvec![Lsb0, u8; 1, 0, 0, 1],
			channels: Channel::A,
			data: vec![8u8, 9],
		};

		let b = a.apply_stencil(Vec2::new(0, 0), stencil).unwrap();
		assert_eq!(&b[0], &[8u8][..]);
		assert_eq!(&b[1], &[2u8][..]);
		assert_eq!(&b[2], &[3u8][..]);
		assert_eq!(&b[3], &[9u8][..]);

		let stencil = Stencil {
			size: Extent2::new(2, 2),
			mask: bitvec![Lsb0, u8; 0, 1, 1, 0],
			channels: Channel::A,
			data: vec![11u8, 12],
		};

		let c = a.apply_stencil(Vec2::new(0, 0), stencil).unwrap();
		assert_eq!(&c[0], &[1u8][..]);
		assert_eq!(&c[1], &[11u8][..]);
		assert_eq!(&c[2], &[12u8][..]);
		assert_eq!(&c[3], &[4u8][..]);
	}

	#[test]
	fn test_iter() {
		let a = Canvas::new(Extent2::new(2, 2), Channel::A, vec![1u8, 2, 3, 4]);

		let mut i = a.into_iter();
		assert_eq!(i.next(), Some(&[1u8][..]));
		assert_eq!(i.next(), Some(&[2u8][..]));
		assert_eq!(i.next(), Some(&[3u8][..]));
		assert_eq!(i.next(), Some(&[4u8][..]));
		assert_eq!(i.next(), None);
	}
}

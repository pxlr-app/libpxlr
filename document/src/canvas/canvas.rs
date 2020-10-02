use crate::prelude::*;
use bytes::Bytes;
use std::collections::VecDeque;

#[derive(Debug, Clone)]
pub struct Canvas {
	pub size: Extent2<u32>,
	pub channels: Channel,
	pub stencils: VecDeque<Arc<CanvasStencil>>,
}

#[derive(Debug, Clone)]
pub struct CanvasStencil {
	pub position: Vec2<u32>,
	pub stencil: Stencil,
}

#[derive(Debug)]
pub enum CanvasError {
	ChannelMismatch,
}

impl std::error::Error for CanvasError {}

impl std::fmt::Display for CanvasError {
	fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
		match *self {
			CanvasError::ChannelMismatch => write!(f, "Channel mismatch."),
		}
	}
}

impl Canvas {
	/// Create an empty canvas of specific size
	pub fn new(size: Extent2<u32>, channels: Channel) -> Self {
		Canvas {
			size,
			channels,
			stencils: VecDeque::new(),
		}
	}

	/// Create a canvas from pixel buffer
	pub fn from_buffer(size: Extent2<u32>, channels: Channel, data: Vec<u8>) -> Self {
		let stencil = Stencil::from_buffer(size, channels, &data[..]);
		let size = stencil.size.clone();
		Self::new(size, channels)
			.apply_stencil(Vec2::new(0, 0), stencil)
			.unwrap()
	}

	/// Apply a stencil on this canvas
	pub fn apply_stencil(
		&self,
		position: Vec2<u32>,
		stencil: Stencil,
	) -> Result<Canvas, CanvasError> {
		self.apply_stencil_with_blend(position, stencil, Blend::Normal, Compose::Lighter)
	}

	/// Apply a stencil on this canvas by blending the stencil on top
	/// of previous stencils.
	pub fn apply_stencil_with_blend(
		&self,
		position: Vec2<u32>,
		mut stencil: Stencil,
		blend_mode: Blend,
		compose_op: Compose,
	) -> Result<Canvas, CanvasError> {
		if self.channels != stencil.channels {
			return Err(CanvasError::ChannelMismatch);
		}
		let mut cloned = self.clone();
		for (x, y, dst) in stencil.iter_mut() {
			for stencil in self.stencils.iter().rev() {
				if x >= stencil.position.x
					&& x <= stencil.position.x + stencil.stencil.size.w
					&& y >= stencil.position.y
					&& y <= stencil.position.y + stencil.stencil.size.h
				{
					let x = x - stencil.position.x;
					let y = y - stencil.position.y;
					if let Ok(bck) = stencil.stencil.try_get((&x, &y)) {
						let frt = unsafe { std::mem::transmute::<&mut [u8], &[u8]>(dst) };
						blend_pixel_into(blend_mode, compose_op, self.channels, frt, bck, dst);
					}
				}
			}
		}
		cloned
			.stencils
			.push_front(Arc::new(CanvasStencil { position, stencil }));
		Ok(cloned)
	}

	/// Iterate over each pixel of this canvas
	pub fn iter(&self) -> CanvasIterator {
		CanvasIterator {
			canvas: self,
			index: 0,
		}
	}

	/// Copy the pixel data to new Bytes buffer
	///
	/// ```
	/// use document::prelude::*;
	/// let canvas = Canvas::from_buffer(Extent2::new(2, 2), Channel::A, vec![1u8, 2, 3, 4]);
	/// let bytes = canvas.copy_to_bytes();
	/// assert_eq!(&bytes[..], &[1u8, 2, 3, 4][..]);
	/// ```
	pub fn copy_to_bytes(&self) -> Bytes {
		Bytes::from(self.into_iter().flatten().map(|b| *b).collect::<Vec<u8>>())
	}

	/// Try to retrieve a pixel from the canvas at location `(x, y)`.
	pub fn try_get_at(&self, x: u32, y: u32) -> Option<&Pixel> {
		let index = (x + self.size.w * y) as usize;
		self.try_get(index)
	}

	/// Try to retrieve a pixel from the canvas at index.
	pub fn try_get(&self, index: usize) -> Option<&Pixel> {
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
				if let Ok(pixel) = stencil.stencil.try_get((&x, &y)) {
					return Some(pixel);
				}
			}
		}
		return None;
	}
}

pub struct CanvasIterator<'a> {
	canvas: &'a Canvas,
	index: usize,
}

impl<'a> Iterator for CanvasIterator<'a> {
	type Item = &'a Pixel;

	fn next(&mut self) -> Option<&'a Pixel> {
		if self.index < (self.canvas.size.w * self.canvas.size.h) as usize {
			let index = self.index;
			self.index += 1;
			return self.canvas.try_get(index);
		}
		return None;
	}
}

impl<'a> IntoIterator for &'a Canvas {
	type Item = &'a Pixel;
	type IntoIter = CanvasIterator<'a>;

	fn into_iter(self) -> Self::IntoIter {
		self.iter()
	}
}

#[cfg(test)]
mod tests {
	use crate::prelude::*;
	use collections::bitvec;

	#[test]
	fn test_index() {
		let a = Canvas::new(Extent2::new(2, 2), Channel::A);

		let stencil = Stencil {
			size: Extent2::new(2, 2),
			mask: bitvec![Lsb0, u8; 1, 0, 0, 1],
			channels: Channel::A,
			data: vec![8u8, 9],
		};

		let b = a.apply_stencil(Vec2::new(0, 0), stencil).unwrap();
		assert_eq!(b.try_get(0).unwrap(), &[8u8][..]);
		assert_eq!(b.try_get(1), None);
		assert_eq!(b.try_get(2), None);
		assert_eq!(b.try_get(3).unwrap(), &[9u8][..]);

		let stencil = Stencil {
			size: Extent2::new(2, 2),
			mask: bitvec![Lsb0, u8; 0, 1, 1, 0],
			channels: Channel::A,
			data: vec![11u8, 12],
		};

		let c = a.apply_stencil(Vec2::new(0, 0), stencil).unwrap();
		assert_eq!(c.try_get(0), None);
		assert_eq!(c.try_get(1).unwrap(), &[11u8][..]);
		assert_eq!(c.try_get(2).unwrap(), &[12u8][..]);
		assert_eq!(c.try_get(3), None);
	}

	#[test]
	fn test_iter() {
		let a = Canvas::new(Extent2::new(2, 2), Channel::A);
		let stencil = Stencil {
			size: Extent2::new(2, 2),
			mask: bitvec![Lsb0, u8; 1, 1, 1, 1],
			channels: Channel::A,
			data: vec![1u8, 2, 3, 4],
		};
		let b = a.apply_stencil(Vec2::new(0, 0), stencil).unwrap();

		let mut i = b.iter();
		assert_eq!(i.next(), Some(&[1u8][..]));
		assert_eq!(i.next(), Some(&[2u8][..]));
		assert_eq!(i.next(), Some(&[3u8][..]));
		assert_eq!(i.next(), Some(&[4u8][..]));
		assert_eq!(i.next(), None);
	}
}

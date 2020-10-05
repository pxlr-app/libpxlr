use crate::prelude::*;
use bytes::Bytes;
use std::{collections::VecDeque, ops::Index};

#[derive(Debug, Clone)]
pub struct Canvas {
	pub size: Extent2<u32>,
	pub channels: Channel,
	empty_pixel: Vec<u8>,
	pub stencils: VecDeque<Arc<CanvasStencil>>,
}

#[derive(Debug, Clone)]
pub struct CanvasStencil {
	pub position: Vec2<i32>,
	pub stencil: Stencil,
}

#[derive(Debug)]
pub enum CanvasError {
	ChannelMismatch,
	RegionNotContained,
}

impl std::error::Error for CanvasError {}

impl std::fmt::Display for CanvasError {
	fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
		match *self {
			CanvasError::ChannelMismatch => write!(f, "Channel mismatch."),
			CanvasError::RegionNotContained => write!(f, "Region not contained in this Canvas"),
		}
	}
}

#[derive(Debug, Copy, Clone, PartialEq)]
pub enum FlipAxis {
	Horizontal,
	Vertical,
	Both,
}

impl Canvas {
	/// Create an empty canvas of specific size
	pub fn new(size: Extent2<u32>, channels: Channel) -> Self {
		Canvas {
			size,
			channels,
			empty_pixel: channels.default_pixel(),
			stencils: VecDeque::new(),
		}
	}

	/// Create a canvas from pixel buffer
	pub fn from_buffer(size: Extent2<u32>, channels: Channel, data: Vec<u8>) -> Self {
		let stencil = Stencil::from_buffer(size, channels, data);
		let size = stencil.size.clone();
		Self::new(size, channels)
			.apply_stencil(Vec2::new(0, 0), stencil)
			.unwrap()
	}

	/// Apply a stencil on this canvas
	pub fn apply_stencil(
		&self,
		position: Vec2<i32>,
		stencil: Stencil,
	) -> Result<Canvas, CanvasError> {
		self.apply_stencil_with_blend(position, stencil, Blend::Normal, Compose::Lighter)
	}

	/// Apply a stencil on this canvas by blending the stencil on top
	/// of previous stencils.
	pub fn apply_stencil_with_blend(
		&self,
		position: Vec2<i32>,
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
				if x as i32 >= stencil.position.x
					&& x as i32 <= stencil.position.x + stencil.stencil.size.w as i32
					&& y as i32 >= stencil.position.y
					&& y as i32 <= stencil.position.y + stencil.stencil.size.h as i32
				{
					let x = x as i32 - stencil.position.x;
					let y = y as i32 - stencil.position.y;
					if x >= 0 && y >= 0 {
						if let Ok(bck) = stencil.stencil.try_get((&(x as u32), &(y as u32))) {
							let frt = unsafe { std::mem::transmute::<&mut [u8], &[u8]>(dst) };
							blend_pixel_into(blend_mode, compose_op, self.channels, frt, bck, dst);
						}
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
			region: Rect::new(0, 0, self.size.w, self.size.h),
			index: 0,
		}
	}

	/// Iterate part of the canvas
	pub fn iter_region(&self, region: Rect<u32, u32>) -> Result<CanvasIterator, CanvasError> {
		if region.x + region.w > self.size.w || region.y + region.h > self.size.h {
			Err(CanvasError::RegionNotContained)
		} else {
			Ok(CanvasIterator {
				canvas: self,
				region,
				index: 0,
			})
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

	/// Flatten canvas
	pub fn flatten(&self) -> Self {
		let data = self.copy_to_bytes().to_vec();
		Self::from_buffer(self.size, self.channels, data)
	}

	/// Resize canvas
	pub fn resize(&self, size: Extent2<u32>, interpolation: Interpolation) -> Self {
		use math::Vec3;
		let mut resized = vec![0u8; size.w as usize * size.h as usize * self.channels.size()];
		let transform = Mat3::scaling_3d(Vec3::new(
			size.w as f32 / self.size.w as f32,
			size.h as f32 / self.size.h as f32,
			1.,
		));
		transform_into(
			&transform,
			&interpolation,
			&size,
			self.channels,
			self,
			&mut resized[..],
		);
		Self::from_buffer(size, self.channels, resized)
	}

	/// Crop canvas
	pub fn crop(&self, region: Rect<i32, u32>) -> Self {
		let mut canvas = self.clone();
		let outer: Rect<i32, i32> = Rect::new(region.x, region.y, region.w as i32, region.h as i32);

		canvas.size = region.extent();
		canvas.stencils = canvas
			.stencils
			.drain(..)
			.filter_map(|stencil| {
				let inner = Rect::new(
					stencil.position.x as i32,
					stencil.position.y as i32,
					stencil.stencil.size.w as i32,
					stencil.stencil.size.h as i32,
				);
				if outer.contains_rect(inner) {
					let mut cloned = (*stencil).clone();
					cloned.position.x -= outer.x;
					cloned.position.y -= outer.y;
					Some(Arc::new(cloned))
				} else {
					None
				}
			})
			.collect();

		canvas
	}

	/// Flip canvas
	pub fn flip(&self, axis: FlipAxis) -> Self {
		use math::Vec3;
		let mut resized =
			vec![0u8; self.size.w as usize * self.size.h as usize * self.channels.size()];
		let (offset, scaled) = match axis {
			FlipAxis::Horizontal => (
				Vec2::new(self.size.w as f32 / -2f32 + 0.5, 0.),
				Vec3::new(-1., 1., 1.),
			),
			FlipAxis::Vertical => (
				Vec2::new(0., self.size.h as f32 / -2f32 + 0.5),
				Vec3::new(1., -1., 1.),
			),
			FlipAxis::Both => (
				Vec2::new(
					self.size.w as f32 / -2f32 + 0.5,
					self.size.h as f32 / -2f32 + 0.5,
				),
				Vec3::new(-1., -1., 1.),
			),
		};
		let transform = Mat3::translation_2d(offset)
			.scaled_3d(scaled)
			.translated_2d(-offset);
		transform_into(
			&transform,
			&Interpolation::Nearest,
			&self.size,
			self.channels,
			self,
			&mut resized[..],
		);
		Self::from_buffer(self.size, self.channels, resized)
	}
}

pub struct CanvasIterator<'a> {
	canvas: &'a Canvas,
	region: Rect<u32, u32>,
	index: usize,
}

impl<'a> Iterator for CanvasIterator<'a> {
	type Item = &'a Pixel;

	fn next(&mut self) -> Option<&'a Pixel> {
		if self.index < (self.region.w * self.region.h) as usize {
			let x = self.index % self.region.w as usize + self.region.x as usize;
			let y = self.index / self.region.w as usize + self.region.y as usize;
			self.index += 1;
			return Some(&self.canvas[(x as u32, y as u32)]);
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

impl Index<(u32, u32)> for Canvas {
	type Output = Pixel;

	fn index(&self, index: (u32, u32)) -> &Self::Output {
		let index = (index.0 + self.size.w * index.1) as usize;
		self.index(index)
	}
}

impl Index<usize> for Canvas {
	type Output = Pixel;

	fn index(&self, index: usize) -> &Self::Output {
		let x = index as u32 % self.size.w;
		let y = index as u32 / self.size.w;
		for stencil in self.stencils.iter() {
			if x as i32 >= stencil.position.x
				&& x as i32 <= stencil.position.x + stencil.stencil.size.w as i32
				&& y as i32 >= stencil.position.y
				&& y as i32 <= stencil.position.y + stencil.stencil.size.h as i32
			{
				let x = x as i32 - stencil.position.x;
				let y = y as i32 - stencil.position.y;
				if x >= 0 && y >= 0 {
					if let Ok(pixel) = stencil.stencil.try_get((&(x as u32), &(y as u32))) {
						return pixel;
					}
				}
			}
		}
		&self.empty_pixel
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
		assert_eq!(b[0], [8u8][..]);
		assert_eq!(b[1], [0u8][..]);
		assert_eq!(b[2], [0u8][..]);
		assert_eq!(b[3], [9u8][..]);

		let stencil = Stencil {
			size: Extent2::new(2, 2),
			mask: bitvec![Lsb0, u8; 0, 1, 1, 0],
			channels: Channel::A,
			data: vec![11u8, 12],
		};

		let c = a.apply_stencil(Vec2::new(0, 0), stencil).unwrap();
		assert_eq!(c[0], [0u8][..]);
		assert_eq!(c[1], [11u8][..]);
		assert_eq!(c[2], [12u8][..]);
		assert_eq!(c[3], [0u8][..]);
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

		let mut i = b.iter_region(Rect::new(1, 0, 1, 2)).unwrap();
		assert_eq!(i.next(), Some(&[2u8][..]));
		assert_eq!(i.next(), Some(&[4u8][..]));
		assert_eq!(i.next(), None);
	}
}

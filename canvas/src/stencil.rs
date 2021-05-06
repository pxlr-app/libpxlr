use crate::braille::braille_fmt2;
use bitvec::{bitvec, order::Lsb0, vec::BitVec};
use color::*;
use serde::{Deserialize, Serialize};
use vek::geom::repr_c::Rect;

#[derive(Clone, Serialize, Deserialize)]
pub struct Stencil {
	bounds: Rect<i32, i32>,
	mask: BitVec<Lsb0, u8>,
	channel: Channel,
	empty_pixel: Vec<u8>,
	data: Vec<u8>,
}

impl Stencil {
	/// Retrieve rectangle
	pub fn bounds(&self) -> Rect<i32, i32> {
		self.bounds
	}

	/// Retrieve channel
	pub fn channel(&self) -> Channel {
		self.channel
	}

	/// Create a new empty stencil
	pub fn new(rect: Rect<i32, i32>, channel: Channel) -> Self {
		let len = (rect.w * rect.h) as usize;
		let mut buffer = Vec::with_capacity(len * channel.pixel_stride());
		let default_pixel = channel.default_pixel();
		for _ in 0..len {
			buffer.extend_from_slice(&default_pixel);
		}
		Self::from_buffer(rect, channel, buffer)
	}

	/// Create a stencil from pixel data
	pub fn from_buffer(rect: Rect<i32, i32>, channel: Channel, buffer: Vec<u8>) -> Self {
		let len = (rect.w * rect.h) as usize;
		assert_eq!(len * channel.pixel_stride(), buffer.len());
		let mut mask = bitvec![Lsb0, u8; 1; len];
		mask.set_uninitialized(false);
		Self {
			bounds: rect,
			mask,
			channel,
			empty_pixel: channel.default_pixel(),
			data: buffer,
		}
	}

	/// Create a stencil from pixel data and masking invisible one based on alpha
	pub fn from_buffer_mask_alpha(rect: Rect<i32, i32>, channel: Channel, buffer: Vec<u8>) -> Self {
		match channel {
			Channel::Lumaa | Channel::LumaaNormal | Channel::Rgba | Channel::RgbaNormal => {
				let len = (rect.w * rect.h) as usize;
				let stride = channel.pixel_stride();
				assert_eq!(len * stride, buffer.len());
				let mut mask = bitvec![Lsb0, u8; 0; len];
				// #[cfg(feature = "rayon")]
				// let chunks = buffer.par_chunks(stride);
				// #[cfg(not(feature = "rayon"))]
				let chunks = buffer.chunks(stride);

				let data = chunks
					.enumerate()
					.filter_map(|(i, data)| {
						let pixel = Pixel::from_buffer(&data, channel);
						let alpha = match channel {
							Channel::Lumaa | Channel::LumaaNormal => pixel.lumaa().unwrap().alpha,
							Channel::Rgba | Channel::RgbaNormal => pixel.rgba().unwrap().alpha,
							_ => 0,
						};
						if alpha == 0 {
							None
						} else {
							mask.set(i, true);
							Some(data.to_vec())
						}
					})
					.flatten()
					.collect::<Vec<_>>();

				Self {
					bounds: rect,
					mask,
					channel,
					empty_pixel: channel.default_pixel(),
					data,
				}
			}
			_ => Self::from_buffer(rect, channel, buffer),
		}
	}

	/// Try to retrieve a pixel at coordinate
	pub fn try_get(&self, x: i32, y: i32) -> Option<&[u8]> {
		// if self.bounds.contains_point(Vec2::new(x, y)) {
		if self.bounds.x <= x
			&& x < self.bounds.x + self.bounds.w
			&& self.bounds.y <= y
			&& y < self.bounds.y + self.bounds.h
		{
			let index = (y.wrapping_sub(self.bounds.y) * self.bounds.w
				+ x.wrapping_sub(self.bounds.x)) as usize;
			self.try_index(index)
		} else {
			None
		}
	}

	/// Try to retrieve a pixel at index
	pub fn try_index(&self, index: usize) -> Option<&[u8]> {
		if self.mask[index] {
			let stride = self.channel.pixel_stride();
			let count: usize = self.mask[..index].count_ones();
			Some(&self.data[(count * stride)..((count + 1) * stride)])
		} else {
			None
		}
	}

	/// Merge two stencil and blend them together if need be
	pub fn merge(frt: &Self, bck: &Self, blend_mode: Blending, compose_op: Compositing) -> Self {
		assert_eq!(frt.channel, bck.channel);
		let channel = frt.channel;

		// Calculate new size
		let rect = frt.bounds.union(bck.bounds);

		// Allocate new buffers
		let stride = frt.channel.pixel_stride();
		let mut mask = bitvec![Lsb0, u8; 0; (rect.w * rect.h) as usize];
		let mut data: Vec<u8> = Vec::with_capacity((rect.w * rect.h * stride as i32) as usize);
		let mut tmp = frt.channel.default_pixel();

		for i in 0..mask.len() {
			let x = (i % rect.w as usize) as i32 + rect.x;
			let y = (i / rect.w as usize) as i32 + rect.y;

			let frt_buf = frt.try_get(x, y);
			let bck_buf = bck.try_get(x, y);

			match (frt_buf, bck_buf) {
				(None, None) => mask.set(i, false),
				(Some(frt_buf), None) => {
					mask.set(i, true);
					data.extend_from_slice(frt_buf);
				}
				(None, Some(bck_buf)) => {
					mask.set(i, true);
					data.extend_from_slice(bck_buf);
				}
				(Some(frt_buf), Some(bck_buf)) => {
					let frt_px = Pixel::from_buffer(frt_buf, frt.channel);
					let bck_px = Pixel::from_buffer(bck_buf, frt.channel);
					let mut pixel = PixelMut::from_buffer_mut(&mut tmp, channel);
					pixel
						.blend(blend_mode, compose_op, &frt_px, &bck_px)
						.unwrap();
					let alpha = match channel {
						Channel::Lumaa | Channel::LumaaNormal => pixel.lumaa().unwrap().alpha,
						Channel::Rgba | Channel::RgbaNormal => pixel.rgba().unwrap().alpha,
						_ => 1,
					};
					if alpha > 0 {
						mask.set(i, true);
						data.extend_from_slice(&tmp);
					}
				}
			}
		}
		Self {
			bounds: rect,
			mask,
			channel,
			empty_pixel: channel.default_pixel(),
			data,
		}
	}

	/// Iterate over pixel of this stencil
	pub fn iter(&self) -> StencilIterator {
		StencilIterator {
			bit_offset: 0,
			data_offset: 0,
			bounds: self.bounds,
			mask: &self.mask,
			pixel_stride: self.channel.pixel_stride(),
			data: &self.data,
		}
	}

	/// Iterate over pixel of this stencil
	pub fn iter_mut(&mut self) -> StencilMutIterator {
		StencilMutIterator {
			bit_offset: 0,
			data_offset: 0,
			bounds: self.bounds,
			mask: &self.mask,
			pixel_stride: self.channel.pixel_stride(),
			data: &mut self.data,
		}
	}
}

impl std::fmt::Debug for Stencil {
	fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
		write!(
			f,
			"Stencil ( {} )",
			braille_fmt2(
				&self.mask,
				self.bounds.w as usize,
				self.bounds.h as usize,
				"\n           "
			)
		)
	}
}

impl std::ops::Add for &Stencil {
	type Output = Stencil;

	fn add(self, other: Self) -> Self::Output {
		Stencil::merge(self, other, Blending::Normal, Compositing::Lighter)
	}
}

impl std::ops::Index<(i32, i32)> for Stencil {
	type Output = [u8];

	fn index(&self, index: (i32, i32)) -> &Self::Output {
		if let Some(pixel) = self.try_get(index.0, index.1) {
			pixel
		} else {
			&self.empty_pixel
		}
	}
}

impl std::ops::Index<usize> for Stencil {
	type Output = [u8];

	fn index(&self, index: usize) -> &Self::Output {
		if let Some(pixel) = self.try_index(index) {
			pixel
		} else {
			&self.empty_pixel
		}
	}
}

pub struct StencilIterator<'stencil> {
	bit_offset: usize,
	data_offset: usize,
	bounds: Rect<i32, i32>,
	mask: &'stencil BitVec<Lsb0, u8>,
	pixel_stride: usize,
	data: &'stencil Vec<u8>,
}

impl<'stencil> Iterator for StencilIterator<'stencil> {
	type Item = (i32, i32, &'stencil [u8]);

	fn next(&mut self) -> Option<(i32, i32, &'stencil [u8])> {
		while self.bit_offset < self.mask.len() {
			let bit_offset = self.bit_offset;
			self.bit_offset += 1;
			let bit = self.mask[bit_offset];
			if bit {
				let x = bit_offset % self.bounds.w as usize;
				let y = (bit_offset / self.bounds.w as usize) | 0;
				self.data_offset += 1;
				return Some((
					x as i32 + self.bounds.x,
					y as i32 + self.bounds.y,
					&self.data[(self.data_offset.wrapping_sub(1) * self.pixel_stride)
						..(self.data_offset * self.pixel_stride)],
				));
			}
		}
		return None;
	}
}

impl<'stencil> IntoIterator for &'stencil Stencil {
	type Item = (i32, i32, &'stencil [u8]);
	type IntoIter = StencilIterator<'stencil>;

	fn into_iter(self) -> Self::IntoIter {
		self.iter()
	}
}

pub struct StencilMutIterator<'stencil> {
	bit_offset: usize,
	data_offset: usize,
	bounds: Rect<i32, i32>,
	mask: &'stencil BitVec<Lsb0, u8>,
	pixel_stride: usize,
	data: &'stencil mut Vec<u8>,
}

impl<'stencil> Iterator for StencilMutIterator<'stencil> {
	type Item = (i32, i32, &'stencil mut [u8]);

	fn next<'iter>(&'iter mut self) -> Option<(i32, i32, &'stencil mut [u8])> {
		while self.bit_offset < self.mask.len() {
			let bit_offset = self.bit_offset;
			self.bit_offset += 1;
			let bit = self.mask[bit_offset];
			if bit {
				let x = bit_offset % self.bounds.w as usize;
				let y = (bit_offset / self.bounds.w as usize) | 0;
				self.data_offset += 1;
				let data: &'iter mut [u8] = &mut self.data[(self.data_offset.wrapping_sub(1)
					* self.pixel_stride)
					..(self.data_offset * self.pixel_stride)];
				let data =
					unsafe { std::mem::transmute::<&'iter mut [u8], &'stencil mut [u8]>(data) };
				return Some((x as i32 + self.bounds.x, y as i32 + self.bounds.y, data));
			}
		}
		return None;
	}
}

#[cfg(test)]
mod tests {
	use super::*;

	#[test]
	fn test_from_buffer() {
		let s = Stencil::from_buffer(Rect::new(0, 0, 2, 2), Channel::Luma, vec![1, 2, 3, 4]);
		assert_eq!(*s.mask, bitvec![1, 1, 1, 1]);
		assert_eq!(*s.data, [1, 2, 3, 4]);
	}

	#[test]
	fn test_from_buffer_mask_alpha() {
		let s = Stencil::from_buffer_mask_alpha(
			Rect::new(0, 0, 2, 2),
			Channel::Lumaa,
			vec![1, 255, 0, 0, 0, 0, 4, 1],
		);
		assert_eq!(*s.mask, bitvec![1, 0, 0, 1]);
		assert_eq!(*s.data, [1, 255, 4, 1]);
	}

	#[test]
	fn test_debug() {
		let s = Stencil::new(Rect::new(0, 0, 3, 1), Channel::Luma);
		assert_eq!(format!("{:?}", s), "Stencil ( ⠉⠁ )");
		let s = Stencil::new(Rect::new(0, 0, 1, 3), Channel::Luma);
		assert_eq!(format!("{:?}", s), "Stencil ( ⠇ )");
	}

	#[test]
	fn test_merge() {
		let a = Stencil::from_buffer_mask_alpha(
			Rect::new(0, 0, 2, 2),
			Channel::Lumaa,
			vec![1, 255, 0, 0, 0, 0, 4, 255],
		);
		assert_eq!(format!("{:?}", a), "Stencil ( ⠑ )");
		let b = Stencil::from_buffer_mask_alpha(
			Rect::new(0, 0, 2, 2),
			Channel::Lumaa,
			vec![0, 0, 2, 255, 3, 255, 0, 0],
		);
		assert_eq!(format!("{:?}", b), "Stencil ( ⠊ )");
		let c = Stencil::merge(&a, &b, Blending::Normal, Compositing::Lighter);
		assert_eq!(format!("{:?}", c), "Stencil ( ⠛ )");
		assert_eq!(c.data, vec![1, 255, 2, 255, 3, 255, 4, 255]);

		let a = Stencil::from_buffer_mask_alpha(
			Rect::new(0, 0, 2, 2),
			Channel::Lumaa,
			vec![1, 255, 2, 255, 0, 0, 4, 255],
		);
		assert_eq!(format!("{:?}", a), "Stencil ( ⠙ )");
		let b = Stencil::from_buffer_mask_alpha(
			Rect::new(0, 0, 2, 2),
			Channel::Lumaa,
			vec![0, 0, 20, 255, 3, 255, 0, 0],
		);
		assert_eq!(format!("{:?}", b), "Stencil ( ⠊ )");
		let c = Stencil::merge(&a, &b, Blending::Normal, Compositing::Lighter);
		assert_eq!(format!("{:?}", c), "Stencil ( ⠛ )");
		assert_eq!(c.data, vec![1, 255, 22, 255, 3, 255, 4, 255]);

		let a = Stencil::from_buffer_mask_alpha(
			Rect::new(0, 0, 1, 2),
			Channel::Lumaa,
			vec![1, 255, 2, 255],
		);
		assert_eq!(format!("{:?}", a), "Stencil ( ⠃ )");
		let b = Stencil::from_buffer_mask_alpha(
			Rect::new(2, 0, 1, 2),
			Channel::Lumaa,
			vec![3, 255, 4, 255],
		);
		assert_eq!(format!("{:?}", b), "Stencil ( ⠃ )");
		let c = Stencil::merge(&a, &b, Blending::Normal, Compositing::Lighter);
		assert_eq!(format!("{:?}", c), "Stencil ( ⠃⠃ )");
		assert_eq!(c.data, vec![1, 255, 3, 255, 2, 255, 4, 255]);

		let a = Stencil::from_buffer(
			Rect::new(0, 0, 4, 4),
			Channel::Lumaa,
			vec![
				1, 255, 2, 255, 3, 255, 4, 255, 5, 255, 6, 255, 7, 255, 8, 255, 9, 255, 10, 255,
				11, 255, 12, 255, 13, 255, 14, 255, 15, 255, 16, 255,
			],
		);
		assert_eq!(format!("{:?}", a), "Stencil ( ⣿⣿ )");
		let b = Stencil::from_buffer(
			Rect::new(1, 1, 2, 2),
			Channel::Lumaa,
			vec![1, 255, 1, 255, 1, 255, 1, 255],
		);
		assert_eq!(format!("{:?}", b), "Stencil ( ⠛ )");
		let c = Stencil::merge(&a, &b, Blending::Normal, Compositing::SourceOut);
		assert_eq!(format!("{:?}", c), "Stencil ( ⣏⣹ )");
		assert_eq!(
			c.data,
			vec![
				1, 255, 2, 255, 3, 255, 4, 255, 5, 255, 8, 255, 9, 255, 12, 255, 13, 255, 14, 255,
				15, 255, 16, 255
			]
		);
	}

	#[test]
	fn iter() {
		let a = Stencil::from_buffer(
			Rect::new(0, 0, 2, 2),
			Channel::Lumaa,
			vec![1, 255, 2, 255, 3, 255, 4, 255],
		);
		let pixels: Vec<_> = a
			.iter()
			.map(|(_, _, data)| data.to_vec())
			.flatten()
			.collect();
		assert_eq!(pixels, vec![1, 255, 2, 255, 3, 255, 4, 255]);

		let a = Stencil::from_buffer_mask_alpha(
			Rect::new(0, 0, 2, 2),
			Channel::Lumaa,
			vec![1, 255, 0, 0, 0, 0, 4, 255],
		);
		let pixels: Vec<_> = a
			.iter()
			.map(|(_, _, data)| data.to_vec())
			.flatten()
			.collect();
		assert_eq!(pixels, vec![1, 255, 4, 255]);
	}

	#[test]
	fn iter_mut() {
		let mut a = Stencil::from_buffer(
			Rect::new(0, 0, 2, 2),
			Channel::Lumaa,
			vec![1, 255, 2, 255, 3, 255, 4, 255],
		);
		let pixels: Vec<_> = a
			.iter_mut()
			.map(|(_, _, data)| data.to_vec())
			.flatten()
			.collect();
		assert_eq!(pixels, vec![1, 255, 2, 255, 3, 255, 4, 255]);

		let mut a = Stencil::from_buffer_mask_alpha(
			Rect::new(0, 0, 2, 2),
			Channel::Lumaa,
			vec![1, 255, 0, 0, 0, 0, 4, 255],
		);
		let pixels: Vec<_> = a
			.iter_mut()
			.map(|(_, _, data)| data.to_vec())
			.flatten()
			.collect();
		assert_eq!(pixels, vec![1, 255, 4, 255]);
	}
}
